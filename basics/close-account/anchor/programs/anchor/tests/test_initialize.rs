use {
    anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas, AccountDeserialize},
    litesvm::LiteSVM,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_keypair::Keypair,
    solana_transaction::versioned::VersionedTransaction,
};
use anchor::state::DataAccount;

#[test]
fn test_initialize() {
    let program_id = anchor::id();
    let payer = Keypair::new(); 
    
    // 1. Properly derive the PDA (Program Derived Address)
    let (data_account, _) = anchor_lang::prelude::Pubkey::find_program_address(
        &[b"create", payer.pubkey().as_ref()], 
        &program_id
    );  

    let mut svm = LiteSVM::new(); 
    let bytes = include_bytes!("../../../target/deploy/anchor.so"); 
    svm.add_program(program_id, bytes).unwrap(); 
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap(); 
    
    let instruction = Instruction::new_with_bytes(
        program_id,
        &anchor::instruction::Create {
            name: "test".to_string(),
        }.data(),
        anchor::accounts::Create {
            user: payer.pubkey(),
            data_account: data_account,
            system_program: anchor_lang::system_program::ID,
        }.to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    // 2. Fetch the account from LiteSVM
    let raw_account = svm.get_account(&data_account).unwrap();
    
    // 3. Deserialize the data using AccountDeserialize
    let mut data_slice: &[u8] = &raw_account.data;
    let parsed_account = DataAccount::try_deserialize(&mut data_slice).unwrap();

    // 4. Assert against the parsed struct fields
    assert_eq!(parsed_account.owner, payer.pubkey());
    assert_eq!(parsed_account.name, "test".to_string());
}
