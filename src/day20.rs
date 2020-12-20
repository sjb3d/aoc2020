use bitvec::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
struct Pixels {
    width: usize,
    height: usize,
    bits: BitVec,
}

struct Tile {
    index: usize,
    pixels: Pixels,
    edges_cw: [EdgePixels; 4],
    edges_ccw: [EdgePixels; 4],
}

#[derive(Clone, Copy)]
enum Winding {
    CW,
    CCW,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct EdgePixels(u16);

impl EdgePixels {
    fn reverse(&self) -> EdgePixels {
        EdgePixels(self.0.reverse_bits() >> 6)
    }
}

impl Pixels {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            bits: bitvec![0; width*height],
        }
    }

    fn set(&mut self, x: usize, y: usize, b: bool) {
        self.bits.set(y * self.width + x, b);
    }

    fn get(&self, x: usize, y: usize) -> bool {
        *self.bits.get(y * self.width + x).unwrap()
    }

    fn flip_horizontal(&self) -> Self {
        let mut bits = BitVec::new();
        for y in 0..self.height {
            bits.extend(self.bits[y * self.width..(y + 1) * self.width].iter().rev());
        }
        Self {
            width: self.width,
            height: self.height,
            bits,
        }
    }

    fn rotate1(&self) -> Self {
        let width = self.height;
        let height = self.width;
        let mut bits = BitVec::new();
        for y in 0..height {
            for x in 0..width {
                bits.push(self.get(y, width - 1 - x));
            }
        }
        Self {
            width,
            height,
            bits,
        }
    }

    fn transform(&self, winding: Winding, rotation: u8) -> Self {
        let tmp = match winding {
            Winding::CW => self.clone(),
            Winding::CCW => self.flip_horizontal(),
        };
        match rotation {
            0 => tmp,
            1 => tmp.rotate1(),
            2 => tmp.rotate1().rotate1(),
            3 => tmp.rotate1().rotate1().rotate1(),
            _ => panic!("bad rotation"),
        }
    }

    fn copy(
        &mut self,
        dest_x: usize,
        dest_y: usize,
        pixels: &Pixels,
        src_x: usize,
        src_y: usize,
        width: usize,
        height: usize,
    ) {
        for y in 0..height {
            for x in 0..width {
                self.set(dest_x + x, dest_y + y, pixels.get(src_x + x, src_y + y));
            }
        }
    }

    fn intersects(&self, dest_x: usize, dest_y: usize, pixels: &Pixels) -> bool {
        for y in 0..pixels.height {
            for x in 0..pixels.width {
                if pixels.get(x, y) && !self.get(dest_x + x, dest_y + y) {
                    return false;
                }
            }
        }
        return true;
    }

    fn subtract(&mut self, dest_x: usize, dest_y: usize, pixels: &Pixels) {
        for y in 0..pixels.height {
            for x in 0..pixels.width {
                if pixels.get(x, y) {
                    self.set(dest_x + x, dest_y + y, false);
                }
            }
        }
    }

    fn subtract_intersections(&self, pixels: &Pixels) -> Option<Self> {
        let mut result = None;
        for dest_y in 0..(self.height - pixels.height) {
            for dest_x in 0..(self.width - pixels.width) {
                if self.intersects(dest_x, dest_y, pixels) {
                    result
                        .get_or_insert_with(|| self.clone())
                        .subtract(dest_x, dest_y, pixels);
                }
            }
        }
        result
    }
}

