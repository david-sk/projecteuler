//
// Prime numbers logic
//

pub fn is_prime(n: u64) -> bool {
    if n < 3 || n % 2 == 0 {
        return n == 2;
    }
    let end = (n as f64).sqrt() as u64;
    for i in (3..=end).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
