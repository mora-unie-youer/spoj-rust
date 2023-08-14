use std::io::BufRead;

/**
 * ALICESIE - Alice Sieve
 *
 * Alice has recently learned to use the Sieve of Eratosthenes, an ancient algorithm for finding all prime numbers up to any
 * given limit. As expected, she was really impressed by it's simplicity and elegancy.
 *
 * Now, she has decided to design her own sieve method: The Sieve of Alice, formally defined by the following procedure,
 * which determines the Sieve of Alice up to a given limit N.
 *   Create a list of consecutive integers from N to 2 (N, N-1, N-2, ..., 3, 2). All of N-1 numbers are initially unmarked.
 *   Initially, let P equal N, and leave this number unmarked.
 *   Mark all the proper divisors of P (i.e. P remains unmarked).
 *   Find the largest unmarked number from 2 to P – 1, and now let P equal this number.
 *   If there were no more unmarked numbers in the list, stop. Otherwise, repeat from step 3.
 * Unfortunately, Alice has not found an useful application for it's Sieve. But she still wants to know, for a given
 * limit N, how many integers will remain unmarked.
 *
 * Input
 * The first line contains an integer T, the number of test cases (1 ≤ T ≤ 10^4) . Each of the next T lines contains an
 * integer N (2 ≤ N ≤ 10^6).
 *
 * Output
 * Output T lines, one for each test case, containing the required answer.
 *
 * Example
 *   Input:
 *   3
 *   2
 *   3
 *   5
 *   
 *   Output:
 *   1
 *   2
 *   3
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(t) {
        let n: usize = line.unwrap().parse().unwrap();
        println!("{}", (n + 1) / 2);
    }
}
