#[derive(Debug)]
struct Game {
    duration: u64,
    record_dist: u64,
    ways_to_win: u64
}

impl Game {
    pub fn new(duration: u64, distance: u64) -> Self {
        let mut ret = Game {
            duration,
            record_dist: distance,
            ways_to_win: 0,
        };
        ret.calc_ways_to_win();
        ret
    }

    pub fn calc_ways_to_win(&mut self) {
        let d = self.duration as f64 / 2.0;
        let d_2 = d.powi(2);
        let root = (d_2 - self.record_dist as f64).sqrt();
        let from_dur_calc = (d - root) as u64;
        let to_dur_calc = (d + root) as u64;
        self.ways_to_win = to_dur_calc - from_dur_calc;
    }

}
fn main() {
    let contents = include_str!("../../input.txt");

    let mut lines = contents.split('\n');
    let mut games = vec![];
    let mut durations = lines.next().unwrap().split_whitespace();
    let mut distances = lines.next().unwrap().split_whitespace();
    // Skip labels
    durations.next();
    distances.next();
    let mut duration_str = String::new();
    let mut distance_str = String::new();
    for duration in durations {
        duration_str.push_str(duration);
        distance_str.push_str(distances.next().unwrap());
    }
    println!("{}", duration_str);
    println!("{}", distance_str);
    games.push(Game::new(
        duration_str.parse().expect("Must be a number"),
        distance_str.parse().expect("Must be a number")
    ));

    println!("{:?}", games);

    let product = games.iter().fold(1, |acc,game| acc*game.ways_to_win);
    println!("Product of ways to win: {}", product);
}
