use std::io::BufRead;

/**
 * FCTRL2 - Small factorials
 *
 * You are asked to calculate factorials of some small positive integers.
 *
 * Input
 * An integer t, 1<=t<=100, denoting the number of testcases, followed by t lines, each containing a single integer n, 1<=n<=100.
 *
 * Output
 * For each integer n given at input, display a line with the value of n!
 *
 * Example
 *   Sample input:
 *   4
 *   1
 *   2
 *   5
 *   3
 *   
 *   Sample output:
 *   1
 *   2
 *   120
 *   6
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(n) {
        let line = line.unwrap();
        let number = line.parse().unwrap();
        println!("{}", factorial(number));
    }
}

fn factorial(n: usize) -> String {
    if n <= 1 {
        return "1".into();
    }

    let mut result = vec![2];
    for i in 3..=n {
        result = multiply(&result, &number_to_vec(i));
    }

    // Reversing digits and returning string
    result.reverse();
    result
        .into_iter()
        .map(|digit| digit.to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn number_to_vec(mut n: usize) -> Vec<usize> {
    if n == 0 {
        return vec![0];
    }

    let mut digits = vec![];
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits
}

fn multiply(a: &[usize], b: &[usize]) -> Vec<usize> {
    let mut result = vec![0; a.len() + b.len()];

    // Multiply
    for i in 0..a.len() {
        for j in 0..b.len() {
            result[i + j] += a[i] * b[j];
        }
    }

    // Overflow
    for i in 0..result.len() - 1 {
        if result[i] >= 10 {
            result[i + 1] += result[i] / 10;
            result[i] %= 10;
        }
    }

    // Reduce
    while result.last() == Some(&0) {
        result.pop();
    }

    result
}
