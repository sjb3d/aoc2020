use std::collections::VecDeque;
use std::convert::TryFrom;

struct Game {
    cups: VecDeque<u32>,
}

impl Game {
    fn from_str(s: &str) -> Game {
        Self {
            cups: s.chars().map(|c| c.to_digit(10).unwrap()).collect(),
        }
    }

    fn turn(&mut self) {
        let len = self.cups.len();

        self.cups.rotate_left(1);
        let tmp = [
            self.cups.pop_front().unwrap(),
            self.cups.pop_front().unwrap(),
            self.cups.pop_front().unwrap(),
        ];

        let dest = {
            let mut target = self.cups.back().unwrap() - 1;
            loop {
                if target == 0 {
                    target = len as u32;
                }
                if let Some((i, _)) = self.cups.iter().enumerate().find(|(_, &v)| v == target) {
                    break i;
                }
                target -= 1;
            }
        };
        self.cups.rotate_left(dest + 1);
        for &c in tmp.iter().rev() {
            self.cups.push_front(c);
        }
        self.cups.rotate_right(dest + 1);
    }

    fn to_string(&self) -> String {
        let mut tmp = self.cups.clone();
        tmp.rotate_left(tmp.iter().enumerate().find(|(_, &v)| v == 1).unwrap().0);
        tmp.iter()
            .skip(1)
            .map(|&n| char::try_from(n + ('0' as u32)).unwrap())
            .collect()
    }
}

pub fn run() {
    let mut game = Game::from_str("219748365");
    for _ in 0..100 {
        game.turn();
    }
    println!("day23: string after 100 moves is {}", game.to_string());
}
