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

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(t) {
        let line = line.unwrap();
        let (l, c) = line.split_once(' ').unwrap();
        let (l, c): (usize, usize) = (l.parse().unwrap(), c.parse().unwrap());

        for i in 0..l {
            for j in 0..c {
                if (i + j) % 2 == 0 {
                    print!("*");
                } else {
                    print!(".");
                }
            }

            println!();
        }

        println!();
    }
}
