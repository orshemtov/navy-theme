// Rust code

use rand;

const WATER: &str = "💧";

#[derive(Debug)]
enum Coin {
    Heads,
    Tails,
}

fn flip_coin() -> Coin {
    let mut rng = rand::thread_rng();
    if rng.gen::<bool>() {
        Coin::Heads
    } else {
        Coin::Tails
    }
}

fn main() {
    let magician = Magician::new();

    if let Coin::Heads = flip_coin() {
        magician.turn_water_into_whiskey(WATER);
    } else {
        magician.turn_water_into_wine(WATER);
    }
}

struct Magician {}

impl Magician {
    fn new() -> Self {
        Magician {}
    }

    fn turn_water_into_whiskey(&self, water: &str) {
        println!("{}", water);
        println!("Enjoy your {}", "🥃");
    }

    fn turn_water_into_wine(&self, water: &str) {
        println!("{}", water);
        println!("Enjoy your {}", "🍷");
    }
}
