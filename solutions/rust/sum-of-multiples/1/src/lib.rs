pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    let mut final_multiples = Vec::new();
    for f in factors {
        if *f as i32  == 0 {
            continue;
        }
        let multiples = get_multiples(limit, f);
        for multiple in multiples {
            if !final_multiples.contains(&multiple) {
                final_multiples.push(multiple)
            }
        }
    }
    for fm in final_multiples {
        sum += fm;
    }
    sum as u32
}

fn get_multiples(limit: u32, cf: &u32) -> Vec<u32> {
    let mut res = Vec::new();
    let mut i = cf.clone();
    while i < limit  {
        if i % cf == 0 {
           res.push(i);
        }
        i += 1;
    }
    res
}