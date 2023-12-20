use std::collections::{VecDeque,HashMap};
use std::any::Any;
use std::cell::RefCell;
use std::mem::{take,replace};

#[derive(Debug, Clone, PartialEq)]
enum Pulse {
    Low,
    High
}

#[derive(Debug, Clone, PartialEq)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
    Registry
}

trait Module {
    fn name(&self) -> &str;
    fn receive(&mut self, from: &str, p: Pulse, r: &mut ModuleRegistry);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_type(&self) -> ModuleType;
    fn get_base(&self) -> &BaseModule;
    fn get_base_mut(&mut self) -> &mut BaseModule;
}

#[derive(Debug)]
struct BaseModule {
    name: String,
    dest_modules: Vec<String>,
}

#[derive(Debug)]
struct FlipFlipModule {
    base: BaseModule,
    powered: bool
}

#[derive(Debug)]
struct ConjunctionModule {
    base: BaseModule,
    last_received: HashMap<String, Pulse>
}

#[derive(Debug)]
struct BroadcasterModule {
    base: BaseModule,
}

impl Module for FlipFlipModule {
    fn receive(&mut self, from: &str, p: Pulse, r: &mut ModuleRegistry) {
        if p == Pulse::High {
            return;
        }
        self.powered = !self.powered;
        if self.powered {
            r.queue_message(self.name(), Pulse::High, &self.base.dest_modules);
        } else {
            r.queue_message(self.name(), Pulse::Low, &self.base.dest_modules);
        }
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type(&self) -> ModuleType {
        ModuleType::FlipFlop
    }

    fn get_base(&self) -> &BaseModule {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseModule {
        &mut self.base
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self 
    }
}

impl Module for ConjunctionModule {
    fn receive(&mut self, from: &str, p: Pulse, r: &mut ModuleRegistry) {
        *self.last_received.get_mut(from).unwrap() = p;
        if self.last_received.iter().all(|(_,last_received)| *last_received == Pulse::High) {
            r.queue_message(self.name(), Pulse::Low, &self.base.dest_modules);
        } else {
            r.queue_message(self.name(), Pulse::High, &self.base.dest_modules);
        }
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type(&self) -> ModuleType {
        ModuleType::Conjunction
    }

    fn get_base(&self) -> &BaseModule {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseModule {
        &mut self.base
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Module for BroadcasterModule {
    fn receive(&mut self, from: &str, p: Pulse, r: &mut ModuleRegistry) {
        r.queue_message(self.name(), p, &self.base.dest_modules);
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type(&self) -> ModuleType {
        ModuleType::Broadcaster
    }

    fn get_base(&self) -> &BaseModule {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseModule {
        &mut self.base
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct ModuleRegistry {
    base: BaseModule,
    pub modules: HashMap<String, RefCell<Box<dyn Module>>>,
    // (from, to, what pulse)
    msg_queue: VecDeque<(String, String, Pulse)>
}

impl ModuleRegistry {
    fn new() -> Self {
        ModuleRegistry {
            base: BaseModule {
                name: "Registry".to_string(),
                dest_modules: vec![ "broadcaster".to_string() ]
            },
            modules: HashMap::new(),
            msg_queue: VecDeque::new()
        }
    }

    fn add_module_from_str(&mut self, s: &str) {
        let mut splits = s.split(" -> ");
        let module_name = splits.next().unwrap();
        let dest_modules = splits.next().unwrap();

        let mod_name_formatted = match module_name.chars().nth(0).unwrap() {
                '%' | '&' => module_name[1..].to_owned(),
                _ => module_name.to_owned()
            };

        let base_mod = BaseModule {
            name: mod_name_formatted.clone(),
            dest_modules: dest_modules.split(",").map(|m| m.trim().to_owned())
            .collect::<Vec<_>>()
        };
        
        let module: Box<dyn Module> = match module_name {
            name if &name[..1] == "%" => {
                println!("Making FlipFlipModule, {:?}", base_mod);
                Box::new(FlipFlipModule {
                    base: base_mod,
                    powered: false
                })
            },
            name if &name[..1] == "&" => {
                println!("Making ConjunctionModule, {:?}", base_mod);
                Box::new(ConjunctionModule {
                    base: base_mod,
                    last_received: HashMap::new()
                })
            },
            "broadcaster" => { 
                println!("Making BroadcasterModule, {:?}", base_mod);
                Box::new(BroadcasterModule {
                    base: base_mod
                })
            },
            _ => panic!("Invalid module name") 
        };

        self.modules.insert(mod_name_formatted, RefCell::new(module));
    }

    // Wire up conjunction modules with their senders
    fn initialize(&mut self) {
        let mut add_to_what = HashMap::new();
        for (mod_name, module) in &self.modules {
            for rec_name in &module.borrow().get_base().dest_modules {
                if (self.modules.get(rec_name).unwrap().borrow().get_type() == ModuleType::Conjunction) {
                    add_to_what.entry(rec_name.clone())
                        .and_modify(|v: &mut Vec<String>| v.push(mod_name.clone()))
                        .or_insert(vec![ mod_name.clone() ]);
                }
            }
        }
        for (rec_name, senders) in &add_to_what {
            let cell = &mut self.modules
                .get_mut(rec_name)
                .unwrap()
                .borrow_mut();
            let conj_module = cell
                .as_any_mut()
                .downcast_mut::<ConjunctionModule>()
                .unwrap();
            for sender in senders {
                conj_module.last_received
                    .insert(sender.to_string(), Pulse::Low);
            }
        }

        /*
        for (rec_name, senders) in &add_to_what {
            let conj_module = &self.modules
                .get(rec_name)
                .unwrap()
                .as_any()
                .downcast_ref::<ConjunctionModule>()
                .unwrap();
            for sender in &conj_module.last_received {
                println!("{:?}", sender);
            }
        }
        */
    }


    fn queue_message(&mut self, from: &str, p: Pulse, rec: &[String]) {
        for r in rec {
            //println!("QUEUED: {} -> {}, {:?}", from, r, p);
            self.msg_queue.push_back((from.to_owned(), r.to_owned(), p.clone()));
        }
    }

    fn send_messages(&mut self) {
        let modules = take(&mut self.modules);
        while (!self.msg_queue.is_empty()) {
            let queue = take(&mut self.msg_queue);
            for msg in &queue {
                println!("SENDING: {}->{}, {:?}", msg.0, msg.1, msg.2);
                let mut receiver = modules.get(&msg.1).unwrap().borrow_mut();
                receiver.receive(
                    &msg.0,
                    msg.2.clone(),
                    self
                );
            }
        }
        let _ = replace(&mut self.modules, modules);
    }

    fn push_button(&mut self) {
        let name = self.name().to_owned();
        self.queue_message(
            &name, 
            Pulse::Low, 
            &[ "broadcaster".to_string() ]
        );
        self.send_messages();
    }
}

impl Module for ModuleRegistry {
    fn receive(&mut self, from: &str, p: Pulse, reg: &mut ModuleRegistry) {
        unreachable!()
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type(&self) -> ModuleType {
        ModuleType::Registry
    }

    fn get_base(&self) -> &BaseModule {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseModule {
        &mut self.base
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


fn main() {
    let mut reg = ModuleRegistry::new();

    let contents = include_str!("../../input2.txt");
    for line in contents.lines() {
        reg.add_module_from_str(line);
    }
    reg.initialize();

    for module in &reg.modules {
        println!("{}", module.0);
    }

    reg.push_button();
}
