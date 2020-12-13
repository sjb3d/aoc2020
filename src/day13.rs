fn wait_time(ts: usize, id: usize) -> usize {
    let r = ts % id;
    if r == 0 {
        0
    } else {
        id - r
    }
}

pub fn run() {
    let text = std::fs::read_to_string("input/day13.txt").unwrap();

    let mut lines = text.lines();
    let start_ts = lines.next().unwrap().parse::<usize>().unwrap();
    let ids: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|&s| s != "x")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut depart_wait = usize::MAX;
    let mut depart_id = 0;
    for &id in ids.iter() {
        let wait = wait_time(start_ts, id);
        if wait < depart_wait {
            depart_wait = wait;
            depart_id = id;
        }
    }
    println!(
        "day13: product is {} * {} = {}",
        depart_wait,
        depart_id,
        depart_wait * depart_id
    );
}
