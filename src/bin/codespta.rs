use std::{cmp::Ordering, io::BufRead};

/**
 * CODESPTA - 2s Complement
 *
 * One of the basics of Computer Science is knowing how numbers are represented in 2's complement. Imagine that you write down all numbers between A and B in 2's complement representation using 32 bits. How many 1's will you write down in all?
 *
 * Input
 * The first line contains the number of test cases T. Each of the next T lines contains two integers A and B.
 *
 * Output
 * Output T lines, one corresponding to each test case.
 *
 * Constraints:
 * -2^31 <= A <= B <= 2^31 - 1
 *
 * Sample
 *   Input:
 *   3
 *   -2 0
 *   -3 4
 *   -1 4
 *   
 *   Output:
 *   63
 *   99
 *   37
 *
 * Explanation
 *   For the first case, -2 contains 31 1's followed by a 0 whereas -1 contains 32 1's. Thus the total is 63.
 *   For the second case, the answer is 31 + 31 + 32 + 0 + 1 + 1 + 2 + 1 = 99
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(t) {
        let line = line.unwrap();
        let (a, b) = line.split_once(' ').unwrap();
        let (a, b): (isize, isize) = (a.parse().unwrap(), b.parse().unwrap());

        let count = match (a.cmp(&0), b.cmp(&0)) {
            (Ordering::Equal, _) | (_, Ordering::Equal) | (Ordering::Less, Ordering::Greater) => {
                count_all_bits(a) + count_all_bits(b)
            }
            (Ordering::Less, Ordering::Less) => count_all_bits(a) - count_all_bits(b + 1),
            (Ordering::Greater, Ordering::Greater) => count_all_bits(b) - count_all_bits(a - 1),
            _ => 0,
        };

        // let mut count = 0;
        // for v in a..=b {
        //     count += v.count_ones() as usize;
        // }

        println!("{}", count);
    }
}

fn count_all_bits(x: isize) -> usize {
    if x >= 0 {
        count_bits(x as usize)
    } else {
        let x = -x as usize;
        x * 32 - count_bits(x.overflowing_sub(1).0)
    }
}

fn count_bits(x: usize) -> usize {
    if x == 0 {
        return 0;
    }

    // As we have 32-bit number -> [0..31]
    let bit = 63 - x.leading_zeros();
    let over = x - (1 << bit);

    let pow = if bit == 0 {
        0
    } else {
        (bit as usize) << (bit - 1)
    };

    pow + over + count_bits(over) + 1
}
