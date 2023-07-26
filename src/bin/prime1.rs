use std::io::BufRead;

/**
 * PRIME1 - Prime Generator
 *
 * Peter wants to generate some prime numbers for his cryptosystem. Help him! Your task is to generate all prime numbers
 * between two given numbers!
 *
 * Input
 * The input begins with the number t of test cases in a single line (t<=10). In each of the next t lines there are two
 * numbers m and n (1 <= m <= n <= 1000000000, n-m<=100000) separated by a space.
 *
 * Output
 * For every test case print all prime numbers p such that m <= p <= n, one number per line, test cases separated by an
 * empty line.
 *
 * Example:
 *   Input:
 *   2
 *   1 10
 *   3 5
 *   
 *   Output:
 *   2
 *   3
 *   5
 *   7
 *   
 *   3
 *   5
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(n) {
        let line = line.unwrap();
        let (a, b) = line.split_once(' ').unwrap();
        let (a, b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());

        for v in a..=b {
            if is_prime(v) {
                println!("{}", v);
            }
        }

        println!();
    }
}

fn is_prime(v: usize) -> bool {
    if v <= 1 {
        return false;
    }

    let mut i = 2;
    while i * i <= v {
        if v % i == 0 {
            return false;
        }

        i += 1;
    }

    true
}
