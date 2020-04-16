#![feature(stdsimd)]
use faster::*;

/// Accurate floating point summation.
///
/// This uses [Kahan Algorithm](https://en.wikipedia.org/wiki/Kahan_summation_algorithm).
pub fn sum(values: &[f64]) -> f64 {
    let mut s: f64 = 0.0;
    let mut c: f64 = 0.0;

    for v in values {
        let y = v - c;
        let t = s + y;
        c = (t - s) - y;
        s = t;
    }

    s
}

/// Accurate floating point summation.
///
/// This implementations uses SIMD instruction to speed things up.
pub fn sum_simd(values: &[f64]) -> f64 {
    let mut c = f64s(0.0);

    values.simd_iter(f64s(0.0))
        .simd_reduce(f64s(0.0), |s, v| {
            let y = v - c;
            let t = s + y;
            c = (t - s) - y;
            let new_s = t;

            new_s
        })
        .scalar_reduce(0.0, |acc, s| acc + s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum(&vec![10000.0, 3.14159, 2.71828]);
        assert_eq!(result, 10005.85987,);
    }

    #[test]
    fn simd_works() {
        let result = sum_simd(&vec![10000.0, 3.14159, 2.71828]);
        assert_eq!(result, 10005.85987,);
    }
}
