use std::io::BufRead;

/**
 * TEST - Life, the Universe, and Everything
 *
 * Your program is to use the brute-force approach in order to find the Answer to Life, the Universe, and Everything. More
 * precisely... rewrite small numbers from input to output. Stop processing input after reading in the number 42. All
 * numbers at input are integers of one or two digits.
 *
 * Example 1:
 *   Input:
 *   1
 *   2
 *   88
 *   42
 *   99
 *   
 *   Output:
 *   1
 *   2
 *   88
 */
fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    for line in lines {
        let line = line.unwrap();
        let n: usize = line.parse().unwrap();

        if n == 42 {
            break;
        }

        println!("{}", n);
    }
}
