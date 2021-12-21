#[derive(Debug)]
struct Game {
    p1_position: u32,
    p1_score: u32,
    p2_position: u32,
    p2_score: u32,
    die_value: u32,
    die_rolls: u32,
}

impl Game {
    fn new(p1_position: u32, p2_position: u32) -> Self {
        Self {
            p1_position,
            p2_position,
            p1_score: 0,
            p2_score: 0,
            die_value: 1,
            die_rolls: 0,
        }
    }

    fn roll(&mut self) -> u32 {
        let r = self.die_value;
        self.die_value += 1;
        self.die_rolls += 1;
        r
    }

    fn move_player(&mut self, player1: bool, amount: u32) -> u32 {
        if player1 {
            self.p1_position = 1 + (self.p1_position + amount - 1) % 10;
            self.p1_position
        } else {
            self.p2_position = 1 + (self.p2_position + amount - 1) % 10;
            self.p2_position
        }
    }

    fn turn(&mut self, player1: bool) {
        let roll1 = self.roll();
        let roll2 = self.roll();
        let roll3 = self.roll();
        let amount = roll1 + roll2 + roll3;

        let score_increase = self.move_player(player1, amount);

        if player1 {
            self.p1_score += score_increase;
        } else {
            self.p2_score += score_increase;
        }
    }
}

fn main() {
    let mut game = Game::new(5, 9);

    loop {
        game.turn(true);
        if game.p1_score >= 1000 {
            break;
        }
        game.turn(false);
        if game.p2_score >= 1000 {
            break;
        }
    }

    println!("{:?}", game);
}
