use std::io::BufRead;

/**
 * CPTTRN2 - Character Patterns (Act 2)
 *
 * Using two characters: . (dot) and * (asterisk) print a frame-like pattern.
 *
 * Input
 * You are given t - the number of test cases and for each of the test cases two positive integers: l - the number of lines
 * and c - the number of columns of a frame.
 *
 * Output
 * For each of the test cases output the requested pattern (please have a look at the example). Use one line break in
 * between successive patterns.
 *
 * Example
 *   Input:
 *   3
 *   3 1
 *   4 4
 *   2 5
 *
 *   Output:
 *   *
 *   *
 *   *
 *
 *   ****
 *   *..*
 *   *..*
 *   ****
 *
 *   *****
 *   *****
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(t) {
        let line = line.unwrap();
        let (l, c) = line.split_once(' ').unwrap();
        let (l, c): (usize, usize) = (l.parse().unwrap(), c.parse().unwrap());

        if l == 1 {
            let line: String = std::iter::once('*').cycle().take(c).collect();
            println!("{}", line);
        } else if c == 1 {
            for _ in 0..l {
                println!("*");
            }
        } else {
            for i in 0..l {
                let filler = if i == 0 || i == l - 1 { '*' } else { '.' };

                print!("*");
                for _ in 1..c - 1 {
                    print!("{}", filler);
                }
                println!("*");
            }
        }

        // End test case with newline
        println!();
    }
}
