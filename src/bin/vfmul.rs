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
        let (a, b) = line.split_once(' ').unwrap();
        let (a, b) = (BigNum::from(a), BigNum::from(b));
        println!("{}", a * b);
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
        const CHUNK_SIZE: usize = 5;
        const CHUNK_BASE: usize = 10usize.pow(CHUNK_SIZE as u32);

        let mut a = self.digits_to_complex(CHUNK_SIZE);
        let mut b = rhs.digits_to_complex(CHUNK_SIZE);

        let n_orig = a.len() + b.len();
        let n = n_orig.next_power_of_two();

        a.resize(n, Default::default());
        b.resize(n, Default::default());

        let a = fft(&a);
        let b = fft(&b);

        let result: Vec<_> = a.into_iter().zip(b).map(|(a, b)| a * b).collect();
        let result = ifft(&result);

        let mut chunks: Vec<_> = result
            .into_iter()
            .map(|x| x.real.round() as usize)
            .collect();
        while chunks.len() > 1 && chunks.last() == Some(&0) {
            chunks.pop();
        }

        let mut length = chunks.len();
        let mut i = 0;
        while i < length {
            if chunks[i] >= CHUNK_BASE {
                let carry = chunks[i] / CHUNK_BASE;

                if i == length - 1 {
                    chunks.push(carry);
                    length += 1;
                } else {
                    chunks[i + 1] += carry;
                }

                chunks[i] %= CHUNK_BASE;
            }

            i += 1;
        }

        let mut digits = vec![];
        for (i, mut chunk) in chunks.into_iter().enumerate() {
            if i < length - 1 {
                for _ in 0..CHUNK_SIZE {
                    digits.push(chunk % 10);
                    chunk /= 10;
                }
            } else {
                while chunk > 0 {
                    digits.push(chunk % 10);
                    chunk /= 10;
                }
            }
        }

        if digits.is_empty() {
            digits.push(0);
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

    fn digits_to_complex(&self, chunk_size: usize) -> Vec<Complex> {
        let mut chunks = Vec::with_capacity((self.len() + chunk_size - 1) / chunk_size);
        let mut i = 0;
        while i < self.len() {
            let mut chunk = 0;
            let max_length = chunk_size.min(self.len() - i);

            for j in (0..max_length).rev() {
                if i + j >= self.len() {
                    break;
                }

                chunk = chunk * 10 + self.digits[i + j];
            }

            chunks.push(Complex::new(chunk as f64, 0.));
            i += chunk_size;
        }

        chunks
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
