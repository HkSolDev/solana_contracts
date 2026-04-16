#[test]
fn test_deposit() {
    let mollusk = setup();

    let (system_program, system_program_account) = keyed_account_for_system_program();

    let user = Address::new_unique();
    let user_account = Account::new(10_000_000_000, 0, &system_program);

    let (vault, _vault_bump) =
        Address::find_program_address(&[b"vault", user.as_ref()], &crate::ID);
    let vault_account = Account::new(0, 0, &system_program);

    let deposit_amount: u64 = 1_000_000_000;

    let instruction: Instruction = DepositInstruction {
        user,
        vault,
        system_program,
        amount: deposit_amount,
    }
    .into();

    let result = mollusk.process_instruction(
        &instruction,
        &[
            (user, user_account.clone()),
            (vault, vault_account.clone()),
            (system_program, system_program_account.clone()),
        ],
    );

    assert!(result.program_result.is_ok());

    let user_after = result.resulting_accounts[0].1.lamports;
    let vault_after = result.resulting_accounts[1].1.lamports;

    assert_eq!(user_after, 10_000_000_000 - deposit_amount);
    assert_eq!(vault_after, deposit_amount);
}
