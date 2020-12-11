#[derive(Copy, Clone)]
enum Place {
    Floor,
    Empty,
    Occupied,
}

fn iterate1(places: &[Place], nx: usize, ny: usize) -> Vec<Place> {
    let mut old_places = places.to_vec();
    let mut new_places = Vec::new();
    loop {
        new_places.clear();
        let mut is_changed = false;
        for y in 0..ny {
            for x in 0..nx {
                let mut occupied_count = 0;
                for yy in (y.max(1) - 1)..(y.min(ny - 2) + 2) {
                    for xx in (x.max(1) - 1)..(x.min(nx - 2) + 2) {
                        if x != xx || y != yy {
                            if matches!(old_places[yy * nx + xx], Place::Occupied) {
                                occupied_count += 1;
                            }
                        }
                    }
                }
                let old_place = old_places[y * nx + x];
                let new_place = match old_place {
                    Place::Empty => {
                        if occupied_count == 0 {
                            is_changed = true;
                            Place::Occupied
                        } else {
                            Place::Empty
                        }
                    }
                    Place::Occupied => {
                        if occupied_count >= 4 {
                            is_changed = true;
                            Place::Empty
                        } else {
                            Place::Occupied
                        }
                    }
                    Place::Floor => Place::Floor,
                };
                new_places.push(new_place);
            }
        }
        if !is_changed {
            return old_places;
        }
        std::mem::swap(&mut new_places, &mut old_places);
    }
}

fn iterate2(places: &[Place], nx: usize, ny: usize) -> Vec<Place> {
    let mut old_places = places.to_vec();
    let mut new_places = Vec::new();
    loop {
        new_places.clear();
        let mut is_changed = false;
        for y in 0..ny {
            for x in 0..nx {
                let mut occupied_count = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let (mut px, mut py) = (x as isize, y as isize);
                        let is_occupied = loop {
                            px += dx;
                            py += dy;
                            if px < 0 || (nx as isize) <= px || py < 0 || (ny as isize) <= py {
                                break false;
                            }
                            match old_places[(py as usize) * nx + (px as usize)] {
                                Place::Floor => {}
                                Place::Empty => {
                                    break false;
                                }
                                Place::Occupied => {
                                    break true;
                                }
                            }
                        };
                        if is_occupied {
                            occupied_count += 1;
                        }
                    }
                }
                let old_place = old_places[y * nx + x];
                let new_place = match old_place {
                    Place::Empty => {
                        if occupied_count == 0 {
                            is_changed = true;
                            Place::Occupied
                        } else {
                            Place::Empty
                        }
                    }
                    Place::Occupied => {
                        if occupied_count >= 5 {
                            is_changed = true;
                            Place::Empty
                        } else {
                            Place::Occupied
                        }
                    }
                    Place::Floor => Place::Floor,
                };
                new_places.push(new_place);
            }
        }
        if !is_changed {
            return old_places;
        }
        std::mem::swap(&mut new_places, &mut old_places);
    }
}

pub fn run() {
    let text = std::fs::read_to_string("input/day11.txt").unwrap();
    let mut ny = 0;
    let mut nx = 0;
    let places: Vec<_> = text
        .lines()
        .flat_map(|s| {
            if nx == 0 {
                nx = s.len();
            } else {
                assert_eq!(nx, s.len());
            }
            ny += 1;
            s.chars().map(|c| match c {
                '.' => Place::Floor,
                'L' => Place::Empty,
                _ => panic!("unknown character"),
            })
        })
        .collect();

    let places1 = iterate1(&places, nx, ny);
    println!(
        "day11: occupied count is {}",
        places1
            .iter()
            .filter(|p| matches!(p, Place::Occupied))
            .count()
    );

    let places2 = iterate2(&places, nx, ny);
    println!(
        "day11: alternate occupied count is {}",
        places2
            .iter()
            .filter(|p| matches!(p, Place::Occupied))
            .count()
    );
}
