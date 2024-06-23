use ark_ff::{Field, PrimeField};
use ark_std::log2;

// helper functio for polynomial addition
pub fn add<E:Field>(p1: &[E], p2: &[E]) -> Vec<E> {
    let mut result = vec![E::ZERO; std::cmp::max(p1.len(), p2.len())];

    for (i, &coeff) in p1.iter().enumerate() {
        result[i] += coeff;
    }
    for (i, &coeff) in p2.iter().enumerate() {
        result[i] += coeff;
    }

    result
}

// helper function for polynomial multiplication
pub fn mul<E:Field>(p1: &[E], p2: &[E]) -> Vec<E> {
    let mut result = vec![E::ZERO; p1.len() + p2.len() - 1];

    for (i, &coeff1) in p1.iter().enumerate() {
        for (j, &coeff2) in p2.iter().enumerate() {
            result[i + j] += coeff1 * coeff2;
        }
    }

    result
}

//  helper function for polynomial division
pub fn div<E:Field>(p1: &[E], p2: &[E]) -> Result<Vec<E>, &'static str>{
    if p2.is_empty() || p2.iter().all(|&x| x == E::ZERO) {
        return Err("Division by zero");
    }

    if p1.len() < p2.len() {
        return Ok(vec![E::ZERO]);
    }

    let mut quotient = vec![E::ZERO; p1.len() - p2.len() + 1];
    let mut remainder: Vec<E> = p1.to_vec();

    while remainder.len() >= p2.len() {
        let coeff = *remainder.last().unwrap() / *p2.last().unwrap();
        let pos = remainder.len() - p2.len();

        quotient[pos] = coeff;

        for (i, &factor) in p2.iter().enumerate() {
            remainder[pos + i] -= factor * coeff;
        }

        while let Some(true) = remainder.last().map(|x| *x == E::ZERO) {
            remainder.pop();
        }
    }

    Ok(quotient)
}

// helper function to evaluate polynomial at a point
pub fn evaluate<E:Field>(p: &[E], x: E) -> E {
    let mut result = E::ZERO;

    for (i, &coeff) in p.iter().enumerate() {
        result += coeff * x.pow(&[i as u64]);
    }

    result
}

//helper function to perform Largrange interpolation given a set of points
pub fn interpolate<E:Field>(points: &[E], values: &[E]) -> Result<Vec<E>, &'static str> {
    assert!(points.len() == values.len(), "Number of points and values must be equal");

    let mut result = vec![E::ZERO; points.len()];

    for i in 0..points.len() {
        let mut numerator = vec![E::ONE];
        let mut denominator = E::ONE;

        for j in 0..points.len() {
            if i != j {
                numerator = mul(&numerator, &[-points[j], E::ONE]);     
                denominator *= points[i] - points[j];
            }
        }
        let denominator_inv = denominator.inverse().unwrap();
        let term: Vec<E> = numerator.iter().map(|&x| x * values[i] * denominator_inv).collect();
        result = add(&result, &term);
    }

    Ok(result)
}

// helper function to get the roots of unity of a polynomial
pub fn get_omega<E:PrimeField>(coefficients: &[E]) -> E {
    let mut coefficients = coefficients.to_vec();
    let n = coefficients.len() - 1;
    if !n.is_power_of_two() {
        let num_coeffs = coefficients.len().checked_next_power_of_two().unwrap();
        // pad the coefficients with zeroes to the nearest power of two
        for i in coefficients.len()..num_coeffs {
            coefficients[i] = E::ZERO;
        }
    }
    
    let m = coefficients.len();
    let exp = log2(m);
    let mut omega = E::TWO_ADIC_ROOT_OF_UNITY;
    for _ in exp..E::TWO_ADICITY {
        omega.square_in_place();
    }
    omega
}

// helper function to multiply a polynomial with a scalar value
pub fn scalar_mul<E:Field>(poly: &[E], scalar:E) -> Vec<E> {
    let mut result = Vec::with_capacity(poly.len());
    for coeff in poly {
        result.push(*coeff * scalar);
    }    
    result
}   


