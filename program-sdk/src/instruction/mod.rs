pub mod accounts;
pub mod data;
use solana_sdk::{ instruction::Instruction, pubkey::Pubkey };

use crate::instruction::{
    accounts::{
        process_accounts,
        AccountsInitializeNewMerkleTree,
        AccountsInitializeUserAccount,
        AccountsTransact,
        GenericAccounts,
    },
    data::TransactInstructionDataLayout,
};

//pub const TEMPLATE_DISC: u8 = 255;
pub enum ProgramInstruction<'a> {
    InitializeNewMerkleTree(&'a GenericAccounts<AccountsInitializeNewMerkleTree>), //240
    InitializeUserAccount(&'a GenericAccounts<AccountsInitializeUserAccount>), //100
    Transact {
        accounts: &'a GenericAccounts<AccountsTransact>,
        data: TransactInstructionDataLayout,
    },
}

impl<'a> ProgramInstruction<'a> {
    pub fn new(&self, program_id: Pubkey) -> Instruction {
        match self {
            &ProgramInstruction::InitializeNewMerkleTree(accounts) => {
                Instruction {
                    program_id,
                    accounts: process_accounts(accounts),
                    data: vec![
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        instruction_indexes::INITIALIZE_NEW_MERKLE_TREE
                    ],
                }
            }
            &ProgramInstruction::InitializeUserAccount(accounts) => {
                Instruction {
                    program_id,
                    accounts: process_accounts(accounts),
                    data: vec![
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        instruction_indexes::INITIALIZE_NEW_USER_ACCOUNT
                    ],
                }
            }
            &ProgramInstruction::Transact { accounts, data } => {
                Instruction {
                    program_id,
                    accounts: process_accounts(accounts),
                    data: data.to_vec(),
                }
            }
        }
    }
}

pub mod instruction_indexes {
    pub const INITIALIZE_NEW_MERKLE_TREE: u8 = 240;
    pub const INITIALIZE_NEW_USER_ACCOUNT: u8 = 100;
}
