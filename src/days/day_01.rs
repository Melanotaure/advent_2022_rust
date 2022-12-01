pub fn run_part(_input: &String, _part: u8) {
    let mut max: u32 = 0;
    let mut current: u32 = 0;
    for l in _input.lines() {
        if l.len() != 0 {
            let value: u32 = l.parse().unwrap();
            current = current + value;
        }
        if current > max {
            max = current;
        }
        if l.len() == 0 {
            current = 0;
        }
    }
    dbg!(max);
}

pub fn run(_input: &String) {
    run_part(_input, 1);
    // run_part(_input, 2);
}
