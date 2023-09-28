use crate::error::MplProjectNameError;
use crate::instruction::{accounts::CreateAccounts, CreateArgs, MplProjectNameInstruction};
use crate::state::{Key, MyAccount, MyData};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program::invoke, pubkey::Pubkey,
    rent::Rent, system_instruction, system_program, sysvar::Sysvar,
};

pub struct Processor;

impl Processor {
    pub fn process_instruction<'a>(
        _program_id: &Pubkey,
        accounts: &'a [AccountInfo<'a>],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: MplProjectNameInstruction =
            MplProjectNameInstruction::try_from_slice(instruction_data)?;
        match instruction {
            MplProjectNameInstruction::Create(args) => {
                msg!("Instruction: Create");
                create(accounts, args)
            }
        }
    }
}

fn create<'a>(accounts: &'a [AccountInfo<'a>], args: CreateArgs) -> ProgramResult {
    let ctx = CreateAccounts::context(accounts)?;

    let address = ctx.accounts.address;
    let _authority = ctx.accounts.authority;
    let payer = ctx.accounts.payer;
    let system_program = ctx.accounts.system_program;

    let rent = Rent::get()?;

    // Guards.
    if *system_program.key != system_program::id() {
        return Err(MplProjectNameError::InvalidSystemProgram.into());
    }

    // Fetch the space and minimum lamports required for rent exemption.
    let space: usize = MyAccount::LEN;
    let lamports: u64 = rent.minimum_balance(space);

    // CPI to the System Program.
    invoke(
        &system_instruction::create_account(
            payer.key,
            address.key,
            lamports,
            space as u64,
            &crate::id(),
        ),
        &[payer.clone(), address.clone(), system_program.clone()],
    )?;

    // Get a mutable reference to the account data.
    let my_account_data = &mut (*address.data).borrow_mut();

    // Get the mutable byte muck version of the account so we can mutate the data directly.
    let my_account = MyAccount::from_bytes(my_account_data);

    // Now can operate on the struct like a normal Rust struct but the bytes are cast directly
    // without deserializ/serializ(ing).
    my_account.key = Key::MyAccount;
    my_account.authority = *ctx.accounts.authority.key;
    my_account.data = MyData {
        alice: args.alice,
        bob: args.bob,
    };

    // No need to serialize the data back into the account, it's already there.

    Ok(())
}
