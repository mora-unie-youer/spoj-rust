use std::io::BufRead;

/**
 * PR003004 - Digit Sum
 *
 * For a pair of integers a and b, the digit sum of the interval [a,b] is defined as the sum of all digits occurring in all
 * numbers between (and including) a and b. For example, the digit sum of [28, 31] can be calculated as:
 *   2+8  +  2+9  +  3+0  +  3+1 = 28
 *
 * Given the numbers a and b, calculate the digit sum of [a,b].
 *
 * Input
 * On the first line one positive number: the number of test cases, at most 100.
 *
 * After that per test case:
 *   one line with two space-separated integers, a and b (0 <= a <= b <= 10^15).
 *
 * Output
 * Per test case:
 *   one line with an integer: the digit sum of [a,b];
 *
 * Example
 *   Input:
 *   3
 *   0 10
 *   28 31
 *   1234 56789
 *   
 *   Output:
 *   46
 *   28
 *   1128600
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(t) {
        let line = line.unwrap();
        let (a, b) = line.split_once(' ').unwrap();
        let (a, b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());

        let a_sum = find_range_digit_sum(a.saturating_sub(1), 1);
        let b_sum = find_range_digit_sum(b, 1);
        println!("{}", b_sum - a_sum);
    }
}

fn find_range_digit_sum(x: usize, mul: usize) -> usize {
    if x == 0 {
        return 0;
    }

    let (a, b) = (x / 10, x % 10);
    let mut sum = b * (b + 1) / 2 * mul;
    sum += digit_sum(a) * (b + 1) * mul;
    sum += a * 45 * mul;
    sum += find_range_digit_sum(a.saturating_sub(1), mul * 10);
    sum
}

fn digit_sum(mut x: usize) -> usize {
    let mut sum = 0;

    while x != 0 {
        sum += x % 10;
        x /= 10;
    }

    sum
}
