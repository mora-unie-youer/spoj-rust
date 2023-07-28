use std::io::BufRead;

/**
 * PALIN - The Next Palindrome
 *
 * A positive integer is called a palindrome if its representation in the decimal system is the same when read from left to
 * right and from right to left. For a given positive integer K of not more than 1000000 digits, write the value of the
 * smallest palindrome larger than K to output. Numbers are always displayed without leading zeros.
 *
 * Input
 * The first line contains integer t, the number of test cases. Integers K are given in the next t lines.
 *
 * Output
 * For each K, output the smallest palindrome larger than K.
 *
 * Example
 *   Input:
 *   2
 *   808
 *   2133
 *   
 *   Output:
 *   818
 *   2222
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(n) {
        let line = line.unwrap();
        let number: Vec<u8> = line.chars().map(|ch| ch as u8 - b'0').collect();
        let next_palindrome = next_palindrome(&number);
        let next_palindrome: String = next_palindrome
            .into_iter()
            .map(|digit| (digit + b'0') as char)
            .collect();
        println!("{}", next_palindrome);
    }
}

fn next_palindrome(number: &[u8]) -> Vec<u8> {
    if number.iter().all(|&digit| digit == 9) {
        let mut next_palindrome = vec![1];
        next_palindrome.extend(vec![0; number.len() - 1]);
        next_palindrome.push(1);
        next_palindrome
    } else {
        let mut new_number = number.to_vec();

        let mid = number.len() / 2;
        let mut i = mid - 1;
        let mut j = mid + number.len() % 2;
        let mut smaller = false;

        while j < number.len() && number[i] == number[j] {
            i = i.saturating_sub(1);
            j += 1;
        }

        if j == number.len() || number[i] < number[j] {
            smaller = true;
        }

        while j < number.len() {
            new_number[j] = new_number[i];
            i = i.saturating_sub(1);
            j += 1
        }

        if smaller {
            let mut carry = 1;
            i = mid - 1;

            if number.len() % 2 == 1 {
                new_number[mid] += carry;
                carry = new_number[mid] / 10;
                new_number[mid] %= 10;
                j = mid + 1;
            } else {
                j = mid;
            }

            while j < number.len() {
                new_number[i] += carry;
                carry = new_number[i] / 10;
                new_number[i] %= 10;

                new_number[j] = new_number[i];
                i = i.saturating_sub(1);
                j += 1;
            }
        }

        new_number
    }
}
