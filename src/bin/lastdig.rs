use std::io::BufRead;

/**
LASTDIG - The last digit

Nestor was doing the work of his math class about three days but he is tired of make operations a lot and he should deliver his task tomorrow. His math’s teacher gives him two numbers a and b. The problem consist of finding the last digit of the potency of base a and index b. Help Nestor with his problem. You are given two integer numbers: the base a (0 <= a <= 20) and the index b (0 <= b <= 2,147,483,000), a and b both are not 0. You have to find the last digit of ab.
Input

The first line of input contains an integer t, the number of test cases (t <= 30). t test cases follow. For each test case will appear a and b separated by space.
Output

For each test case output an integer per line representing the result.
Example

Input:
2
3 10
6 2

Output:
9
6
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(n) {
        let line = line.unwrap();
        let (a, b) = line.split_once(' ').unwrap();
        let (a, b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());

        println!("{}", modular_pow(a, b, 10));
    }
}

fn modular_pow(mut base: usize, mut exp: usize, modulus: usize) -> usize {
    if modulus == 1 {
        return 0;
    }

    let mut res = 1;
    base %= modulus;
    while exp > 0 {
        if (exp % 2) == 1 {
            res = (res * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }

    res
}
