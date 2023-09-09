use std::io::BufRead;

/**
 * CPTTRN6 - Character Patterns (Act 6)
 *
 * Given specified dimensions, print a grid-like pattern. Use the | (pipe) sign to print vertical elements, the - (minus) to
 * print horizontal ones and + (plus) for crossings. The rest of the space fill with . (dots) characters.
 *
 * Input
 * You are given t - the number of test cases and for each of the test cases four positive integers: l - the number of
 * horizontal elements, c - the number of vertical elements in the grid; h and w - the high and the width of the single
 * rectangle respectively.
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
 *   2 5 3 2
 *
 *   Output:
 *   .|.
 *   .|.
 *   -+-
 *   .|.
 *   .|.
 *   -+-
 *   .|.
 *   .|.
 *   -+-
 *   .|.
 *   .|.
 *
 *   ..|..|..|..|..
 *   --+--+--+--+--
 *   ..|..|..|..|..
 *   --+--+--+--+--
 *   ..|..|..|..|..
 *   --+--+--+--+--
 *   ..|..|..|..|..
 *   --+--+--+--+--
 *   ..|..|..|..|..
 *
 *
 *   ..|..|..|..|..|..
 *   ..|..|..|..|..|..
 *   ..|..|..|..|..|..
 *   --+--+--+--+--+--
 *   ..|..|..|..|..|..
 *   ..|..|..|..|..|..
 *   ..|..|..|..|..|..
 *   --+--+--+--+--+--
 *   ..|..|..|..|..|..
 *   ..|..|..|..|..|..
 *   ..|..|..|..|..|..
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(t) {
        let line = line.unwrap();
        let mut line = line.split_ascii_whitespace().map(|v| v.parse().unwrap());
        let l: usize = line.next().unwrap();
        let c = line.next().unwrap();
        let height = line.next().unwrap();
        let width = line.next().unwrap();

        let length = (c + 1) * (width + 1) - 1;
        let border: String = std::iter::once('-')
            .cycle()
            .take(width)
            .chain(std::iter::once('+'))
            .cycle()
            .take(length)
            .collect();
        let content: String = std::iter::once('.')
            .cycle()
            .take(width)
            .chain(std::iter::once('|'))
            .cycle()
            .take(length)
            .collect();

        for i in 0..=l {
            if i != 0 {
                println!("{}", border);
            }

            for _ in 0..height {
                println!("{}", content);
            }
        }

        // End test case with newline
        println!();
    }
}
