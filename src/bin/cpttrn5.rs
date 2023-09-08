use std::io::BufRead;

/**
 * CPTTRN5 - Character Patterns (Act 5)
 *
 * Using two characters: . (dot) and * (asterisk) print a grid-like pattern. The grid will have l lines, c columns, and each
 * square shaped element of the grid will have the height and width equal to s.
 *
 * Moreover, each of the grid elements will have a diagonal. The diagonal of the first square in the first line of the grid
 * is directed towards down and right corner - use the \ (backslash) character to print it; while the next diagonal will be
 * directed towards upper right corner - use the / (slash) character to print it. Print the successive diagonals alternately
 * (please consult the example below).
 *
 * Input
 * You are given t - the number of test cases and for each of the test case three positive integers: l - the number of
 * lines, c - the number of columns in the grid and s - the size of the single square shaped element.
 *
 * Output
 * For each of the test cases output the requested pattern (please have a look at the example). Use one line break in
 * between successive patterns.
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
        let size = line.next().unwrap();

        let border: String = std::iter::once('*')
            .cycle()
            .take((size + 1) * c + 1)
            .collect();
        let mut forward: Vec<String> = vec![String::new(); size];
        let mut backward: Vec<String> = vec![String::new(); size];
        for i in 0..size {
            for j in 0..size {
                if i == size - j - 1 {
                    forward[i].push('/');
                } else {
                    forward[i].push('.');
                }

                if i == j {
                    backward[i].push('\\');
                } else {
                    backward[i].push('.');
                }
            }

            forward[i].push('*');
            backward[i].push('*');
        }

        println!("{}", border);
        for i in 0..l * (size + 1) {
            let y = i / (size + 1);
            let part = i % (size + 1);

            if part == size {
                println!("{}", border);
            } else {
                print!("*");

                for x in 0..c {
                    if (x + y) % 2 == 1 {
                        print!("{}", &forward[part]);
                    } else {
                        print!("{}", &backward[part]);
                    }
                }

                println!();
            }
        }

        // End test case with newline
        println!();
    }
}
