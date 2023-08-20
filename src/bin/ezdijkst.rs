use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    io::BufRead,
};

/**
 * EZDIJKST - Easy Dijkstra Problem
 *
 * Determine the shortest path between the specified vertices in the graph given in the input data.
 * Hint: You can use Dijkstra's algorithm.
 * Hint 2: if you're a lazy C++ programmer, you can use set and cin/cout (with sync_with_stdio(0)) - it should suffice.
 *
 * Input
 * First line - one integer - number of test cases
 * For each test case the numbers V, K (number of vertices, number of edges) are given.
 * Then K lines follow, each containing the following numbers separated by a single space:
 * ai, bi, ci
 * It means that the graph being described contains an edge from ai to bi, with a weight of ci.
 * Below the graph description a line containing a pair of integers A, B is present.
 * The goal is to find the shortest path from vertex A to vertex B.
 * All numbers in the input data are integers in the range 0..10000.
 *
 * Output
 * For each test case your program should output (in a separate line) a single number C - the length of the shortest path
 * from vertex A to vertex B. In case there is no such path, your program should output a single word "NO" (without quotes)
 *
 * Example
 *   Input:
 *   3
 *   3 2
 *   1 2 5
 *   2 3 7
 *   1 3
 *   3 3
 *   1 2 4
 *   1 3 7
 *   2 3 1
 *   1 3
 *   3 1
 *   1 2 4
 *   1 3
 *   
 *   Output:
 *   12
 *   5
 *   NO
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let first_line = lines.next().unwrap().unwrap();
        let (v, k) = first_line.split_once(' ').unwrap();
        let (v, k): (usize, usize) = (v.parse().unwrap(), k.parse().unwrap());

        let mut graph = vec![vec![]; v];
        for line in lines.by_ref().take(k) {
            let line = line.unwrap();
            let mut line = line.split_ascii_whitespace().map(|v| v.parse().unwrap());

            let vertex1: usize = line.next().unwrap();
            let vertex2 = line.next().unwrap();
            let weight = line.next().unwrap();

            let (vertex1, vertex2) = (vertex1 - 1, vertex2 - 1);
            // Graph is directed
            graph[vertex1].push((weight, vertex2));
            // graph[vertex2].push((weight, vertex1));
        }

        let query_line = lines.next().unwrap().unwrap();
        let (from, to) = query_line.split_once(' ').unwrap();
        let (from, to): (usize, usize) = (from.parse().unwrap(), to.parse().unwrap());
        let (from, to) = (from - 1, to - 1);
        let distances = dijkstra(&graph, from);

        match distances[to] {
            std::usize::MAX => println!("NO"),
            _ => println!("{}", distances[to]),
        }
    }
}

fn dijkstra(graph: &[Vec<(usize, usize)>], start: usize) -> Vec<usize> {
    let mut distances = vec![std::usize::MAX; graph.len()];

    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start)));

    while let Some(Reverse((distance, from))) = queue.pop() {
        if !visited.insert(from) {
            continue;
        }

        for &(cost, neighbor) in &graph[from] {
            let new_distance = distance + cost;
            if new_distance < distances[neighbor] {
                distances[neighbor] = new_distance;
                queue.push(Reverse((new_distance, neighbor)));
            }
        }
    }

    distances
}
