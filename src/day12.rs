#[derive(Clone, Copy)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn left_90(&mut self) {
        *self = match self {
            Heading::North => Heading::West,
            Heading::East => Heading::North,
            Heading::South => Heading::East,
            Heading::West => Heading::South,
        }
    }

    fn right_90(&mut self) {
        *self = match self {
            Heading::North => Heading::East,
            Heading::East => Heading::South,
            Heading::South => Heading::West,
            Heading::West => Heading::North,
        }
    }

    fn turn_by(&mut self, turn: Turn, mut amount: u32) {
        while amount > 0 {
            match turn {
                Turn::Left => self.left_90(),
                Turn::Right => self.right_90(),
            };
            amount -= 90;
        }
    }
}

#[derive(Clone, Copy)]
enum Turn {
    Left,
    Right,
}

#[derive(Clone, Copy)]
enum Action {
    Move(Heading, u32),
    Turn(Turn, u32),
    Forward(u32),
}

#[derive(Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn move_by(&mut self, heading: Heading, amount: u32) {
        match heading {
            Heading::North => self.y += amount as i32,
            Heading::East => self.x += amount as i32,
            Heading::South => self.y -= amount as i32,
            Heading::West => self.x -= amount as i32,
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn rotate_by(&mut self, turn: Turn, mut amount: u32) {
        while amount > 0 {
            match turn {
                Turn::Left => {
                    *self = Coord {
                        x: -self.y,
                        y: self.x,
                    }
                }
                Turn::Right => {
                    *self = Coord {
                        x: self.y,
                        y: -self.x,
                    }
                }
            };
            amount -= 90;
        }
    }
}

pub fn run() {
    let text = std::fs::read_to_string("input/day12.txt").unwrap();
    let actions: Vec<_> = text
        .lines()
        .map(|s| {
            let amount = s[1..].parse::<u32>().unwrap();
            match s.chars().next().unwrap() {
                'N' => Action::Move(Heading::North, amount),
                'E' => Action::Move(Heading::East, amount),
                'S' => Action::Move(Heading::South, amount),
                'W' => Action::Move(Heading::West, amount),
                'L' => Action::Turn(Turn::Left, amount),
                'R' => Action::Turn(Turn::Right, amount),
                'F' => Action::Forward(amount),
                _ => panic!("unknown input"),
            }
        })
        .collect();
    {
        let mut coord = Coord { x: 0, y: 0 };
        let mut heading = Heading::East;
        for action in actions.iter() {
            match *action {
                Action::Move(heading, amount) => coord.move_by(heading, amount),
                Action::Turn(turn, amount) => heading.turn_by(turn, amount),
                Action::Forward(amount) => coord.move_by(heading, amount),
            }
        }
        println!(
            "day12: manhattan distance is {}",
            coord.manhattan_distance()
        );
    }

    {
        let mut waypoint_coord = Coord { x: 10, y: 1 };
        let mut ship_coord = Coord { x: 0, y: 0 };
        for action in actions.iter() {
            match *action {
                Action::Move(heading, amount) => waypoint_coord.move_by(heading, amount),
                Action::Turn(turn, amount) => waypoint_coord.rotate_by(turn, amount),
                Action::Forward(amount) => {
                    ship_coord.x += (amount as i32) * waypoint_coord.x;
                    ship_coord.y += (amount as i32) * waypoint_coord.y;
                }
            }
        }
        println!(
            "day12: alt manhattan distance is {}",
            ship_coord.manhattan_distance()
        );
    }
}
