pub(super) struct Coordinates {
    x: i32,
    depth: i32,
}

impl Coordinates {
    fn new(x: i32, depth: i32) -> Self {
        Coordinates {
            x, depth
        }
    }

    fn up(&mut self, dist: i32) {
        self.depth -= dist;
    }

    fn down(&mut self, dist: i32) {
        self.depth += dist;
    }

    fn forward(&mut self, dist: i32) {
        self.x += dist;
    }

    fn parse_move(&mut self, line: String) {
        let split = line.find(" ").unwrap_or(0);
        let direction = line[0..split].to_owned();
        let distance: i32 = line[split + 1..line.len()].parse().unwrap_or(0);
        match direction.as_str() {
            "up" => self.up(distance),
            "down" => self.down(distance),
            "forward" => self.forward(distance),
            &_ => todo!()
        }
    }
}

pub mod part1 {
    pub fn main() {
        let text = read_file();
        let mut position = crate::day2::Coordinates::new(0, 0);
        let splits = text.split("\n");
        for split in splits {
            position.parse_move(split.trim().to_owned());
        }
        println!("Final position: {:?}, {:?}", position.x, position.depth);
        println!("Multiplied result: {:?}", position.x * position.depth);
    }

    pub(super) fn read_file() -> String {
        std::fs::read_to_string("day2.txt")
            .unwrap_or("".to_owned())
    }
}