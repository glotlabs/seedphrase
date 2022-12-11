use ethers::signers::coins_bip39::{English, MnemonicError, WordlistError};
use ethers::signers::{MnemonicBuilder, Signer, WalletError};
use serde::{Deserialize, Serialize};

pub fn to_address(mnemonic: &str) -> Result<String, Error> {
    let maybe_wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic)
        .build();

    match maybe_wallet {
        Ok(wallet) => {
            let address = wallet.address();
            Ok(format!("{:?}", address))
        }
        Err(err) => Err(Error::from_wallet_error(err)),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Error {
    InvalidPhrase,
    InvalidWordCount(usize),
    InvalidWord(String),
    Internal(String),
}

impl Error {
    fn from_wallet_error(wallet_error: WalletError) -> Error {
        match wallet_error {
            WalletError::Bip39Error(err) => {
                // fmt
                match err {
                    MnemonicError::InvalidPhrase(_) => {
                        // fmt
                        Error::InvalidPhrase
                    }

                    MnemonicError::InvalidWordCount(count) => {
                        // fmt
                        Error::InvalidWordCount(count)
                    }

                    MnemonicError::WordlistError(e) => {
                        // fmt
                        match e {
                            WordlistError::InvalidWord(word) => {
                                // fmt
                                Error::InvalidWord(word)
                            }

                            _ => {
                                // fmt
                                Error::Internal(e.to_string())
                            }
                        }
                    }

                    _ => {
                        // fmt
                        Error::Internal(err.to_string())
                    }
                }
            }
            _ => {
                // fmt
                Error::Internal(wallet_error.to_string())
            }
        }
    }
}
