use num::traits::Pow;
use num::{FromPrimitive, Num, Zero};
use std::collections::HashMap;
use std::iter::repeat_with;

pub struct Polynomial<T: Num + Clone> {
    coefficients: Vec<T>,
}

impl<T: Num + Clone> Polynomial<T> {
    pub fn new(coefficients: Vec<T>) -> Self {
        let mut coeffs = coefficients.clone();
        coeffs.reverse();
        while let Some(coeff) = coeffs.last() {
            if coeff.is_zero() {
                coeffs.pop();
            } else {
                break;
            }
        }
        coeffs.reverse();
        Polynomial {
            coefficients: coeffs,
        }
    }

    pub fn zero() -> Self {
        Polynomial {
            coefficients: vec![T::zero()],
        }
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    pub fn leading_coefficient(&self) -> &T {
        self.coefficients.last().unwrap()
    }

    pub fn add(&self, other: &Self) -> Self {
        let max_len = std::cmp::max(self.coefficients.len(), other.coefficients.len());
        let sum_coeffs = repeat_with(T::zero)
            .take(max_len)
            .zip(self.coefficients.iter().chain(repeat_with(T::zero)))
            .zip(other.coefficients.iter().chain(repeat_with(T::zero)))
            .map(|((_, a), b)| a.clone() + b.clone())
            .collect();
        Polynomial::new(sum_coeffs)
    }

    pub fn mul(&self, other: &Self) -> Self {
        let coeffs_len = self.coefficients.len() + other.coefficients.len() - 1;
        let mut new_coeffs = vec![T::zero(); coeffs_len];

        for (i, a) in self.coefficients.iter().enumerate() {
            for (j, b) in other.coefficients.iter().enumerate() {
                new_coeffs[i + j] = new_coeffs[i + j].clone() + a.clone() * b.clone();
            }
        }

        Polynomial::new(new_coeffs)
    }

    pub fn divmod(&self, divisor: &Self) -> (Self, Self) {
        let mut quotient = Self::zero();
        let mut remainder = self.clone();

        while remainder.degree() >= divisor.degree() {
            let monomial_exponent = remainder.degree() - divisor.degree();
            let monomial_coefficient =
                remainder.leading_coefficient().clone() / divisor.leading_coefficient().clone();
            let monomial_coeffs = repeat_with(T::zero)
                .take(monomial_exponent)
                .chain(std::iter::once(monomial_coefficient))
                .collect();
            let monomial = Polynomial::new(monomial_coeffs);

            quotient = quotient.add(&monomial);
            remainder = remainder.add(&monomial.mul(&divisor).neg());
        }

        (quotient, remainder)
    }

    pub fn eval_at(&self, x: T) -> T {
        let mut y = self.leading_coefficient().clone();
        for coeff in self.coefficients.iter().rev().skip(1) {
            y = y * x.clone() + coeff.clone();
        }
        y
    }

    pub fn interpolate(xs: &Vec<T>, ys: &Vec<T>) -> Self {
        let mut f = Self::zero();
        let n = xs.len();

        for i in 0..n {
            let xi = xs[i].clone();
            let yi = ys[i].clone();

            let mut num = Self::new(vec![T::one()]);
            let mut den = T::one();
            for j in 0..n {
                if i != j {
                    let xj = xs[j].clone();
                    num = num.mul(&Self::new(vec![-xj.clone(), T::one()]));
                    den = den.clone() * (xi.clone() - xj.clone());
                }
            }

            f = f.add(&num.mul_scalar(&yi / den));
        }
        f
    }

    pub fn neg(&self) -> Self {
        Polynomial::new(self.coefficients.iter().map(|c| c.clone().neg()).collect())
    }

    pub fn mul_scalar(&self, scalar: &T) -> Self {
        Polynomial::new(
            self.coefficients
                .iter()
                .map(|c| c.clone() * scalar.clone())
                .collect(),
        )
    }

    pub fn clone(&self) -> Self {
        Polynomial::new(self.coefficients.clone())
    }
}

impl<T: Num + Clone + std::fmt::Display> std::fmt::Display for Polynomial<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.coefficients.is_empty() {
            write!(f, "0")
        } else {
            let terms: Vec<String> = self
                .coefficients
                .iter()
                .enumerate()
                .filter(|(_, c)| !c.is_zero())
                .map(|(i, c)| format!("{}x^{}", c, i))
                .collect();
            write!(f, "{}", terms.join(" + "))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_polynomial() -> Polynomial<BigRational> {
        let coefficients = vec![
            BigRational::new(BigInt::from(1), BigInt::from(1)),
            BigRational::new(BigInt::from(2), BigInt::from(1)),
            BigRational::new(BigInt::from(3), BigInt::from(1)),
        ];
        Polynomial::new(coefficients)
    }
}
