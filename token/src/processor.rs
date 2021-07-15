//! Program State Processor

use crate::{
    error::TokenError,
    instruction::{is_valid_signer_index, AuthorityType, TokenInstruction, MAX_SIGNERS},
    state::{Account, AccountState, Mint, Multisig},
};
use num_traits::FromPrimitive;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    decode_error::DecodeError,
    entrypoint::ProgramResult,
    info,
    program_error::{PrintProgramError, ProgramError},
    program_option::COption,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

// Program State handler
pub struct Processor {}
impl Processor {
    
    //Process an [InitializeMint](enum.TokenInstruction.html) instruction
    pub fn process_initial_mint(
        accounts: &[AccountInfo],
        decimals: u8,
        mint_authority: Pubkey,
        freeze_authority: COption<Pubkey>,
    ) -> ProgramResult {

}
