//
// Palindrome number logic
//

pub fn is_palindrome(n: i64) -> bool {
    if n < 10 {
        return n >= 0;
    }
    let mut digits = vec![];
    let mut divided_n = n;
    while divided_n > 0 {
        digits.push(divided_n % 10);
        divided_n /= 10;
    }
    let num_digits = digits.len();
    for i in 0..num_digits {
        if digits[i] != digits[num_digits - i - 1] {
            return false;
        }
    }
    return true;
}
