use std::io::BufRead;

/**
 * PIR - Pyramids
 *
 * Recently in Farland, a country in Asia, the famous scientist Mr. Log Archeo discovered ancient pyramids. But unlike those
 * in Egypt and Central America, they have a triangular (not rectangular) foundation. That is, they are tetrahedrons in the
 * mathematical sense. In order to find out some important facts about the early society of the country (it is widely
 * believed that the pyramid sizes are closely connected with Farland's ancient calendar), Mr. Archeo needs to know the
 * volume of the pyramids. Unluckily, he has reliable data about their edge lengths only. Please, help him!
 *
 * Input
 * t [number of tests to follow] In each of the next t lines six positive integer numbers not exceeding 1000 separated by
 * spaces (each number is one of the edge lengths of the pyramid ABCD). The order of the edges is the following: AB, AC, AD,
 * BC, BD, CD.
 *
 * Output
 * For each test output a real number - the volume, printed accurate to four digits after decimal point.
 *
 * Example
 *   Input:
 *   2
 *   1 1 1 1 1 1
 *   1000 1000 1000 3 4 5
 *   
 *   Output:
 *   0.1179
 *   1999.9937
 */
fn main() {
    const DIGITS: [[&str; 5]; 10] = [
        // 0
        ["-----", "|   |", "|   |", "|   |", "-----"],
        // 1
        ["    -", "    |", "    |", "    |", "    -"],
        // 2
        ["-----", "    |", "|---|", "|    ", "-----"],
        // 3
        ["-----", "    |", "|---|", "    |", "-----"],
        // 4
        ["-   -", "|   |", "|---|", "    |", "    -"],
        // 5
        ["-----", "|    ", "|---|", "    |", "-----"],
        // 6
        ["-----", "|    ", "|---|", "|   |", "-----"],
        // 7
        ["-----", "    |", "    |", "    |", "    -"],
        // 8
        ["-----", "|   |", "|---|", "|   |", "-----"],
        // 9
        ["-----", "|   |", "|---|", "    |", "-----"],
    ];

    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for (i, line) in lines.take(t).enumerate() {
        // Empty line between cases
        if i != 0 {
            println!();
        }

        let line = line.unwrap();
        let mut n: usize = line.parse().unwrap();

        let digits = if n == 0 {
            vec![0]
        } else {
            let mut digits = vec![];

            while n > 0 {
                digits.push(n % 10);
                n /= 10;
            }

            digits.reverse();
            digits
        };

        let digit_rows = (0..5).map(|row| {
            digits
                .iter()
                .map(|&digit| DIGITS[digit][row])
                .collect::<Vec<_>>()
                .join(" ")
        });

        for row in digit_rows {
            println!("{}", row);
        }
    }
}
