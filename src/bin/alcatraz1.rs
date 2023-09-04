use std::io::BufRead;

/**
 * ALCATRAZ1 - SUM OF DIGITS
 *
 * You are being given a number N. (1<=N<=10^50) . You have to print the sum of digits of that particular number.
 *
 * Input
 * The first line will contain T, the number of testcases (T<10) . The next T lines will contain the numbers whose sum of
 * digits you have to calculate .
 *
 * Output
 * Output T lines  containing the Sum of Digits of the numbers .
 *
 * Example
 *   Input:
 *   3
 *   123123123
 *   3434
 *   1234567890
 *
 *   Output:
 *   18
 *   14
 *   45
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(t) {
        let line = line.unwrap();
        let sum: usize = line.bytes().map(|v| (v - b'0') as usize).sum();
        println!("{}", sum);
    }
}
