use anchor_lang::{
    prelude::Pubkey, system_program, AccountDeserialize, InstructionData, ToAccountMetas,
};
use litesvm::LiteSVM;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_message::{Message, VersionedMessage};
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;

#[test]
fn test_create_pda_account() {
    let program_id = pda_cpi::ID;

    let mut svm = LiteSVM::new();
    let program_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/deploy/pda_cpi.so"
    );

    svm.add_program_from_file(program_id, program_path)
        .expect("Failed to load PDA program");

    let payer = Keypair::new();

    svm.airdrop(&payer.pubkey(), 5_000_000_000)
        .expect("Airdrop failed");

    // Derive the PDA using the same seeds as the program.
    let (pda, bump) =
        Pubkey::find_program_address(&[b"client1", payer.pubkey().as_ref()], &program_id);

    let instruction = Instruction {
        program_id,
        accounts: pda_cpi::accounts::CreatedPdaAccount {
            payer: payer.pubkey(),
            pda_account: pda,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: pda_cpi::instruction::CreatePdaAccount {}.data(),
    };

    let recent_blockhash = svm.latest_blockhash();

    let message =
        Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &recent_blockhash);

    let transaction = VersionedTransaction::try_new(VersionedMessage::Legacy(message), &[&payer])
        .expect("Failed to sign transaction");

    // Execute the instruction.
    svm.send_transaction(transaction)
        .expect("PDA creation transaction failed");

    // Verify the PDA account exists.
    let account = svm.get_account(&pda).expect("PDA account was not created");

    assert_eq!(account.owner, program_id);
    assert_eq!(account.data.len(), 49);

    // Deserialize and verify the account state.
    let mut data: &[u8] = &account.data;

    let state = pda_cpi::StakeAccount::try_deserialize(&mut data)
        .expect("Failed to deserialize StakeAccount");

    assert_eq!(state.authority, payer.pubkey());
    assert_eq!(state.amount, 0);
    assert_eq!(state.bump, bump);
}
