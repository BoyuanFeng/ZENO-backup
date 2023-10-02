use algebra::ed_on_bls12_381::*;
use algebra::CanonicalSerialize;
use algebra::UniformRand;
use algebra_core::Zero;
use crypto_primitives::commitment::pedersen::Randomness;
use groth16::*;
use r1cs_core::*;
use std::time::Instant;
use zk_ml_knit_encoding::demo_circuit::*;
use zk_ml_knit_encoding::pedersen_commit::*;
use zk_ml_knit_encoding::read_inputs::*;
use zk_ml_knit_encoding::vanilla::*;
use rayon::prelude::*;

fn main() {
    rayon::ThreadPoolBuilder::new()
    .num_threads(1)
    .build_global()
    .unwrap();
    // let n0:Fq = 0u32.into();
    // let n1:Fq = 1u32.into();
    // let n2:Fq = 2u32.into();
    // let n3:Fq = 3u32.into();
    // let n4:Fq = 4u32.into();
    // let n5:Fq = 5u32.into();
    // let n6:Fq = 6u32.into();
    // let n7:Fq = 7u32.into();
    // let n8:Fq = 8u32.into();
    // let n9:Fq = 9u32.into();

    // let n0:Fr = 0u32.into();
    // let n1:Fr = 1u32.into();
    // let n2:Fr = 2u32.into();
    // let n3:Fr = 3u32.into();
    // let n4:Fr = 4u32.into();
    // let n5:Fr = 5u32.into();
    // let n6:Fr = 6u32.into();
    // let n7:Fr = 7u32.into();
    // let n8:Fr = 8u32.into();
    // let n9:Fr = 9u32.into();

    // let n0:Fq = 10u32.into();
    // let n1:Fq = 11u32.into();
    // let n2:Fq = 12u32.into();
    // let n3:Fq = 13u32.into();
    // let n4:Fq = 14u32.into();
    // let n5:Fq = 15u32.into();
    // let n6:Fq = 16u32.into();
    // let n7:Fq = 17u32.into();
    // let n8:Fq = 18u32.into();
    // let n9:Fq = 19u32.into();
    // println!("0:{:?}",n0);
    // println!("1:{:?}",n1);
    // println!("2:{:?}",n2);
    // println!("3:{:?}",n3);
    // println!("4:{:?}",n4);
    // println!("5:{:?}",n5);
    // println!("6:{:?}",n6);
    // println!("7:{:?}",n7);
    // println!("8:{:?}",n8);
    // println!("9:{:?}",n9);
    
    let mut rng = rand::thread_rng();
    let input:u8 = 1;
    let demo_circuit = DemoCircuit{x:input};
    let begin = Instant::now();
    let zk_param =
        generate_random_parameters::<algebra::Bls12_381, _, _>(demo_circuit.clone(), &mut rng)
            .unwrap();
    let end = Instant::now();
    println!("setup time {:?}", end.duration_since(begin));

    let pvk = prepare_verifying_key(&zk_param.vk);
    println!("random parameters generated!\n");

    // prover
    let begin = Instant::now();
    let proof = create_random_proof(demo_circuit, &zk_param, &mut rng).unwrap();
    let end = Instant::now();
    println!("prove time {:?}", end.duration_since(begin));

    // verifier
    let commitment = [].to_vec();

    let inputs: Vec<Fq> = [commitment[..].as_ref()].concat();
    //println!("len l1 {}\n len l2 {}", l1_mat_fq.len(), l2_mat_fq.len());
    let begin = Instant::now();
    assert!(verify_proof(&pvk, &proof, &inputs[..]).unwrap());
    let end = Instant::now();
    println!("verification time {:?}", end.duration_since(begin));
}
