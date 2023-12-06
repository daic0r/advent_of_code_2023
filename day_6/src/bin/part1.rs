#[derive(Debug)]
struct Game {
    duration: u32,
    record_dist: u32,
    times_button_to_win: Vec<u32>
}

impl Game {
    pub fn new(duration: u32, distance: u32) -> Self {
        let mut ret = Game {
            duration,
            record_dist: distance,
            times_button_to_win: vec![]
        };
        ret.calc_ways_to_win();
        ret
    }

    pub fn calc_ways_to_win(&mut self) {
        // i represents the time the button is held down
        for i in 0..=self.duration {
            let dist_traveled = (self.duration - i) * i;
            // If we're currently in the range where we break the record and
            // suddenly drop below that number, the rest will also be below
            // the record => we can abort here
            if dist_traveled <= self.duration && !self.times_button_to_win.is_empty() {
                println!("Aborted at i={}", i);
                break;
            }
            if dist_traveled > self.record_dist {
                self.times_button_to_win.push(i);
            }
        }
    }

}
fn main() {
    let contents = include_str!("../../input2.txt");

    let mut lines = contents.split('\n');
    let mut games = vec![];
    let mut durations = lines.next().unwrap().split_whitespace();
    let mut distances = lines.next().unwrap().split_whitespace();
    // Skip labels
    durations.next();
    distances.next();
    for (i,duration) in durations.enumerate() {
        games.push(Game::new(
            duration.parse().expect("Must be a number"),
            distances.next().unwrap().parse().expect("Must be a number")
        ));
    }

    println!("{:?}", games);
    let product = games.iter().fold(1, |acc,game| acc*game.times_button_to_win.len());
    println!("Product of ways to win: {}", product);
}
