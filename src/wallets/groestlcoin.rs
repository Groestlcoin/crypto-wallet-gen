use anyhow::Result;

use super::Wallet;
use crate::bip32::HDPrivKey;

pub struct GroestlcoinWallet {
    private_key: HDPrivKey,
}

impl GroestlcoinWallet {
    pub fn private_key(&self) -> String {
        self.private_key.to_base58()
    }
}

impl Wallet for GroestlcoinWallet {
    fn from_hd_key(private_key: HDPrivKey) -> Result<Self> {
        Ok(Self { private_key })
    }
}
