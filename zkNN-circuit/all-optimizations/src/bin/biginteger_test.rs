use algebra::ed_on_bls12_381::*;
// use algebra::CanonicalSerialize;
// use algebra::UniformRand;
// use algebra::biginteger::BigInteger;
// use algebra_core::Zero;
// use algebra_core::Field;
// use rand::SeedableRng;
// use rand_xorshift::XorShiftRng;
// use r1cs_core::*;

use algebra::biginteger::BigInteger256 as BigInteger;

fn main() {

   let biginteger_one = BigInteger::from(1u64);
   let fq_one:Fq = 1u32.into();
   let fr_one:Fr = 1u32.into();

   let R2: BigInteger = BigInteger([
    0x67719aa495e57731,
    0x51b0cef09ce3fc26,
    0x69dab7fac026e9a5,
    0x4f6547b8d127688,
    ]);  //R^2 = 2^512 mod r

    println!("biginteger_one:{:?}",biginteger_one);
    println!("fq_one:{:?}",fq_one);
    println!("fr_one:{:?}",fr_one);
    println!("R2:{:?}",R2);

    // fr = ([u64,0,0,0] * R2) mod r
    // Jubjub scalar field
    // This module provides an implementation of the Jubjub scalar field $\mathbb{F}_r$
    // where `r = 0x0e7db4ea6533afa906673b0101343b00a6682093ccc81082d0970e5ed6f72cb7`
}