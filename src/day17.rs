use bitvec::prelude::*;

#[derive(Clone)]
struct Region {
    dims: (usize, usize, usize, usize),
    bits: BitVec,
}

impl Region {
    fn get(&self, x: usize, y: usize, z: usize, w: usize) -> Option<bool> {
        if x < self.dims.0 && y < self.dims.1 && z < self.dims.2 && w < self.dims.3 {
            Some(unsafe {
                *self
                    .bits
                    .get_unchecked(((w * self.dims.2 + z) * self.dims.1 + y) * self.dims.0 + x)
            })
        } else {
            None
        }
    }

    fn count_active(&self) -> usize {
        self.bits.count_ones()
    }

    fn from_str(s: &str) -> Self {
        let mut bits = BitVec::new();
        let mut ny = 0;
        for line in s.lines() {
            for c in line.chars() {
                bits.push(match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("unknown input"),
                });
            }
            ny += 1;
        }
        let nx = bits.len() / ny;
        assert_eq!(nx * ny, bits.len());
        Self {
            dims: (nx, ny, 1, 1),
            bits,
        }
    }

    fn next_3d(&self) -> Self {
        let (nx, ny, nz) = (self.dims.0 + 2, self.dims.1 + 2, self.dims.2 + 2);
        let mut bits = BitVec::new();
        for z in 0..nz {
            for y in 0..ny {
                for x in 0..nx {
                    let mut centre_active = false;
                    let mut ring_active_count = 0;
                    for oz in 0..=2 {
                        for oy in 0..=2 {
                            for ox in 0..=2 {
                                let active = self
                                    .get(
                                        (x + ox).wrapping_sub(2),
                                        (y + oy).wrapping_sub(2),
                                        (z + oz).wrapping_sub(2),
                                        0,
                                    )
                                    .unwrap_or(false);
                                if ox == 1 && oy == 1 && oz == 1 {
                                    centre_active = active;
                                } else if active {
                                    ring_active_count += 1;
                                }
                            }
                        }
                    }
                    let active = if centre_active {
                        ring_active_count == 2 || ring_active_count == 3
                    } else {
                        ring_active_count == 3
                    };
                    bits.push(active);
                }
            }
        }
        Self {
            dims: (nx, ny, nz, 1),
            bits,
        }
    }

    fn next_4d(&self) -> Self {
        let (nx, ny, nz, nw) = (
            self.dims.0 + 2,
            self.dims.1 + 2,
            self.dims.2 + 2,
            self.dims.3 + 2,
        );
        let mut bits = BitVec::new();
        for w in 0..nw {
            for z in 0..nz {
                for y in 0..ny {
                    for x in 0..nx {
                        let mut centre_active = false;
                        let mut ring_active_count = 0;
                        for ow in 0..=2 {
                            for oz in 0..=2 {
                                for oy in 0..=2 {
                                    for ox in 0..=2 {
                                        let active = self
                                            .get(
                                                (x + ox).wrapping_sub(2),
                                                (y + oy).wrapping_sub(2),
                                                (z + oz).wrapping_sub(2),
                                                (w + ow).wrapping_sub(2),
                                            )
                                            .unwrap_or(false);
                                        if ox == 1 && oy == 1 && oz == 1 && ow == 1 {
                                            centre_active = active;
                                        } else if active {
                                            ring_active_count += 1;
                                        }
                                    }
                                }
                            }
                        }
                        let active = if centre_active {
                            ring_active_count == 2 || ring_active_count == 3
                        } else {
                            ring_active_count == 3
                        };
                        bits.push(active);
                    }
                }
            }
        }
        Self {
            dims: (nx, ny, nz, nw),
            bits,
        }
    }
}

pub fn run() {
    let text = std::fs::read_to_string("input/day17.txt").unwrap();
    //let text = ".#.\n..#\n###\n";
    let region = Region::from_str(&text);

    let mut tmp = region.clone();
    for _ in 0..6 {
        tmp = tmp.next_3d();
    }
    println!(
        "day17: active count after 6 3D cycles is {}",
        tmp.count_active()
    );

    let mut tmp = region.clone();
    for _ in 0..6 {
        tmp = tmp.next_4d();
    }
    println!(
        "day17: active count after 6 4D cycles is {}",
        tmp.count_active()
    );
}
