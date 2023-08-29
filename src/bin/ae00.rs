use std::io::BufRead;

/**
 * AE00 - Rectangles
 *
 * Byteman has a collection of N squares with side 1. How many different rectangles can he form using these squares?
 *
 * Two rectangles are considered different if none of them can be rotated and moved to obtain the second one. During
 * rectangle construction, Byteman can neither deform the squares nor put any squares upon any other ones.
 *
 * Input
 * The first and only line of the standard input contains one integer N (1 <= N <= 10000).
 *
 * Output
 * The first and only line of the standard output should contain a single integer equal to the number of different
 * rectangles that Byteman can form using his squares.
 *
 * Example
 *   For the input data:
 *   6
 *   
 *   the correct result is:
 *   8
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut i = 1;
    let mut answer = 0;
    while i * i <= n {
        let j = n / i;
        answer += j.saturating_sub(i - 1);
        i += 1;
    }

    println!("{}", answer);
}
