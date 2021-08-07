// public key
// privkey > ED25519 => pubkey
//
// public addr = pubkey > base32 encoder

use blake2::digest::{Update, VariableOutput};
use blake2::VarBlake2b;
use hex;

/// local UUID used to reference a specific wallet
/// only useful in conjunction with a node's internal database file
pub struct WalletID(Vec<u8>);

/// Deterministically derived from Seed

#[derive(Debug)]
pub struct PrivateKey(Vec<u8>);

impl PartialEq for PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for PrivateKey {}

impl PrivateKey {
    fn to_pubkey(&self) -> PublicKey {
        // ED25519(self.0.to_owned()) -> PublicKey
        todo!();
    }
}

/// Derived from PrivateKey using ED25519
/// 32bytes, 64 upper-hex
pub struct PublicKey(Vec<u8>);

pub trait ToPrivKey {
    fn to_key(&self, index: u32) -> PrivateKey;
}

pub trait ToMnemonic {
    fn to_mnemonic() {
        todo!();
    }
}

/// Series of 32 random bytes of data, represented as a 64 char, uppercase-hex string (0-9A-F)
/// Used to derive PrivateKey(s) by combining with an index, the result is then hashed with blake2b.
///
/// # Notes
/// - Alternative implementation using BIP44 deterministic wallets and mnemonic seeds
///
pub struct Seed(Vec<u8>);

impl Seed {
    pub fn new(seed: &str) -> Self {
        Seed(hex::decode(seed).expect("Error parsing hex string"))
    }
}

impl ToPrivKey for Seed {
    fn to_key(&self, index: u32) -> PrivateKey {
        // PrivK[i] = blake2b(outLen=32, input=(seed.concat(i)), i : u32_big-endian
        let mut input: [u8; 32 + 4] = [0; 36];
        for (dst, src) in input.iter_mut().zip(&*self.0) {
            *dst = *src;
        }
        for (dst, src) in input[32..].iter_mut().zip(index.to_be_bytes()) {
            *dst = src;
        }

        let mut hasher = VarBlake2b::new(32).unwrap(); // 32 byte output
        hasher.update(input);
        let output = hasher.finalize_boxed();
        PrivateKey(output.to_vec())
    }
}

//Privk[i] > bip39(`44'/165'/i'`) => mnemonic
//format!({seed}{i}) > blake2b(32) > bip39 => mnemonic
//
//format!({seed}{i}) > bip44 => mnemonic

#[cfg(test)]
mod tests {
    use super::*;
    const hex1: &str = "51c1df03bdae52b02268e973cb13ebcd";
    const hex2: &str = "d91d04cca0545768190e547cb8dadd7a";
    const hex3: &str = "6f4b1faa9ec2f4e41edfc3a97e5e0103";
    const hex4: &str = "70d6aa8bdd75b91fb8508e59f1ee5992";
    const hex5: &str = "027ef22aa0f809e4845897d7144c9ed1";

    #[test]
    fn same_seed_same_index() {
        let seed1 = Seed::new(hex1);
        let hash1 = seed1.to_key(1);

        let seed2 = Seed::new(hex1);
        let hash2 = seed2.to_key(1);

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn same_seed_diff_index() {
        let seed1 = Seed::new(hex1);
        let hash1 = seed1.to_key(1);

        let seed2 = Seed::new(hex1);
        let hash2 = seed2.to_key(2);

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn diff_seed_same_index() {
        let seed1 = Seed::new(hex1);
        let hash1 = seed1.to_key(1);

        let seed2 = Seed::new(hex2);
        let hash2 = seed2.to_key(1);

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn diff_seed_diff_index() {
        let seed1 = Seed::new(hex1);
        let hash1 = seed1.to_key(1);

        let seed2 = Seed::new(hex2);
        let hash2 = seed2.to_key(2);

        assert_ne!(hash1, hash2);
    }
}
