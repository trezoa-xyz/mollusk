#![cfg(target_os = "trezoa")]

use {trezoa_account_info::AccountInfo, trezoa_program_error::ProgramError, trezoa_pubkey::Pubkey};

// Declare the custom syscall that we expect to be registered.
// This matches the `sol_burn_cus` syscall from the test.
extern "C" {
    fn sol_burn_cus(to_burn: u64) -> u64;
}

trezoa_program_entrypoint::entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    input: &[u8],
) -> Result<(), ProgramError> {
    let to_burn = input
        .get(0..8)
        .and_then(|bytes| bytes.try_into().map(u64::from_le_bytes).ok())
        .ok_or(ProgramError::InvalidInstructionData)?;

    // Call the custom syscall to burn CUs.
    unsafe {
        sol_burn_cus(to_burn);
    }

    Ok(())
}
