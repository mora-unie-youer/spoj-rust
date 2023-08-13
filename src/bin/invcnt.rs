use std::io::BufRead;

/**
 * INVCNT - Inversion Count
 *
 * Let A[0...n - 1] be an array of n distinct positive integers. If i < j and A[i] > A[j] then the pair (i, j) is called an
 * inversion of A. Given n and an array A your task is to find the number of inversions of A.
 *
 * Input
 * The first line contains t, the number of testcases followed by a blank space. Each of the t tests start with a number
 * n (n <= 200000). Then n + 1 lines follow. In the ith line a number A[i - 1] is given (A[i - 1] <= 10^7). The (n + 1)th
 * line is a blank space.
 *
 * Output
 * For every test output one line giving the number of inversions of A.
 *
 * Example
 *   Input:
 *   2
 *   
 *   3
 *   3
 *   1
 *   2
 *   
 *   5
 *   2
 *   3
 *   8
 *   6
 *   1
 *   
 *   
 *   Output:
 *   2
 *   5
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    // Empty line
    let _ = lines.next();

    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

        let arr: Vec<usize> = lines
            .by_ref()
            .take(n)
            .map(Result::unwrap)
            .map(|v| v.parse().unwrap())
            .collect();

        // Normalizing array
        let mut temp = arr.clone();
        temp.sort_unstable();
        let arr = arr.into_iter().filter_map(|v| temp.binary_search(&v).ok());

        // Creating Binary Indexed Tree (Fenwick tree)
        let mut tree = FenwickTree::new(n);

        // Counting inversions
        let mut inversions = 0;
        for i in arr.into_iter().rev() {
            inversions += tree.query(0, i);
            tree.update(i, 1);
        }
        println!("{}", inversions);

        // Empty line
        let _ = lines.next();
    }
}

struct FenwickTree {
    tree: Vec<usize>,
}

impl FenwickTree {
    fn new(len: usize) -> Self {
        Self { tree: vec![0; len] }
    }

    fn len(&self) -> usize {
        self.tree.len()
    }

    fn update(&mut self, mut index: usize, delta: usize) {
        fn next(i: usize) -> usize {
            i | (i + 1)
        }

        if index >= self.len() {
            panic!("Index is bigger than tree size");
        }

        while index < self.len() {
            self.tree[index] += delta;
            index = next(index);
        }
    }

    fn query(&self, mut start: usize, mut end: usize) -> usize {
        fn prev(i: usize) -> usize {
            i & (i - 1)
        }

        let mut sum = 0;

        while end > start {
            sum += self.tree[end - 1];
            end = prev(end);
        }

        while start > end {
            sum += self.tree[start - 1];
            start = prev(start);
        }

        sum
    }
}
