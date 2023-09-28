#![cfg(feature = "test-bpf")]

use mpl_project_name::{
    instruction::{
        builders::{CreateBuilder, InstructionBuilder},
        CreateArgs,
    },
    state::MyAccount,
};

use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

macro_rules! account {
    ($type:ty, $account_var:ident) => {{
        <$type>::from_bytes($account_var.data.as_mut())
    }};
}

#[tokio::test]
async fn create() {
    let mut context = ProgramTest::new("mpl_project_name", mpl_project_name::ID, None)
        .start_with_context()
        .await;

    let alice = 1;
    let bob = [2u8; 32];

    let address = Keypair::new();
    let create_args = CreateArgs { alice, bob };

    let ix = CreateBuilder::new()
        .address(address.pubkey())
        .authority(context.payer.pubkey())
        .payer(context.payer.pubkey())
        .build(create_args)
        .unwrap()
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &address],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    let account = context
        .banks_client
        .get_account(address.pubkey())
        .await
        .unwrap();

    assert!(account.is_some());

    let mut account = account.unwrap();
    assert_eq!(account.data.len(), MyAccount::LEN);

    let my_account = account!(MyAccount, account);

    assert_eq!(my_account.data.alice, alice);
    assert_eq!(my_account.data.bob, bob);
}
