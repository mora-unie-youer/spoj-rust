use std::io::BufRead;

/**
 * HCPC0114 - Just draw a square
 *
 * Your task is to draw a square of size n*n but you have to distinguish the inner area from the circumference.
 *
 * The edge should be drawn using the character '#'.
 *
 * The inner area should be drawn using the character '='.
 *
 * Here is the first few squares:
 *   size 1:
 *   #
 *   
 *   size 2:
 *   ##
 *   ##
 *
 * Input
 * The first line contains one integer T the number of test cases (T < 50), then T lines follows each with integer n the
 * size of the required square (n < 50).
 *
 * Output
 * for each test case draw the required square, please separate every case by an empty line.
 *
 * Example
 *   Input:
 *   2
 *   3
 *   4
 *   
 *   Output:
 *   ###
 *   #=#
 *   ###
 *   
 *   ####
 *   #==#
 *   #==#
 *   ####
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(t) {
        let n: usize = line.unwrap().parse().unwrap();

        if n == 1 {
            println!("#");
        } else if n == 2 {
            println!("##");
            println!("##");
        } else {
            let top_bottom_layer: String = std::iter::once('#').cycle().take(n).collect();
            let middle_layer: String = std::iter::once('#')
                .chain(std::iter::once('=').cycle().take(n - 2))
                .chain(std::iter::once('#'))
                .collect();

            println!("{}", top_bottom_layer);
            for _ in 0..n - 2 {
                println!("{}", middle_layer)
            }
            println!("{}", top_bottom_layer);
        }

        // Empty line between cases
        println!();
    }
}
