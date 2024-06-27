mod prover;
mod verifier;

use ark_ff::Field;
use ark_std::marker::PhantomData;

pub struct IPForSumCheck<F: Field>{
    #[doc(hidden)]
    _marker: PhantomData<F>,
}


#[cfg(test)]
mod tests {
    use super::*;
    use ark_poly::multivariate::SparsePolynomial;
    use ark_poly::DenseMVPolynomial;
    use ark_std::test_rng;
    use ark_bls12_381::Fr;


    #[test]
    fn test_protocol() {
        let mut rng = test_rng();

        // construct a l-variate polynomial which is the sum of l d-degreee univariate polynomials
        // where each coefficient is sampled uniformly at random.
        let d = 1;
        let l = 10;
        let polynomial = SparsePolynomial::rand(d, l.clone(), &mut rng);

        let mut prover_state = IPForSumCheck::prover_init(&polynomial);
        let claimed_value: Fr = IPForSumCheck::run_init_prover_round(&mut prover_state);
        let mut verifier_state = IPForSumCheck::verifier_init(&polynomial, claimed_value);
        let mut verifier_msg = None;
        for _ in 0..l {
            let prover_msg = IPForSumCheck::prove_round(&mut prover_state, &verifier_msg);
            let verifier_msg2 = IPForSumCheck::verify_round(&mut verifier_state, &prover_msg.uvpolynomial, &mut rng);
            match verifier_msg2 {
                Ok(msg) => {
                    verifier_msg = Some(msg);
                },
                Err(e) => {
                    panic!("Verification Failed: {}", e);
                }
            }
            
        }
        assert!(IPForSumCheck::verify_last_round(&mut verifier_state, &polynomial).is_ok());
    }

    
}
