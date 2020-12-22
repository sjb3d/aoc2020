use std::collections::VecDeque;

#[derive(Clone)]
struct Player {
    deck: VecDeque<usize>,
}

impl Player {
    fn is_out(&self) -> bool {
        self.deck.is_empty()
    }

    fn score(&self) -> usize {
        self.deck.iter().rev().enumerate().map(|(i, v)| (i + 1) * v).sum()
    }
}

fn play(p1: &mut Player, p2: &mut Player) {
    let c1 = p1.deck.pop_front().unwrap();
    let c2 = p2.deck.pop_front().unwrap();
    if c1 > c2 {
        p1.deck.push_back(c1);
        p1.deck.push_back(c2);
    } else {
        p2.deck.push_back(c2);
        p2.deck.push_back(c1);
    }
}

pub fn run() {
    let text = std::fs::read_to_string("input/day22.txt").unwrap();
    let mut lines = text.lines();
    assert_eq!(lines.next(), Some("Player 1:"));
    let player1 = Player {
        deck: (0..25)
            .map(|_| lines.next().unwrap().parse::<usize>().unwrap())
            .collect(),
    };
    lines.next();
    assert_eq!(lines.next(), Some("Player 2:"));
    let player2 = Player {
        deck: (0..25)
            .map(|_| lines.next().unwrap().parse::<usize>().unwrap())
            .collect(),
    };

    let score = {
        let mut p1 = player1.clone();
        let mut p2 = player2.clone();
        loop {
            play(&mut p1, &mut p2);
            if p1.is_out() {
                break p2.score();
            }
            if p2.is_out() {
                break p1.score();
            }
        }
    };
    println!("day22: winning score is {}", score);
}
