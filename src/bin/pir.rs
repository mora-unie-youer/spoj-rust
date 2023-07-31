use std::io::BufRead;

/**
 * PIR - Pyramids
 *
 * Recently in Farland, a country in Asia, the famous scientist Mr. Log Archeo discovered ancient pyramids. But unlike those
 * in Egypt and Central America, they have a triangular (not rectangular) foundation. That is, they are tetrahedrons in the
 * mathematical sense. In order to find out some important facts about the early society of the country (it is widely
 * believed that the pyramid sizes are closely connected with Farland's ancient calendar), Mr. Archeo needs to know the
 * volume of the pyramids. Unluckily, he has reliable data about their edge lengths only. Please, help him!
 *
 * Input
 * t [number of tests to follow] In each of the next t lines six positive integer numbers not exceeding 1000 separated by
 * spaces (each number is one of the edge lengths of the pyramid ABCD). The order of the edges is the following: AB, AC, AD,
 * BC, BD, CD.
 *
 * Output
 * For each test output a real number - the volume, printed accurate to four digits after decimal point.
 *
 * Example
 *   Input:
 *   2
 *   1 1 1 1 1 1
 *   1000 1000 1000 3 4 5
 *   
 *   Output:
 *   0.1179
 *   1999.9937
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(n) {
        let line = line.unwrap();
        let edges: Vec<f64> = line
            .split_ascii_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();

        let (ab, ac, ad, bc, bd, cd) = (edges[0], edges[1], edges[2], edges[3], edges[4], edges[5]);

        let a = (bc * bc) + (bd * bd) - (cd * cd);
        let b = (bd * bd) + (ab * ab) - (ad * ad);
        let c = (ab * ab) + (bc * bc) - (ac * ac);
        let volume = ((4. * ab * ab * bc * bc * bd * bd)
            - (ab * ab * a * a)
            - (bc * bc * b * b)
            - (bd * bd * c * c)
            + (a * b * c))
            .sqrt()
            / 12.;

        println!("{:.4}", volume);
    }
}
