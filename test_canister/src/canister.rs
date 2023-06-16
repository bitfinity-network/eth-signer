use candid::Principal;
use eth_signer::ic_sign::{IcSigner, SigningKeyId};
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{TransactionRequest, H160};
use ic_canister::{generate_idl, update, Canister, Idl, PreUpdate};
use ic_exports::ic_ic00_types::DerivationPath;

#[derive(Canister)]
pub struct TestCanister {
    #[id]
    id: Principal,
}

impl PreUpdate for TestCanister {}

impl TestCanister {
    #[update]
    pub async fn sign_and_check(&self) {
        let pubkey = IcSigner
            .public_key(SigningKeyId::Dfx, DerivationPath::new(vec![]))
            .await
            .unwrap();
        let from = IcSigner.pubkey_to_address(&pubkey).unwrap();
        let tx: TypedTransaction = TransactionRequest::new()
            .from(from)
            .to(H160::zero())
            .value(10)
            .chain_id(355113)
            .nonce(0)
            .gas_price(10)
            .gas(53000)
            .into();

        let signature = IcSigner
            .sign_transaction(&tx, SigningKeyId::Dfx, DerivationPath::new(vec![]))
            .await
            .unwrap();

        let recovered_from = signature.recover(tx.sighash()).unwrap();
        assert_eq!(recovered_from, from);
    }

    /// Important: This function must be added to the canister to provide the idl.
    pub fn idl() -> Idl {
        generate_idl!()
    }
}
