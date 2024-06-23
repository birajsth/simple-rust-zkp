pub mod kzg;
pub mod asvc;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use ark_std::UniformRand;
    use rand::seq::IteratorRandom;
    use ark_bls12_381::{Bls12_381, G1Projective as G1, G2Projective as G2, Fr};
    use kzg::KZG;
    use asvc::ASVC;
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

        // evaluate and verify the kzg proof
        let value = evaluate(&poly, point);
        assert!(kzg_instance.verify(point, value, commitment, proof));

        println!("Single point evaluation verified");
    }

    #[test]
    fn test_multi_evaluation() {
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

        // generate a random polynomial and commit to it
        let poly = vec![Fr::rand(&mut rng); degree + 1];
        let commitment = kzg_instance.commit(&poly);

        // open the polynomial at three random points
        let points: Vec<Fr>  = (0..3).map(|_| Fr::rand(&mut rng)).collect();
        let proof = kzg_instance.multi_open(&poly, &points);

        // evaluate and verify the kzg proof
        let mut values = vec![];
        for i in 0..points.len() {
            values.push(evaluate(&poly, points[i]));
        }
        assert!(kzg_instance.verify_multi(&points, &values, commitment, proof));

        println!("Multi point evaluation verified");
    }

    #[test]
    fn test_vector_evaluation() {
        let mut rng = ark_std::test_rng();
        let degree = 16;

        let secret = Fr::rand(&mut rng);

        // initialize a ASVC instance
        let asvc_instance = ASVC::<Bls12_381>::key_gen( 
            G1::rand(&mut rng),
            G2::rand(&mut rng),
            degree,
            secret
        );

        // generate a random vector and commit to it
        let vector = vec![Fr::rand(&mut rng); degree];
        let commitment = asvc_instance.vector_commit(&vector);

        // randomly select three items in the vector and also record their indices
        let mut selected_indices = Vec::new();
        while selected_indices.len() < 3 {
            let value = (0..=15).choose(&mut rng).unwrap();
            if !selected_indices.contains(&value) {
                selected_indices.push(value);
            } 
        }

        // prove positions for these three selected indices
        let pi = asvc_instance.prove_position(&selected_indices, &vector);

        // verify the proof
        let mut subvector = vec![];
        for &index in &selected_indices {
            subvector.push(vector[index]);
        }
        assert!(asvc_instance.verify_positon(commitment, &selected_indices, &subvector, pi));

        println!("Vector evaluation verified");

    }
}
