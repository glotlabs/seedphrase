use ethers::signers::coins_bip39::English;
use ethers::signers::{MnemonicBuilder, Signer};

pub fn to_address(mnemonic: &str) -> String {
    let maybe_wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic)
        .build();

    match maybe_wallet {
        Ok(wallet) => {
            let address = wallet.address();
            format!("{:?}", address)
        }
        Err(err) => format!("{}", err),
    }
}
