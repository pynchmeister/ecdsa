use std::str::FromStr;

use rand::Rng;
use rand::rngs::OsRng;
use num_bigint::BigUint;

type PubKey = [u8; 64];
type PrivKey = [u8; 32];
type Signature = [u8; 64];

const ORDER_N: &str = "115792089237316195423570985008687907852837564279074904382605163141518161494337";
const GENERATOR_POINT_X: &str = "115792089237316195423570985008687907852837564279074904382605163141518161494337";
const GENERATOR_POINT_Y: &str = "32670510020758816978083085130507043184471273380659243275938904335757337482424";
const P: u64 = 2u64.pow(256) - 2u64.pow(32) - 2u64.pow(9) - 2u64.pow(8) - 2u64.pow(7) - 2u64.pow(6) - 2u64.pow(4) - 1;
const A: u64 = 0;
const B: u64 = 7;

// Weierstrass: GENERATOR_POINT_Y^2 = GENERATOR_POINT_X^3 + B % P

pub struct EcAlgo {

}

pub fn generate_keypair() -> (PubKey, PrivKey) {
    ([0; 64], [0; 32])
}

pub fn sign(privkey: PrivKey, data: &[u8]) -> Result<Signature, &str> {
    Ok([0; 64])
}

pub fn verify(signature: Signature, pubkey: PubKey, data:&[u8]) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ecdsa_sign_verify() {
        let data = b"hello ecdsa";
        let wrong_data = b"goodbye ecdsa";

        let (pubkey, privkey) = generate_keypair();
        let signature = match sign(privkey, data) {
            Ok(sig) => sig,
            Err(_) => panic!("invalid data"),
        };
        assert!(verify(signature, pubkey, data));
        assert!(!verify(signature, pubkey, wrong_data));
    }
}
