// A module which contains necessary algorithms to compute Shamir's shares and recover secrets

use std::vec::Vec;

use rand::distributions::{Distribution, Uniform};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

use super::field::GF256;
use super::share::Share;

// Finds the [root of the Lagrange polynomial](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing#Computationally_efficient_approach).
// The expected `shares` argument format is the same as the output by the `get_evaluator´ function.
// Where each (key, value) pair corresponds to one share, where the key is the `x` and the value is a vector of `y`,
// where each element corresponds to one of the secret's byte chunks.
pub fn interpolate(shares: &[Share]) -> Vec<u8> {
    (0..shares[0].y.len())
        .map(|s| {
            shares
                .iter()
                .map(|s_i| {
                    shares
                        .iter()
                        .filter(|s_j| s_j.x != s_i.x)
                        .map(|s_j| s_j.x.clone() / (s_j.x.clone() - s_i.x.clone()))
                        .product::<GF256>()
                        * s_i.y[s].clone()
                })
                .sum::<GF256>()
                .0
        })
        .collect()
}

// Generates `k` polynomial coefficients, being the last one `s` and the others generated with a fixed seed between `[1, 255]`.
// Coefficient degrees go from higher to lower in the returned vector order.
pub fn random_polynomial(s: GF256, k: u8) -> Vec<GF256> {
    let k = k as usize;
    let mut poly = Vec::with_capacity(k);
    let between = Uniform::new_inclusive(1, 255);
    let mut rng = ChaCha8Rng::from_seed([0x42; 32]);

    for _ in 1..k {
        poly.push(GF256(between.sample(&mut rng)));
    }
    poly.push(s);

    poly
}

// Returns an iterator over the points of the `polys` polynomials passed as argument.
// Each item of the iterator is a tuple `(x, [f_1(x), f_2(x)..])` where eaxh `f_i` is the result for the ith polynomial.
// Each polynomial corresponds to one byte chunk of the original secret.
// The iterator will start at `x = 1` and end at `x = 255`.
pub fn get_evaluator(polys: Vec<Vec<GF256>>) -> impl Iterator<Item = Share> {
    (1..=u8::max_value()).map(GF256).map(move |x| Share {
        x: x.clone(),
        y: polys
            .iter()
            .map(|p| {
                p.iter()
                    .fold(GF256(0), |acc, c| acc * x.clone() + c.clone())
            })
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::{get_evaluator, interpolate, random_polynomial, GF256};
    use crate::share::Share;
    use std::{vec, vec::Vec};

    #[test]
    fn random_polynomial_works() {
        let poly = random_polynomial(GF256(1), 3);
        assert_eq!(poly.len(), 3);
        assert_eq!(poly[2], GF256(1));
    }

    #[test]
    fn evaluator_works() {
        let iter = get_evaluator(vec![vec![GF256(3), GF256(2), GF256(5)]]);
        let values: Vec<_> = iter.take(2).map(|s| (s.x.clone(), s.y.clone())).collect();
        assert_eq!(
            values,
            vec![(GF256(1), vec![GF256(4)]), (GF256(2), vec![GF256(13)])]
        );
    }

    #[test]
    fn interpolate_works() {
        let poly = random_polynomial(GF256(185), 10);
        let iter = get_evaluator(vec![poly]);
        let shares: Vec<Share> = iter.take(10).collect();
        let root = interpolate(&shares);
        assert_eq!(root, vec![185]);
    }
}
