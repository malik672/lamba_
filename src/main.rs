use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        short_weierstrass::curves::bls12_381::curve::BLS12381Curve,
        traits::IsEllipticCurve
    }, traits::AsBytes
};
use hex;

fn main() {
    const PRIVATE_KEY: usize = 0x6C616D6264617370;
    let generated_point = BLS12381Curve::generator();
    let public_key = generated_point.operate_with_self(PRIVATE_KEY).as_bytes();
    println!("{:?}", hex::encode(public_key));
}
