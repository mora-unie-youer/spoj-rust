use std::io::BufRead;

/**
 * BSCXOR - XOR
 *
 * Given two logic values p and q (0 or 1) please compute p XOR q.
 *
 * Example
 *   Input:
 *   1 1
 *   
 *   Output:
 *   0
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let (p, q) = first_line.split_once(' ').unwrap();
    let (p, q): (usize, usize) = (p.parse().unwrap(), q.parse().unwrap());
    println!("{}", p ^ q);
}
