struct Probe {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Probe {
    fn new(vx: i32, vy: i32) -> Self {
        Self { x: 0, y: 0, vx, vy }
    }

    fn tick(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.vy -= 1;

        if self.vx > 0 {
            self.vx -= 1;
        }

        if self.vx < 0 {
            self.vx += 1;
        }
    }
}

const TARGET_X1: i32 = 88;
const TARGET_X2: i32 = 125;
const TARGET_Y1: i32 = -103;
const TARGET_Y2: i32 = -157;

fn main() {
    let mut counter = 0;

    for vx in 0..=150 {
        for vy in -200..=4000 {
            println!("Testing: {}, {}", vx, vy);
            let mut probe = Probe::new(vx, vy);
            loop {
                probe.tick();

                if probe.y <= TARGET_Y1
                    && probe.y >= TARGET_Y2
                    && probe.x >= TARGET_X1
                    && probe.x <= TARGET_X2
                {
                    counter += 1;
                    break;
                }

                if probe.y < TARGET_Y2 {
                    break;
                }
            }
        }
    }

    println!("Count: {}", counter);
}
