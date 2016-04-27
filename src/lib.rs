extern crate num;
use num::Float;

const TAU: f64 = 2. * std::f64::consts::PI;
const LAMBDA: f64 = TAU / 4.;

/// Calculates the volume of the n-sphere.
pub fn sphere_volume<F: Float>(radius: F, n: u64) -> F {
    let pieces = cast::<F>(2.).powi(n as i32);
    let half_n = (n as f64 / 2.).floor() as i32;
    let lambda_power = cast::<F>(LAMBDA).powi(half_n);
    let double_fact = F::from(double_fact(n))
        .expect("Used too high dimensions for sphere.");
    let piece_volume = (lambda_power * radius.powi(n as i32)) / double_fact;
    F::from(pieces * piece_volume).unwrap()
}

/// Calulates the surface area of the n-sphere.
pub fn sphere_surface<F: Float>(radius: F, n: u64) -> F {
    let pieces = cast::<F>(2.).powi(n as i32);
    let half_n = (n as f64 / 2.).floor() as i32;
    let lambda_power = cast::<F>(LAMBDA).powi(half_n);
    let double_fact = F::from(double_fact(n - 2))
        .expect("Used too high dimensions for sphere.");
    let piece_volume = (lambda_power * radius.powi(n as i32 - 1)) / double_fact;
    F::from(pieces * piece_volume).unwrap()
}

#[inline]
fn cast<F: Float>(f: f64) -> F {
    F::from(f).expect("Trusted cast should succeed.")
}

#[test]
fn volume_of_1_sphere_works() {
    for radius in (0..5000000).map(|n| n as f64 / 7000.) {
        let expected = (1. / 2.) * TAU * radius.powi(2);
        let actual = sphere_volume(radius, 2);
        assert!(equals(expected, actual, 0.0001, 5), "Expected: {}, Actual: {}", expected, actual);
    }
}

#[test]
fn volume_of_2_sphere_works() {
    for radius in (0..5000000).map(|n| n as f64 / 7000.) {
        let expected = (4. / 6.) * TAU * radius.powi(3);
        let actual = sphere_volume(radius, 3);
        assert!(equals(expected, actual, 0.0001, 5), "Expected: {}, Actual: {}", expected, actual);
    }
}

#[test]
fn volume_of_3_sphere_works() {
    for radius in (0..5000000).map(|n| n as f64 / 7000.) {
        let expected = (1. / 8.) * TAU.powi(2) * radius.powi(4);
        let actual = sphere_volume(radius, 4);
        assert!(equals(expected, actual, 0.0001, 5), "Expected: {}, Actual: {}", expected, actual);
    }
}

#[test]
fn surface_of_1_sphere_works() {
    for radius in (0..5000000).map(|n| n as f64 / 7000.) {
        let expected = TAU * radius;
        let actual = sphere_surface(radius, 2);
        assert!(equals(expected, actual, 0.0001, 5), "Expected: {}, Actual: {}", expected, actual);
    }
}

#[test]
fn surface_of_2_sphere_works() {
    for radius in (0..5000000).map(|n| n as f64 / 7000.) {
        let expected = 2. * TAU * radius.powi(2);
        let actual = sphere_surface(radius, 3);
        assert!(equals(expected, actual, 0.0001, 5), "Expected: {}, Actual: {}", expected, actual);
    }
}

#[test]
fn surface_of_3_sphere_works() {
    for radius in (0..5000000).map(|n| n as f64 / 7000.) {
        let expected = (1. / 2.) * TAU.powi(2) * radius.powi(3);
        let actual = sphere_surface(radius, 4);
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
fn double_fact(n: u64) -> u64 {
    if n < 2 {
        1
    } else {
        n * double_fact(n - 2)
    }
}

#[test]
fn double_factorial_works() {
    assert_eq!(1, double_fact(0));
    let mut x = 1;
    for n in 1..17 {
        let n = 2 * n;
        x *= n;
        assert_eq!(x, double_fact(n));
    }
    let mut x = 1;
    for n in 0..16 {
        let n = 2 * n + 1;
        x *= n;
        assert_eq!(x, double_fact(n));
    }
}
