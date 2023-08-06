use std::{
    cmp::Ordering,
    fmt::Display,
    io::BufRead,
    ops::{Add, Div, Mul, Neg, Sub},
};

/**
 * SQRROOT - Square Root
 *
 * In this problem you have to find the Square Root for given number. You may assume that such a number exist and it will be
 * always an integer.
 *
 * Input
 * t - the number of test cases [t <= 50]
 * then t positive numbers follow, each of them have up to 800 digits in decimal representation.
 *
 * Output
 * Output must contain exactly t numbers equal to the square root for given numbers. See sample input/output for details.
 *
 * Example
 *   Input:
 *   3
 *   36
 *   81
 *   226576
 *
 *   Output:
 *   6
 *   9
 *   476
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(n) {
        let line = line.unwrap();
        let num = BigNum::from(line.as_str());
        let sqrt = num.sqrt();
        println!("{}", sqrt);
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Self {
            real: value,
            imag: 0.,
        }
    }
}

impl From<(f64, f64)> for Complex {
    fn from(value: (f64, f64)) -> Self {
        Self {
            real: value.0,
            imag: value.1,
        }
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Self {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Complex;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            real: self.real * rhs,
            imag: self.imag * rhs,
        }
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, rhs: Self) -> Self::Output {
        let norm = rhs.real * rhs.real + rhs.imag * rhs.imag;
        Self {
            real: (self.real * rhs.real + self.imag * rhs.imag) / norm,
            imag: (self.imag * rhs.real - self.real * rhs.imag) / norm,
        }
    }
}

impl Div<f64> for Complex {
    type Output = Complex;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            real: self.real / rhs,
            imag: self.imag / rhs,
        }
    }
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    fn exp(&self) -> Self {
        // e^(a + bi) = e^a * (cos(b) + i*sin(b))
        let scale = self.real.exp();
        let real = scale * self.imag.cos();
        let imag = scale * self.imag.sin();
        Self { real, imag }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct BigNum {
    digits: Vec<usize>,
}

impl From<usize> for BigNum {
    fn from(mut value: usize) -> Self {
        if value == 0 {
            return Self { digits: vec![0] };
        }

        let mut digits = vec![];
        while value > 0 {
            digits.push(value % 10);
            value /= 10;
        }

        Self { digits }
    }
}

impl From<&str> for BigNum {
    fn from(value: &str) -> Self {
        let digits = value.bytes().rev().map(|v| (v - b'0') as usize).collect();
        Self { digits }
    }
}

impl PartialOrd for BigNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigNum {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.len().cmp(&other.len()) {
            Ordering::Equal => (),
            ord => return ord,
        }

        let mut self_digits = self.digits.clone();
        let mut other_digits = other.digits.clone();

        self_digits.reverse();
        other_digits.reverse();

        self_digits.cmp(&other_digits)
    }
}

impl Add for &BigNum {
    type Output = BigNum;

    fn add(self, rhs: Self) -> Self::Output {
        let (mut result, addend) = if self.len() >= rhs.len() {
            (self.clone(), rhs.clone())
        } else {
            (rhs.clone(), self.clone())
        };

        let mut carry = 0;
        for (i, digit) in addend.digits.iter().enumerate() {
            result.digits[i] += digit + carry;

            if result.digits[i] >= 10 {
                result.digits[i] %= 10;
                carry = 1;
            } else {
                carry = 0;
            }
        }

        if carry != 0 {
            let mut i = addend.len();
            while i < result.len() && carry != 0 {
                result.digits[i] += carry;
                if result.digits[i] >= 10 {
                    result.digits[i] %= 10;
                    carry = 1;
                } else {
                    carry = 0;
                }

                i += 1;
            }

            if carry != 0 {
                result.digits.push(carry);
            }
        }

        result
    }
}

impl Add for BigNum {
    type Output = BigNum;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Add<usize> for &BigNum {
    type Output = BigNum;

    fn add(self, rhs: usize) -> Self::Output {
        self + &BigNum::from(rhs)
    }
}

impl Add<usize> for BigNum {
    type Output = BigNum;

    fn add(self, rhs: usize) -> Self::Output {
        &self + rhs
    }
}

impl Sub for &BigNum {
    type Output = BigNum;

    fn sub(self, rhs: Self) -> Self::Output {
        if self < rhs {
            panic!("OVERFLOW");
        }

        let mut borrow = 0;
        let mut digits = self.digits.clone();
        for (i, digit) in digits.iter_mut().enumerate().take(rhs.len()) {
            if *digit < borrow + rhs.digits[i] {
                *digit += 10 - (borrow + rhs.digits[i]);
                borrow = 1;
            } else {
                *digit -= borrow + rhs.digits[i];
                borrow = 0;
            }
        }

        let mut i = rhs.len();
        while borrow != 0 {
            if digits[i] < borrow {
                digits[i] += 10 - borrow;
                borrow = 1;
            } else {
                digits[i] -= borrow;
                borrow = 0;
            }

            i += 1;
        }

        // Reduce
        while digits.len() > 1 && digits.last() == Some(&0) {
            digits.pop();
        }

        Self::Output { digits }
    }
}

impl Sub for BigNum {
    type Output = BigNum;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl Mul for &BigNum {
    type Output = BigNum;

    fn mul(self, rhs: Self) -> Self::Output {
        let n_orig = self.len() + rhs.len();
        let n = n_orig.next_power_of_two();

        let mut a = self.digits_to_complex();
        let mut b = rhs.digits_to_complex();
        a.resize(n, Default::default());
        b.resize(n, Default::default());

        let a = fft(&a);
        let b = fft(&b);

        let result: Vec<_> = a.into_iter().zip(b).map(|(a, b)| a * b).collect();
        let result = ifft(&result);
        let chunks: Vec<_> = result
            .into_iter()
            .map(|x| x.real.round() as usize)
            .collect();

        let mut digits = Vec::with_capacity(n_orig);
        let mut carry = 0;
        for chunk in chunks {
            let value = chunk + carry;
            digits.push(value % 10);
            carry = value / 10;
        }

        while carry > 0 {
            digits.push(carry % 10);
            carry /= 10;
        }

        // Reduce
        while digits.len() > 1 && digits.last() == Some(&0) {
            digits.pop();
        }

        Self::Output { digits }
    }
}

impl Mul for BigNum {
    type Output = BigNum;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl Div for &BigNum {
    type Output = BigNum;

    fn div(self, rhs: Self) -> Self::Output {
        if self.len() < rhs.len() {
            return BigNum::from(0);
        }

        if rhs.len() == 1 {
            let divisor = rhs.digits[0];
            let mut digits = vec![];
            let mut carry = 0;
            for digit in self.digits.iter().rev() {
                digits.push((digit + carry) / divisor);
                carry = 10 * ((digit + carry) % divisor);
            }

            // Reverse
            digits.reverse();
            // Reduce
            while digits.len() > 1 && digits.last() == Some(&0) {
                digits.pop();
            }

            Self::Output { digits }
        } else {
            let mut quotient = vec![];
            let mut partial_dividend = BigNum { digits: vec![] };
            for &digit in self.digits.iter().rev() {
                partial_dividend.digits.insert(0, digit);
                if partial_dividend.len() < rhs.len() {
                    continue;
                }

                let mut q = 0;
                while &partial_dividend >= rhs {
                    partial_dividend = &partial_dividend - rhs;
                    q += 1;
                }

                quotient.push(q);
            }

            // Reverse
            quotient.reverse();
            // Reduce
            while quotient.len() > 1 && quotient.last() == Some(&0) {
                quotient.pop();
            }

            Self::Output { digits: quotient }
        }
    }
}

impl Div for BigNum {
    type Output = BigNum;

    fn div(self, rhs: Self) -> Self::Output {
        &self / &rhs
    }
}

impl Div<usize> for &BigNum {
    type Output = BigNum;

    fn div(self, rhs: usize) -> Self::Output {
        self / &BigNum::from(rhs)
    }
}

impl Div<usize> for BigNum {
    type Output = BigNum;

    fn div(self, rhs: usize) -> Self::Output {
        &self / rhs
    }
}

impl std::fmt::Debug for BigNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for BigNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num: String = self
            .digits
            .iter()
            .rev()
            .map(|&digit| (digit as u8 + b'0') as char)
            .collect();
        write!(f, "{}", num)
    }
}

impl BigNum {
    fn len(&self) -> usize {
        self.digits.len()
    }

    fn digits_to_complex(&self) -> Vec<Complex> {
        self.digits
            .iter()
            .map(|&x| Complex::new(x as f64, 0.))
            .collect()
    }

    fn sqrt(&self) -> Self {
        let guess = {
            let d = (self.len() + 1) / 2;

            let mut guess2 = vec![0; d];
            guess2[d - 1] = 2;
            let mut guess7 = vec![0; d];
            guess7[d - 1] = 7;

            let guess2 = BigNum { digits: guess2 };
            let guess7 = BigNum { digits: guess7 };

            let square2 = &guess2 * &guess2;
            let square7 = &guess7 * &guess7;

            if &square7 < self {
                BigNum { digits: vec![9; d] }
            } else if &square2 < self {
                guess7
            } else {
                guess2
            }
        };

        // Doing one iteration of Babylonian algorithm
        let guess = (self / &guess + guess) / 2;
        // let guess = (self / &guess + guess) / 2;
        // let guess = (self / &guess + guess) / 2;

        // Then searching with binary search
        let mut lo = BigNum::from(0);
        let mut hi = guess;

        while lo <= hi {
            let mid = (&lo + &hi) / 2;
            let next = &mid + 1;

            let sq1 = &mid * &mid;
            let sq2 = &next * &next;
            if &sq1 <= self && &sq2 > self {
                return mid;
            } else {
                match sq1.cmp(self) {
                    Ordering::Less => lo = mid + 1,
                    Ordering::Greater => hi = mid,
                    _ => unreachable!(),
                }
            }
        }

        unreachable!()
    }
}

// dir =  1 -> forward
// dir = -1 -> backward
fn parametric_fft(input: &[Complex], dir: isize) -> Vec<Complex> {
    fn fft_inner(buf_a: &mut [Complex], buf_b: &mut [Complex], n: usize, step: usize, dir: f64) {
        if step >= n {
            return;
        }

        // Recursively apply FFT
        fft_inner(buf_b, buf_a, n, step * 2, dir);
        fft_inner(&mut buf_b[step..], &mut buf_a[step..], n, step * 2, dir);
        let (left, right) = buf_a.split_at_mut(n / 2);

        for i in (0..n).step_by(step * 2) {
            const I: Complex = Complex { real: 0., imag: 1. };
            let t =
                (-I * std::f64::consts::PI * dir * (i as f64) / (n as f64)).exp() * buf_b[i + step];
            left[i / 2] = buf_b[i] + t;
            right[i / 2] = buf_b[i] - t;
        }
    }

    // round n (length) up to a power of 2:
    let n_orig = input.len();
    let n = n_orig.next_power_of_two();

    // copy the input into a buffer:
    let mut buf_a = input.to_vec();
    // right pad with zeros to a power of two:
    buf_a.resize(n, Complex::default());

    // alternate between buf_a and buf_b to avoid allocating a new vector each time:
    let mut buf_b = buf_a.clone();
    fft_inner(&mut buf_a, &mut buf_b, n, 1, dir as f64);

    if dir == -1 {
        let length = buf_a.len() as f64;
        buf_a.iter_mut().for_each(|v| *v = *v / length);
    }

    buf_a
}

fn fft(input: &[Complex]) -> Vec<Complex> {
    parametric_fft(input, 1)
}

fn ifft(input: &[Complex]) -> Vec<Complex> {
    parametric_fft(input, -1)
}
