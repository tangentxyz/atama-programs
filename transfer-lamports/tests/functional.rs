use {
    solana_instruction::{AccountMeta, Instruction},
    solana_program_test::*,
    solana_pubkey::Pubkey,
    solana_sdk::{account::Account, signature::Signer, transaction::Transaction},
    std::str::FromStr,
};

#[tokio::test]
async fn test_lamport_transfer() {
    let program_id = Pubkey::from_str("TransferLamports111111111111111111111111111").unwrap();
    let source_pubkey = Pubkey::new_unique();
    let destination_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        option_env!("PROGRAM_NAME").unwrap_or("solana_program_rosetta_transfer_lamports"),
        program_id,
        None,
    );

    let source_lamports = 555_555;
    let destination_lamports = 890_875;
    program_test.add_account(
        source_pubkey,
        Account {
            lamports: source_lamports,
            data: vec![0],
            owner: program_id, // Can only withdraw lamports from accounts owned by the program
            ..Account::default()
        },
    );
    program_test.add_account(
        destination_pubkey,
        Account {
            lamports: destination_lamports,
            ..Account::default()
        },
    );
    let (banks_client, payer, recent_blockhash) = program_test.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &source_lamports.to_le_bytes(),
            vec![
                AccountMeta::new(source_pubkey, false),
                AccountMeta::new(destination_pubkey, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
    let source = banks_client.get_account(source_pubkey).await.unwrap();
    assert_eq!(source, None);
    let destination = banks_client
        .get_account(destination_pubkey)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(destination.lamports, destination_lamports + source_lamports);
}
