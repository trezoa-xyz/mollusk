use {
    trezoa_account::Account,
    trezoa_instruction::{BorrowedAccountMeta, BorrowedInstruction, Instruction},
    trezoa_instructions_sysvar::construct_instructions_data,
    trezoa_pubkey::Pubkey,
};

pub fn keyed_account<'a>(instructions: impl Iterator<Item = &'a Instruction>) -> (Pubkey, Account) {
    let data = construct_instructions_data(
        instructions
            .map(|instruction| BorrowedInstruction {
                program_id: &instruction.program_id,
                accounts: instruction
                    .accounts
                    .iter()
                    .map(|meta| BorrowedAccountMeta {
                        pubkey: &meta.pubkey,
                        is_signer: meta.is_signer,
                        is_writable: meta.is_writable,
                    })
                    .collect(),
                data: &instruction.data,
            })
            .collect::<Vec<_>>()
            .as_slice(),
    );

    (
        trezoa_instructions_sysvar::ID,
        Account {
            lamports: 0,
            data,
            owner: trezoa_sysvar_id::ID,
            executable: false,
            rent_epoch: Default::default(),
        },
    )
}
