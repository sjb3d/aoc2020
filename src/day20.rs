use bitvec::prelude::*;
use std::collections::HashMap;

struct Pixels {
    bits: BitVec,
}

struct Tile {
    index: usize,
    pixels: Pixels,
    edges_cw: [EdgePixels; 4],
    edges_ccw: [EdgePixels; 4],
}

enum Winding {
    CW,
    CCW,
}

struct Transform {
    winding: Winding,
    rotation: u8,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct EdgePixels(u16);

impl EdgePixels {
    fn reverse(&self) -> EdgePixels {
        EdgePixels(self.0.reverse_bits() >> 6)
    }
}

impl Pixels {
    fn get(&self, x: usize, y: usize) -> bool {
        *self.bits.get(y * 10 + x).unwrap()
    }
}

impl Tile {
    fn new(index: usize, pixels: Pixels) -> Self {
        let fold_bits = |acc: u16, b: bool| (acc << 1) | if b { 1 } else { 0 };
        let edges_cw = [
            EdgePixels((0..10).map(|i| pixels.get(i, 0)).fold(0, fold_bits)), // top (left to right)
            EdgePixels((0..10).map(|i| pixels.get(9, i)).fold(0, fold_bits)), // right (top to bottom)
            EdgePixels((0..10).map(|i| pixels.get(9 - i, 9)).fold(0, fold_bits)), // bottom (right to left)
            EdgePixels((0..10).map(|i| pixels.get(0, 9 - i)).fold(0, fold_bits)), // left (bottom to top)
        ];
        let edges_ccw = [
            edges_cw[0].reverse(),
            edges_cw[3].reverse(),
            edges_cw[1].reverse(),
            edges_cw[2].reverse(),
        ];
        Self {
            index,
            pixels,
            edges_cw,
            edges_ccw,
        }
    }
}

pub fn run() {
    let text = std::fs::read_to_string("input/day20.txt").unwrap();

    let mut lines = text.lines();
    let mut tiles = Vec::new();
    while let Some(line) = lines.next() {
        let index = line
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut bits = BitVec::new();
        for _ in 0..10 {
            let line = lines.next().unwrap();
            for c in line.chars() {
                bits.push(match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("uknown symbol"),
                })
            }
        }
        tiles.push(Tile::new(index, Pixels { bits }));
        lines.next();
    }

    let mut edges: HashMap<EdgePixels, Vec<(&Tile, Transform)>> = HashMap::new();
    for tile in tiles.iter() {
        for rot in 0..4 {
            edges.entry(tile.edges_cw[rot]).or_insert(Vec::new()).push((
                tile,
                Transform {
                    winding: Winding::CW,
                    rotation: rot as u8,
                },
            ));
            edges
                .entry(tile.edges_ccw[rot])
                .or_insert(Vec::new())
                .push((
                    tile,
                    Transform {
                        winding: Winding::CCW,
                        rotation: rot as u8,
                    },
                ));
        }
    }

    let mut corners = Vec::new();
    for tile in tiles.iter() {
        if tile
            .edges_cw
            .iter()
            .filter(|e| edges.get(e).unwrap().len() == 1)
            .count()
            == 2
            || tile
                .edges_ccw
                .iter()
                .filter(|e| edges.get(e).unwrap().len() == 1)
                .count()
                == 2
        {
            corners.push(tile);
        }
    }
    assert_eq!(corners.len(), 4);
    println!(
        "day20: corner index product is {}",
        corners.iter().map(|t| t.index).product::<usize>()
    );
}
