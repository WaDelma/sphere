extern crate num;
use num::Float;

use std::f64::consts::PI;

/// Calculates the volume of the n-sphere.
pub fn sphere_volume<F: Float>(radius: F, dim: u64) -> F {
    let pi = F::from(PI)
        .expect("Casting PI should be valid.");
    let gamma = if dim % 2 == 0 {
        // This works because dim / 2 is a whole number.
        F::from(fact(dim / 2))
            .expect("Casting result of factorial should be always castable as Float.")
    } else {
        // This works because the function is gamma(1/2 + n) where n = dim / 2 + 1
        let n = dim / 2 + 1;
        let fact_2n = F::from(fact(2 * n))
            .expect("Casting result of factorial should be always castable as Float.");
        let div = F::from(4u64.pow(n as u32) * fact(n))
            .expect("Casting result of factorial should be always castable as Float.");
        (fact_2n / div) * pi.sqrt()
    };
    let dim = F::from(dim)
        .expect("u64 should be always castable as Float.");
    let half = F::one() / (F::one() + F::one());
    (pi.powf(half * dim) * radius.powf(dim)) / gamma
}

#[test]
fn volume_of_1_sphere_works() {
    for radius in (0..5000000).map(|n| n as f64 / 7000.) {
        let expected = PI * radius.powi(2);
        let actual = sphere_volume(radius, 2);
        assert!(equals(expected, actual, 0.0001, 5), "Expected: {}, Actual: {}", expected, actual);
    }
}

#[test]
fn volume_of_2_sphere_works() {
    for radius in (0..5000000).map(|n| n as f64 / 7000.) {
        let expected = (4. / 3.) * PI * radius.powi(3);
        let actual = sphere_volume(radius, 3);
        assert!(equals(expected, actual, 0.0001, 5), "Expected: {}, Actual: {}", expected, actual);
    }
}

#[test]
fn volume_of_3_sphere_works() {
    for radius in (0..5000000).map(|n| n as f64 / 7000.) {
        let expected = (1. / 2.) * PI.powi(2) * radius.powi(4);
        let actual = sphere_volume(radius, 4);
        assert!(equals(expected, actual, 0.0001, 5), "Expected: {}, Actual: {}", expected, actual);
    }
}

#[cfg(test)]
fn equals(a: f64, b: f64, max_diff: f64, max_ulps_diff: i64) -> bool {
    let diff = f64::abs(a - b);
    if diff <= max_diff {
        return true;
    }
    if a.is_sign_positive() && !b.is_sign_positive() {
        return false;
    }
    let a_i64 = unsafe{::std::mem::transmute::<_, i64>(a)};
    let b_i64 = unsafe{::std::mem::transmute::<_, i64>(b)};
    let ulps_diff = i64::abs(a_i64 - b_i64);
    return ulps_diff <= max_ulps_diff;
}

#[inline]
fn fact(n: u64) -> u64 {
    //TODO: This could be made faster with better algorithm, but it's called with low values so it doesn't matter much.
    if n < 2 {
        1
    } else {
        n * fact(n - 1)
    }
}

#[test]
fn factorial_works() {
    assert_eq!(1, fact(0));
    let mut x = 1;
    for n in 1..21 {
        x *= n;
        assert_eq!(x, fact(n));
    }
}
