use std::io::BufRead;

/**
 * LAGRANGE - Lagrange’s Four-Square Theorem
 *
 * The fact that any positive integer has a representation as the sum of at most four positive squares (i.e. squares of
 * positive integers) is known as Lagrange's Four-Square Theorem. The first published proof of the theorem was given by
 * Joseph-Louis Lagrange in 1770. Your mission however is not to explain the original proof nor to discover a new proof
 * but to show that the theorem holds for some specific numbers by counting how many such possible representations there
 * are. For a given positive integer n, you should report the number of all representations of n as the sum of at most four
 * positive squares. The order of addition does not matter, e.g. you should consider 4^2 + 3^2 and 3^2 + 4^2 are the same
 * representation.
 *
 * For example, let's check the case of 25. This integer has just three representations 1^2+2^2+2^2+4^2, 3^2 + 4^2, and 5^2.
 * Thus you should report 3 in this case. Be careful not to count 4^2 + 3^2 and 3^2 + 4^2 separately.
 *
 * Input
 * The input is composed of at most 255 lines, each containing a single positive integer less than 2^15 , followed by a line
 * containing a single zero. The last line is not a part of the input data.
 *
 * Output
 * The output should be composed of lines, each containing a single integer. No other characters should appear in the
 * output. The output integer corresponding to the input integer n is the number of all representations of n as the sum of
 * at most four positive squares.
 *
 * Example
 *   Input:
 *   1
 *   25
 *   2003
 *   211
 *   20007
 *   0
 *   
 *   Output:
 *   1
 *   3
 *   48
 *   7
 *   738
 */
fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    // Precalculating squares
    let squares: Vec<usize> = (0..192).map(|x| x * x).collect();

    // Precalculating ways of representing every number
    const MAX: usize = 1 << 15;
    let mut ways = vec![0; MAX];

    let mut i = 0;
    while 4 * squares[i] < MAX {
        let mut j = i;
        while squares[i] + 3 * squares[j] < MAX {
            let mut k = j;
            while squares[i] + squares[j] + 2 * squares[k] < MAX {
                let mut l = k;
                while squares[i] + squares[j] + squares[k] + squares[l] < MAX {
                    ways[squares[i] + squares[j] + squares[k] + squares[l]] += 1;
                    l += 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    for line in lines {
        let line = line.unwrap();
        let num: usize = line.parse().unwrap();

        if num == 0 {
            break;
        }

        println!("{}", ways[num]);
    }
}
