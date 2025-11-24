pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    if s == 1 {
        return 1;
    }
    if s == 2 {
        return 2;
    }
    let mut sqrre = 2;
    let mut grains = 2;
    while sqrre < s {
        grains = grains * 2;
        sqrre += 1;
    } 
    return grains;
}

pub fn total() -> u64 {
    let mut tot = 0;
    for i in 1..=64 {
        tot += square(i);
    }
    return tot;
}
