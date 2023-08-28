use std::io::BufRead;

/**
 * SMPDIV - Divisibility
 *
 * Print all integers ai such that ai is divisible by x and not divisible by y, where 1 < ai < n < 100000.
 *
 * Input
 * First, you are given t (t < 100) - the number of test cases. In each of the following t lines, 3 integers: n x y.
 *
 * You might assume also that x < n and x is not divisible by y.
 *
 * Output
 * In each of the following t lines, numbers requested in the problem description in the separated by a single space in
 * ascending order.
 *
 * Example
 *   Input:
 *   2
 *   7 2 4
 *   35 5 12
 *   
 *   Output:
 *   2 6
 *   5 10 15 20 25 30
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(t) {
        let line = line.unwrap();
        let mut line = line.split_ascii_whitespace().map(|v| v.parse().unwrap());

        let n: usize = line.next().unwrap();
        let x = line.next().unwrap();
        let y = line.next().unwrap();

        let start = 2.max(x);
        let result = (start..n)
            .step_by(x)
            .filter(|a| a % y != 0)
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", result);
    }
}
