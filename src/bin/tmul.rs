use std::{
    io::BufRead,
    ops::{Add, Div, Mul, Neg, Sub},
};

/**
 * MUL - Fast Multiplication
 *
 * Multiply the given numbers.
 *
 * Input
 * n [the number of multiplications <= 1000]
 * l1 l2 [numbers to multiply (at most 10000 decimal digits each)]
 * Text grouped in [ ] does not appear in the input file.
 *
 * Output
 * The results of multiplications.
 *
 * Example
 *   Input:
 *   5
 *   4 2
 *   123 43
 *   324 342
 *   0 12
 *   9999 12345
 *   
 *   Output:
 *   8
 *   5289
 *   110808
 *   0
 *   123437655
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(n) {
        let line = line.unwrap();
        let (a, b) = line.split_once(' ').unwrap();
        let result = multiply(a, b);
        println!("{}", result);
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

fn multiply_large_integers(a: &[usize], b: &[usize]) -> Vec<usize> {
    let n_orig = a.len() + b.len();
    let n = n_orig.next_power_of_two();

    let mut a: Vec<_> = a.iter().map(|&x| Complex::new(x as f64, 0.)).collect();
    let mut b: Vec<_> = b.iter().map(|&x| Complex::new(x as f64, 0.)).collect();
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

    let mut result = Vec::with_capacity(n_orig);
    let mut carry = 0;
    for chunk in chunks {
        let value = chunk + carry;
        result.push(value % 10);
        carry = value / 10;
    }

    while carry > 0 {
        result.push(carry % 10);
        carry /= 10;
    }

    // Reduce
    while result.len() > 1 && result.last() == Some(&0) {
        result.pop();
    }

    result
}

fn multiply(a: &str, b: &str) -> String {
    let a: Vec<_> = a.bytes().rev().map(|v| (v - b'0') as usize).collect();
    let b: Vec<_> = b.bytes().rev().map(|v| (v - b'0') as usize).collect();

    let mut result = multiply_large_integers(&a, &b);
    result.reverse();

    result
        .into_iter()
        .map(|digit| (digit as u8 + b'0') as char)
        .collect()
}
