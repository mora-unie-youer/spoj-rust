use std::io::BufRead;

/**
 * LCMSUM - LCM Sum
 *
 * Given n, calculate the sum LCM(1,n) + LCM(2,n) + .. + LCM(n,n), where LCM(i,n) denotes the Least Common Multiple of the
 * integers i and n.
 *
 * Input
 * The first line contains T the number of test cases. Each of the next T lines contain an integer n.
 *
 * Output
 * Output T lines, one for each test case, containing the required sum.
 *
 * Example
 *   Sample Input:
 *   3
 *   1
 *   2
 *   5
 *
 *   Sample Output:
 *   1
 *   4
 *   55
 *
 * Constraints
 *   1 <= T <= 300000
 *   1 <= n <= 1000000
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    // Precalculating Euler's totient
    const N: usize = 1_000_000;
    let mut phi: Vec<usize> = (0..=N).collect();
    for i in 2..=N {
        if phi[i] == i {
            for j in (i..=N).step_by(i) {
                phi[j] /= i;
                phi[j] *= i - 1;
            }
        }
    }

    let mut phi_sum = vec![0; N + 1];
    for (i, phi) in phi.iter().enumerate().skip(1) {
        for j in (i..=N).step_by(i) {
            phi_sum[j] += i * phi;
        }
    }

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(t) {
        let line = line.unwrap();
        let n: usize = line.parse().unwrap();
        println!("{}", (phi_sum[n] + 1) * n / 2);
    }
}
