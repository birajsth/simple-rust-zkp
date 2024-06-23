#![allow(dead_code)]

pub(crate) mod kzg;
pub(crate) mod utils;

#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::UniformRand;
    use ark_bls12_381::{Bls12_381, G1Projective as G1, G2Projective as G2, Fr};
    use kzg::KZG;
    use utils::evaluate;

    #[test]
    fn test_single_evaluation() {
        let mut rng = ark_std::test_rng();
        let degree = 16;

        // initialize a KZG instance
        let mut kzg_instance = KZG::<Bls12_381>::new(
            G1::rand(&mut rng),
            G2::rand(&mut rng),
            degree
        );
        
        // trusted setup ceremony
        let secret = Fr::rand(&mut rng);
        kzg_instance.setup(secret);

        // generate a random polynomial and commit it
        let poly = vec![Fr::rand(&mut rng); degree + 1];
        let commitment = kzg_instance.commit(&poly);

        // open the polynomial at random point
        let point = Fr::rand(&mut rng);
        let proof = kzg_instance.open(&poly, point);

        // verify the kzg proof
        let value = evaluate(&poly, point);
        assert!(kzg_instance.verify(point, value, commitment, proof));

        println!("Single point evaluation verified");
    }
}
