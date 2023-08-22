use std::io::BufRead;

/**
 * ELIS - Easy Longest Increasing Subsequence
 *
 * Given a list of numbers A output the length of the longest increasing subsequence. An increasing subsequence is defined
 * as a set {i0 , i1 , i2 , i3 , ... , ik} such that 0 <= i0 < i1 < i2 < i3 < ... < ik < N and
 * A[ i0 ] < A[ i1 ] < A[ i2 ] < ... < A[ik]. A longest increasing subsequence is a subsequence with the maximum k (length).
 *
 * i.e. in the list {33 , 11 , 22 , 44}
 * the subsequence {33 , 44} and {11} are increasing subsequences while {11 , 22 , 44} is the longest increasing subsequence.
 *
 * Input
 * First line contain one number N (1 <= N <= 10) the length of the list A.
 * Second line contains N numbers (1 <= each number <= 20), the numbers in the list A separated by spaces.
 *
 * Output
 * One line containing the lenght of the longest increasing subsequence in A.
 *
 * Example
 *   Input:
 *   5
 *   1 4 2 4 3
 *   
 *   Output:
 *   3
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let numbers: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let mut length_table = vec![1; numbers.len()];
    for i in 1..n {
        for j in 0..i {
            if length_table[i] <= length_table[j] && numbers[i] > numbers[j] {
                length_table[i] = length_table[j] + 1;
            }
        }
    }

    let max_length = length_table.into_iter().max().unwrap();
    println!("{}", max_length);
}
