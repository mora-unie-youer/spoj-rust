use std::io::BufRead;

/**
 * CPTTRN4 - Character Patterns (Act 4)
 *
 * Using two characters: . (dot) and * (asterisk) print a grid-like pattern.
 *
 * Input
 * You are given t - the number of test cases and for each of the test cases four positive integers: l - the number of
 * lines, c - the number of columns in the grid; h and w - the high and the width of the single rectangle.
 *
 * Output
 * For each of the test cases output the requested pattern (please have a look at the example). Use one line break in
 * between successive patterns.
 *
 * Example
 *   Input:
 *   3
 *   3 1 2 1
 *   4 4 1 2
 *   2 5 2 2
 *   
 *   Output:
 *   ***
 *   *.*
 *   *.*
 *   ***
 *   *.*
 *   *.*
 *   ***
 *   *.*
 *   *.*
 *   ***
 *   
 *   *************
 *   *..*..*..*..*
 *   *************
 *   *..*..*..*..*
 *   *************
 *   *..*..*..*..*
 *   *************
 *   *..*..*..*..*
 *   *************
 *   
 *   ****************
 *   *..*..*..*..*..*
 *   *..*..*..*..*..*
 *   ****************
 *   *..*..*..*..*..*
 *   *..*..*..*..*..*
 *   ****************
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(t) {
        let line = line.unwrap();
        let mut line = line.split_ascii_whitespace().map(|v| v.parse().unwrap());
        let rows: usize = line.next().unwrap();
        let cols = line.next().unwrap();
        let h = line.next().unwrap();
        let w = line.next().unwrap();

        let border: String = std::iter::once('*')
            .cycle()
            .take((w + 1) * cols + 1)
            .collect();
        let cell: String = std::iter::once('.')
            .cycle()
            .take(w)
            .chain(std::iter::once('*'))
            .collect();
        let content: String = std::iter::once(cell.as_str()).cycle().take(cols).collect();

        println!("{}", border);
        for _ in 0..rows {
            for _ in 0..h {
                println!("*{}", content);
            }
            println!("{}", border);
        }

        // End test case with newline
        println!();
    }
}
