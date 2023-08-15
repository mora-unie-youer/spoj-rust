use std::io::BufRead;

/**
 * STRHH - Half of the half
 *
 * Given a sequence of 2*k characters, please print every second character from the first half of the sequence. Start
 * printing with the first character.
 *
 * Input
 * In the first line of input you are given the positive integer t (1<=t<=100) - the number of test cases. In the each of
 * the next t lines, you are given a sequence of 2*k (1<=k<=100) characters.
 *
 * Output
 * For each of the test cases please please print every second character from the first half of a given sequence (the
 * first character should appear).
 *
 * Example
 *   Input:
 *   4
 *   your
 *   progress
 *   is
 *   noticeable
 *   
 *   Output:
 *   y
 *   po
 *   i
 *   ntc
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(t) {
        let line = line.unwrap();
        let processed_string: String = line.chars().take(line.len() / 2).step_by(2).collect();
        println!("{}", processed_string);
    }
}
