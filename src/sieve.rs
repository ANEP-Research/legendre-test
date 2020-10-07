use bitset_core::BitSet;

const MAX_MASK: usize = 64;

// O(nlogn) Linear Sieve
pub fn prime_sieve(n: u32) -> Vec<u32> {
    let mut is_composite: Vec<u64> = vec![0; ((n as usize) / MAX_MASK) + 2];
    let mut primes: Vec<u32> = Vec::new();
    for p in 2..=n {
        if !is_composite.bit_test(p as usize) {
            primes.push(p);
        }
        for j in 0..primes.len() {
            let m = (p as u64) * (primes[j] as u64);
            if m > n as u64 {
                break;
            }
            is_composite.bit_set(m as usize);
        }
    }
    primes
}
