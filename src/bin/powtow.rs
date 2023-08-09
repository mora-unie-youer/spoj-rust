use std::io::BufRead;

/**
 * POWTOW - Power Tower City
 *
 * You are living in a city build entirely of power towers such as 3^3^3 and 10^10^10^10. To enter a building you must type
 * the last 9 digits of the number represented by the tower, written in decimal form, on a keypad next to the main entrance.
 * You are not sharp enough at mental maths, but you can write a handy program to bring along in your pocket.
 *
 * A power tower is defined as repeated exponentiation. We write this using Knuth's up-arrow notation as: e↑↑a = e^e^...^e
 * (a terms). Remember that ^ (exponentiation) is right associative.
 * For example: 2↑↑4 = 2^2^2^2 = 2^(2^(2^2)) = 2^2^4 = 2^16 = 65536, and 3↑↑1 = 3. The value of a tower of height 0 is 1.
 *
 * Input
 * The first line contains integer C in [0..1000], the number of test cases.
 *
 * Then follows C lines, each with integers e,a in [0..2147483647]. (non-negative 32-bit integers).
 *
 * Output
 * For each testcase output e↑↑a, or if the output has more than 9 digits, output "..." and then the last 9 digits.
 *
 * Example
 *   Input:
 *   3
 *   0 0
 *   2 5
 *   993306745 75707320
 *   
 *   Output:
 *   1
 *   ...719156736
 *   ...884765625
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(n) {
        let line = line.unwrap();
        let (e, a) = line.split_once(' ').unwrap();
        let (e, a) = (e.parse().unwrap(), a.parse().unwrap());

        // e^a (mod 1_000_000_000)
        const MOD: usize = 1_000_000_000;

        // Tower of height 0
        if a == 0 {
            println!("1");
            continue;
        }

        // Tower of height 1
        if a == 1 {
            if e >= MOD {
                println!("...{:0>9}", e % MOD);
            } else {
                println!("{}", e);
            }
        }

        // If we have number "k * MOD"
        if e % MOD == 0 {
            if e == 0 && a % 2 == 1 {
                println!("1");
            } else if e == 0 && a % 2 == 0 {
                println!("0");
            } else {
                println!("...000000000");
            }

            continue;
        }

        let (result, big) = reduce_tower(e, a, MOD);
        if big {
            println!("...{:0>9}", result);
        } else {
            println!("{}", result);
        }
    }
}

const PHI: [usize; 27] = [
    400000000, 160000000, 64000000, 25600000, 10240000, 4096000, 1638400, 655360, 262144, 131072,
    65536, 32768, 16384, 8192, 4096, 2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1,
];

fn reduce_tower(e: usize, a: usize, modulus: usize) -> (usize, bool) {
    if e % modulus == 0 {
        return (0, false);
    } else if a == 2 {
        return modular_pow(e, e, modulus);
    } else if e == 2 && a == 3 && modulus > 16 {
        return (16, false);
    } else if e == 2 && a == 4 && modulus > 65536 {
        return (65536, false);
    }

    // Finding PHI for this modulus
    let phi = *PHI.iter().find(|&&phi| phi < modulus).unwrap_or(&PHI[0]);
    let log2 = if phi <= modulus {
        (modulus as f64).log2().floor() as usize
    } else {
        0
    };

    // Finding result of "higher" tower
    let (mut result, prev_big) = reduce_tower(e, a - 1, phi);
    if phi != 1 {
        while result < log2 {
            result += phi;
        }
    }

    // Finding result of the power
    let (result, big) = modular_pow(e, result, modulus);

    (result, prev_big | big)
}

// Took it from my AoC 2019 day 22
fn modular_pow(mut base: usize, mut exp: usize, modulus: usize) -> (usize, bool) {
    if modulus == 1 {
        return (0, false);
    }

    let mut res = 1;
    let mut big = false;

    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            res *= base;
            if res > modulus {
                big = true;
                res %= modulus;
            }
        }

        exp >>= 1;

        base *= base;
        if base > modulus {
            // Making it "big" only if it will be multiplied by this base
            big |= exp != 0;
            base %= modulus;
        }
    }

    (res, big)
}
