use std::fs;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct LanternFish {
    counter: u32,
}

impl LanternFish {
    fn new(counter: u32) -> Self {
        Self { counter }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Sea {
    fish: Vec<LanternFish>,
}

impl Sea {
    fn new() -> Self {
        Self { fish: Vec::new() }
    }

    fn tick(&mut self) {
        let mut new_gen: Vec<LanternFish> = Vec::new();
        for mut fish in self.fish.iter_mut() {
            if fish.counter == 0 {
                fish.counter = 7;
                new_gen.push(LanternFish::new(8));
            }
            fish.counter -= 1;
            new_gen.push(*fish);
        }
        self.fish = new_gen;
    }

    fn add_fish(&mut self, fish: LanternFish) {
        self.fish.push(fish);
    }
}

fn main() {
    let input: Vec<u32> = fs::read_to_string("inputs/day_06.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|fish_counter| fish_counter.parse::<u32>().unwrap())
        .collect();

    let mut sea = Sea::new();

    for fish_counter in input {
        sea.add_fish(LanternFish::new(fish_counter));
    }

    for _i in 0..80 {
        sea.tick();
    }

    println!("{:#?}", sea.fish.len());
}
