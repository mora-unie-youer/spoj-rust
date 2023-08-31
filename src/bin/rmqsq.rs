use std::io::BufRead;

/**
 * RMQSQ - Range Minimum Query
 *
 * You are given a list of N numbers and Q queries. Each query is specified by two numbers i and j; the answer to each query
 * is the minimum number between the range [i, j] (inclusive).
 *
 * Note: the query ranges are specified using 0-based indexing.
 *
 * Input
 * The first line contains N, the number of integers in our list (N <= 100,000). The next line holds N numbers that are
 * guaranteed to fit inside an integer. Following the list is a number Q (Q <= 10,000). The next Q lines each contain two
 * numbers i and j which specify a query you must answer (0 <= i, j <= N-1).
 *
 * Output
 * For each query, output the answer to that query on its own line in the order the queries were made.
 *
 * Example
 *   Input:
 *   3
 *   1 4 1
 *   2
 *   1 1
 *   1 2
 *
 *   Output:
 *   4
 *   1
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let nums: Vec<isize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .take(n)
        .map(|v| v.parse().unwrap())
        .collect();

    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(q) {
        let line = line.unwrap();

        let (i, j) = line.split_once(' ').unwrap();
        let (i, j): (usize, usize) = (i.parse().unwrap(), j.parse().unwrap());
        let min = nums[i..=j].iter().min().unwrap();

        println!("{}", min);
    }
}
