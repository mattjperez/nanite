// public key
// privkey > ED25519 => pubkey
//
// public addr = pubkey > base32 encoder

use blake2b::digest::{Update, VariableOutput};
use blake2b::VarBlake2;

/// local UUID used to reference a specific wallet
/// only useful in conjunction with a node's internal database file
pub struct WalletID(Vec<u8>);

/// Deterministically derived from Seed
#[derive(Debug)]
pub struct PrivateKey(Vec<u8>);

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

/// Series of 32 random bytes of data, represented as a 64 char, uppercase-hex string (0-9A-F)
/// Used to derive PrivateKey(s) by combining with an index, the result is then hashed with blake2b.
///
/// # Notes
/// - Alternative implementation using BIP44 deterministic wallets and mnemonic seeds
///
pub struct Seed(Vec<u8>);

impl Seed {
    pub fn new() -> Self {
        Seed(Vec::<u8>::new())
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
        let output = Params::new()
            .hash_length(32)
            .hash(&input)
            .as_array()
            .to_vec();
        PrivateKey(output)
    }
}

//Privk[i] > bip39(`44'/165'/i'`) => mnemonic
//format!({seed}{i}) > blake2b(32) > bip39 => mnemonic
//
//format!({seed}{i}) > bip44 => mnemonic
