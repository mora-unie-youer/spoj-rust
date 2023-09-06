use std::io::BufRead;

/**
 * CPTTRN3 - Character Patterns (Act 3)
 *
 * Using two characters: . (dot) and * (asterisk) print a grid-like pattern.
 *
 * Input
 * You are given t - the number of test cases and for each of the test cases two positive integers: l - the number of lines
 * and c - the number of columns in the grid. Each square of the grid is of the same size and filled with 4 dots (see the
 * example below).
 *
 * Output
 * For each of the test cases output the requested pattern (please have a look at the example). Use one line break in
 * between successive patterns.
 *
 * Example
 * Input:
 * 3
 * 3 1
 * 4 4
 * 2 5
 *
 * Output:
 * ****
 * *..*
 * *..*
 * ****
 * *..*
 * *..*
 * ****
 * *..*
 * *..*
 * ****
 *
 * *************
 * *..*..*..*..*
 * *..*..*..*..*
 * *************
 * *..*..*..*..*
 * *..*..*..*..*
 * *************
 * *..*..*..*..*
 * *..*..*..*..*
 * *************
 * *..*..*..*..*
 * *..*..*..*..*
 * *************
 *
 * ****************
 * *..*..*..*..*..*
 * *..*..*..*..*..*
 * ****************
 * *..*..*..*..*..*
 * *..*..*..*..*..*
 * ****************
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(t) {
        let line = line.unwrap();
        let (l, c) = line.split_once(' ').unwrap();
        let (l, c): (usize, usize) = (l.parse().unwrap(), c.parse().unwrap());

        let border: String = std::iter::once('*').cycle().take(3 * c + 1).collect();
        let content: String = std::iter::once("..*").cycle().take(c).collect();

        println!("{}", border);
        for _ in 0..l {
            println!("*{}", content);
            println!("*{}", content);
            println!("{}", border);
        }

        // End test case with newline
        println!();
    }
}
