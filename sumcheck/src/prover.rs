use ark_poly::{univariate::DensePolynomial, multivariate::{SparsePolynomial, SparseTerm, Term}};
use ark_poly::{Polynomial, DenseUVPolynomial, DenseMVPolynomial};
use ark_std::cfg_into_iter;
use ark_ff::Field;
use crate::IPForSumCheck;
use crate::verifier::VerifierMsg;

pub struct ProverState<F: Field> {
    /// polynomial to be proved
    pub polynomial: SparsePolynomial<F, SparseTerm>,
    /// number of variables
    pub num_vars: usize,
    /// randomness given by the verifier at each round
    pub randomness: Vec<F>,
    /// current round number
    pub round: usize,
}
pub struct ProverMsg<F: Field> {
    pub uvpolynomial: DensePolynomial<F>,
}

impl<F: Field> IPForSumCheck<F> {
    pub fn prover_init(
        polynomial:&SparsePolynomial<F, SparseTerm>
    ) -> ProverState<F> {
        let num_vars = polynomial.num_vars();
        if num_vars == 0 {
            panic!("polynomial must have at least one variable");
        }
        ProverState {
            polynomial: polynomial.clone(),
            num_vars: num_vars,
            randomness: Vec::with_capacity(polynomial.num_vars()),
            round: 0,
        }
    }

    pub fn run_init_prover_round(
        prover_state: &mut ProverState<F>,
    ) -> F {
        // Evaluate the polynomial at all possible points
        let num_vars = prover_state.polynomial.num_vars();
    
        let mut accum = F::zero();
        for i in 0..2i32.pow(num_vars as u32) {
            let mut counter = i;
            let mut coeffs = Vec::with_capacity(num_vars);
            for _ in 0..num_vars {
                coeffs.push(if counter % 2 == 0 { F::zero()} else { F::one()});
                counter /= 2;
            }
            accum += prover_state.polynomial.evaluate(&coeffs);

        }

        accum
    }

    pub fn prove_round(
        prover_state: &mut ProverState<F>, 
        verifier_msg: &Option<VerifierMsg<F>>
    ) -> ProverMsg<F> {
        if let Some(msg) = verifier_msg {
            prover_state.randomness.push(msg.randomn_value);
        } 
        
        let to_sum = prover_state.num_vars - prover_state.round - 1;

        let mut coeffs = vec![F::zero(); prover_state.num_vars];
        for i in 0..2i32.pow(to_sum as u32) {
            let mut inputs = Vec::with_capacity(prover_state.num_vars);
            inputs.extend(prover_state.randomness.clone());
            inputs.push(F::zero());
            let mut counter = i;
            for _ in 0..to_sum{
                inputs.push(if counter % 2 == 0 { F::zero()} else { F::one()});
                counter /= 2;
            }

            // conribute to polynomial
            for (coeff, term) in cfg_into_iter!(&prover_state.polynomial.terms) {
                let mut coeff_accum: F = F::one();
                let mut which = 0;
                for (&var, pow) in term.vars().iter().zip(term.powers()) {
                    if var == prover_state.round {
                        which = pow;
                    } else {
                        coeff_accum *= inputs[var].pow(&[pow as u64]);
                    }
                }
                coeffs[which] += coeff.mul(&coeff_accum);
            }
        }
        prover_state.round += 1;
        
        ProverMsg {
            uvpolynomial: DensePolynomial::from_coefficients_vec(coeffs)
        }
    }

}

