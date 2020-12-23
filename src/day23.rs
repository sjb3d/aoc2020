use std::convert::TryFrom;

struct Game {
    links: Vec<(u32, u32)>,
    cursor: u32,
}

impl Game {
    fn links(&self, cup: u32) -> &(u32, u32) {
        self.links.get((cup - 1) as usize).unwrap()
    }

    fn links_mut(&mut self, cup: u32) -> &mut (u32, u32) {
        self.links.get_mut((cup - 1) as usize).unwrap()
    }

    fn from_str(s: &str) -> Self {
        let cups: Vec<_> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut game = Game {
            links: vec![(0, 0); cups.len()],
            cursor: cups.first().cloned().unwrap(),
        };
        let len = cups.len();
        for i in 0..len {
            *game.links_mut(cups[i]) = (cups[(i + len - 1) % len], cups[(i + 1) % len]);
        }
        game
    }

    fn move_segment(&mut self, first: u32, last: u32, dest: u32) {
        let old_before = self.links(first).0;
        let old_after = self.links(last).1;
        self.links_mut(old_before).1 = old_after;
        self.links_mut(old_after).0 = old_before;

        let new_before = dest;
        let new_after = self.links(dest).1;
        self.links_mut(new_before).1 = first;
        self.links_mut(first).0 = new_before;
        self.links_mut(last).1 = new_after;
        self.links_mut(new_after).0 = last;
    }

    fn turn(&mut self) {
        let len = self.links.len() as u32;

        let a = self.links(self.cursor).1;
        let b = self.links(a).1;
        let c = self.links(b).1;

        let mut dest = self.cursor - 1;
        loop {
            if dest == 0 {
                dest = len;
            }
            if dest != a && dest != b && dest != c {
                break;
            }
            dest -= 1;
        }

        self.move_segment(a, c, dest);
        self.cursor = self.links(self.cursor).1;
    }

    fn to_string(&self) -> String {
        let mut cup = self.links(1).1;
        let mut tmp = String::new();
        while cup != 1 {
            tmp.push(char::try_from(cup + ('0' as u32)).unwrap());
            cup = self.links(cup).1;
        }
        tmp
    }

    fn extend_to(&mut self, n: u32) {
        let first = (self.links.len() as u32) + 1;
        let last = n;
        for i in first..=last {
            self.links.push(((i - 1), (i + 1)));
        }

        let before = self.links(self.cursor).0;
        let after = self.cursor;
        self.links_mut(before).1 = first;
        self.links_mut(first).0 = before;
        self.links_mut(last).1 = after;
        self.links_mut(after).0 = last;
    }

    fn after_one(&self) -> (u32, u32) {
        let a = self.links(1).1;
        let b = self.links(a).1;
        (a, b)
    }
}

pub fn run() {
    let input = "219748365";

    let mut game = Game::from_str(input);
    for _ in 0..100 {
        game.turn();
    }
    println!("day23: string after 100 moves is {}", game.to_string());

    let mut game = Game::from_str(input);
    game.extend_to(1_000_000);
    for _ in 0..10_000_000 {
        game.turn();
    }
    let (a, b) = game.after_one();
    println!(
        "day23: product after many moves is {:?}",
        (a as usize) * (b as usize)
    );
}
