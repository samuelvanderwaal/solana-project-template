use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankBuilder, ShankContext, ShankInstruction};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct CreateArgs {
    /// Some description for foo.
    pub alice: u64,
    /// Some description for bar.
    pub bob: [u8; 32],
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, ShankBuilder, ShankContext, ShankInstruction,)]
#[rustfmt::skip]
pub enum MplProjectNameInstruction {
    /// Create My Account.
    /// A detailed description of the instruction.
    #[account(0, writable, signer, name="address", desc = "The address of the new account")]
    #[account(1, name="authority", desc = "The authority of the new account")]
    #[account(2, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(3, name="system_program", desc = "The system program")]
    Create(CreateArgs),
}
