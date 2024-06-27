use ark_poly::{univariate::DensePolynomial, multivariate::{SparsePolynomial, SparseTerm}};
use ark_poly::{Polynomial, DenseMVPolynomial};
use ark_ff::Field;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use ark_std::rand::RngCore;
use crate::IPForSumCheck;

#[derive(Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct VerifierMsg<F: Field> {
    pub randomn_value: F,
}

pub struct VerifierState<F: Field> {
    /// univariate polynomials sent by the prover at each round
    last_polynomial: Option<DensePolynomial<F>>,
    claimed_value: F,
    /// randomness sampled by the verifier at each round
    randomness: Vec<F>,
    round: usize,
}

impl<F: Field> IPForSumCheck<F> {
    pub fn verifier_init(
        polynomial:&SparsePolynomial<F, SparseTerm>, 
        claimed_value: F
    ) -> VerifierState<F> {
        VerifierState {
            last_polynomial: None,
            claimed_value: claimed_value,
            randomness: Vec::with_capacity(polynomial.num_vars()),
            round: 0,
        }
    }

    pub fn verify_round<R: RngCore>(
        verifier_state: &mut VerifierState<F>, 
        polynomial:&DensePolynomial<F>, 
        rng:&mut R
    ) -> Result<VerifierMsg<F>, &'static str> {
        if verifier_state.round == 0 {
            // check C1 = g1(0) +g1(1),
            let eval = polynomial.evaluate(&F::zero()) + polynomial.evaluate(&F::one());
            if eval !=  verifier_state.claimed_value {
                return Err("First Evaluation Check Failed");
            }
        } else {
            // check gj−1(rj−1) = gj(0) +gj(1)
            let last_polynomial = verifier_state.last_polynomial.clone().unwrap();
            let eval = polynomial.evaluate(&F::zero()) + polynomial.evaluate(&F::one());
            if eval != last_polynomial.evaluate(&verifier_state.randomness[verifier_state.round - 1]) {
                return Err("Round Evaluation Check Failed");
            }
        }
        let r = F::rand(rng);
        verifier_state.last_polynomial = Some(polynomial.clone());
        verifier_state.randomness.push(r.clone());
        verifier_state.round += 1;

        Ok(VerifierMsg {
            randomn_value: r,
        })
    }
    pub fn verify_last_round(
        verifier_state: &mut VerifierState<F>,
        polynomial:&SparsePolynomial<F, SparseTerm>
    ) -> Result<(), &'static str> {
        // check gv(rv) = g(r1,...,rv),
        let rv = verifier_state.randomness[verifier_state.round - 1].clone();
        let expected = polynomial.evaluate(&verifier_state.randomness);
        let eval = verifier_state.last_polynomial.clone().unwrap().evaluate(&rv);
        if eval == expected {
            Ok(())
        } else {
            Err("Final Evaluation Check Failed")
        }
    }
}