impl Tile {
    fn new(index: usize, pixels: Pixels) -> Self {
        assert_eq!(pixels.width, 10);
        assert_eq!(pixels.height, 10);
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
            edges_cw[2].reverse(),
            edges_cw[1].reverse(),
        ];
        Self {
            index,
            pixels,
            edges_cw,
            edges_ccw,
        }
    }

    fn transform(&self, winding: Winding, rotation: u8) -> Self {
        Self::new(self.index, self.pixels.transform(winding, rotation))
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
        tiles.push(Tile::new(
            index,
            Pixels {
                width: 10,
                height: 10,
                bits,
            },
        ));
        lines.next();
    }

    let mut edges: HashMap<EdgePixels, Vec<(&Tile, Winding, u8)>> = HashMap::new();
    for tile in tiles.iter() {
        for rot in 0..4 {
            edges.entry(tile.edges_cw[rot]).or_insert(Vec::new()).push((
                tile,
                Winding::CW,
                rot as u8,
            ));
            edges
                .entry(tile.edges_ccw[rot])
                .or_insert(Vec::new())
                .push((tile, Winding::CCW, rot as u8));
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
        {
            corners.push(tile);
        }
    }
    assert_eq!(corners.len(), 4);
    println!(
        "day20: corner index product is {}",
        corners.iter().map(|t| t.index).product::<usize>()
    );

    let mut grid: Vec<Tile> = Vec::new();
    let mut image = Pixels::new(96, 96);
    for ty in 0..12 {
        for tx in 0..12 {
            let tile = if ty == 0 {
                if tx == 0 {
                    let tile = corners.first().unwrap();
                    let counts: Vec<_> = tile
                        .edges_cw
                        .iter()
                        .map(|e| edges.get(e).unwrap().len())
                        .collect();
                    tile.transform(
                        Winding::CW,
                        match (counts[0], counts[1], counts[2], counts[3]) {
                            (1, 1, _, _) => 3,
                            (_, 1, 1, _) => 2,
                            (_, _, 1, 1) => 1,
                            (1, _, _, 1) => 0,
                            _ => panic!("bad corner"),
                        },
                    )
                } else {
                    let left_tile = grid.get(ty * 12 + tx - 1).unwrap();
                    let left_edge = left_tile.edges_cw[1].reverse();
                    let (tile, winding, rotation) = edges
                        .get(&left_edge)
                        .unwrap()
                        .iter()
                        .filter(|(t, _, _)| t.index != left_tile.index)
                        .next()
                        .unwrap();
                    let rotation = 3 - rotation;
                    let tile = tile.transform(*winding, rotation);
                    assert_eq!(tile.edges_cw[3], left_edge);
                    tile
                }
            } else {
                let top_tile = grid.get((ty - 1) * 12 + tx).unwrap();
                let top_edge = top_tile.edges_cw[2].reverse();
                let (tile, winding, rotation) = edges
                    .get(&top_edge)
                    .unwrap()
                    .iter()
                    .filter(|(t, _, _)| t.index != top_tile.index)
                    .next()
                    .unwrap();
                let rotation = (4 - rotation) % 4;
                let tile = tile.transform(*winding, rotation);
                assert_eq!(tile.edges_cw[0], top_edge);
                if tx != 0 {
                    assert_eq!(
                        tile.edges_cw[3],
                        grid.get(ty * 12 + tx - 1).unwrap().edges_cw[1].reverse()
                    );
                }
                tile
            };
            image.copy(8 * tx, 8 * ty, &tile.pixels, 1, 1, 8, 8);
            grid.push(tile);
        }
    }

    let monster_text = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   \n";
    let monster = Pixels {
        width: 20,
        height: 3,
        bits: {
            let mut bits = BitVec::new();
            for line in monster_text.lines() {
                for c in line.chars() {
                    bits.push(match c {
                        '#' => true,
                        ' ' => false,
                        _ => panic!("unknown symbol"),
                    })
                }
            }
            assert_eq!(bits.len(), 60);
            bits
        },
    };

    for winding in [Winding::CW, Winding::CCW].iter() {
        for rotation in 0..4 {
            let needle = monster.transform(*winding, rotation);
            if let Some(result) = image.subtract_intersections(&needle) {
                println!(
                    "day20: pixel count remaining is {}",
                    result.bits.count_ones()
                );
            }
        }
    }
}
