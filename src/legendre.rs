pub fn pow(a: u64, x: usize, p: u64) -> u64 {
    let (mut a_t, mut x_t, mut r) = (a, x, 1);
    while x_t != 0 {
        if (x_t & 1) == 1 {
            r *= a_t;
            r %= p;
        }
        a_t *= a_t;
        a_t %= p;
        x_t >>= 1;
    }
    r
}

pub fn legendre(p: u64, a: u64) -> bool {
    let l = pow(a, ((p - 1) / 2) as usize, p);
    if l == 1 {
        true
    } else {
        false
    }
}
