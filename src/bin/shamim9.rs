use std::io::BufRead;

/**
 * SHAMIM9 - Antik the Numerologist
 *
 * Siam Hasan Antik is the great Numerologist in Sylhet International University . One day he wants to work with the last
 * digit of those numbers that are multiplied by 9.
 *
 * You need to help him.
 *
 * Input
 * There are only input N (2<N<1040 ).
 *
 * Output
 * print only last digit of 9*N.
 *
 * Example
 *   Input:
 *   3
 *   Output:
 *   7
 *
 *   Input:
 *   99
 *   Output:
 *   1
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let last_digit = n % 10;
    println!("{}", last_digit * 9 % 10);
}
