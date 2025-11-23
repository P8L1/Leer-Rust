pub fn nth(n: u32) -> u32 {
    if n == 0  {
        2
    }
    else if n == 1 {
        3
    }
    else if n == 2 {
        5
    } else {
        let mut test = 6;
        let mut primes: Vec<i64> = Vec::new();
        primes.push(2);
        primes.push(3);
        primes.push(5);

        while primes.len() <= n.try_into().unwrap() {
            if is_prime(test) {
                primes.push(test);
            }
            test += 1;
        }
        let un: usize = n as usize;
        primes[un] as u32
    }  
}

fn is_prime(x: i64) -> bool {
    let mut start = 2;
    let mut is_prime = true;
    while start <= x.isqrt() {
        if x % start == 0 {
            is_prime = false;
            break;
        }
        start += 1;
    }
    is_prime
}