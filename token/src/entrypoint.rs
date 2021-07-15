//! Program entrypoint

use crate::{error::TokenError, processor::Processor}
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    program_error::PrintProgramError,
    pubkey::PubKey
};

// NOTE, not sure what the template <'a> is used for yet
entrypoint!(process_instruction);
fn process_instruction<'a>(
    program_id: &PubKey,
    accounts: &'a[AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    // Process input
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // Error caught
        error.print::<TokenError>();
        return Err(error);
    }
    // END
    Ok(())
}
