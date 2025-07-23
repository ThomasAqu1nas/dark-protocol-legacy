use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{ message::{ v0::Message, VersionedMessage }, pubkey::Pubkey, signer::Signer };

use crate::instruction::{
    accounts::{
        AccountsInitializeNewMerkleTree,
        AccountsInitializeUserAccount,
        AccountsTransact,
        GenericAccounts,
        ProgramAccounts,
    },
    data::TransactInstructionDataLayout,
    ProgramInstruction,
};

pub mod instruction;
pub mod prover;
pub mod utils;

pub struct MainProgram {
    program_id: Pubkey,
    connection: RpcClient,
}

impl MainProgram {
    pub fn new(program_id: Pubkey, connection: RpcClient) -> Self {
        Self { program_id, connection }
    }

    pub async fn initialize_new_merkle_tree(
        &self,
        accounts: &GenericAccounts<AccountsInitializeNewMerkleTree>
    ) -> anyhow::Result<solana_sdk::signature::Signature> {
        let ix = ProgramInstruction::InitializeNewMerkleTree(accounts).new(self.program_id);

        let recent_blockhash = self.connection.get_latest_blockhash().await?;

        let message = Message::try_compile(
            &accounts.signer_account().pubkey(),
            &[ix],
            &[],
            recent_blockhash
        )?;

        let versioned_message = VersionedMessage::V0(message);

        let tx = solana_sdk::transaction::VersionedTransaction::try_new(
            versioned_message,
            accounts
        )?;

        let signature: solana_sdk::signature::Signature = self.connection.send_and_confirm_transaction(
            &tx
        ).await?;
        Ok(signature)
    }

    pub async fn initialize_user_account(
        &self,
        accounts: &GenericAccounts<AccountsInitializeUserAccount>
    ) -> anyhow::Result<solana_sdk::signature::Signature> {
        let ix = ProgramInstruction::InitializeUserAccount(accounts).new(self.program_id);

        let recent_blockhash = self.connection.get_latest_blockhash().await?;

        let message = Message::try_compile(
            &accounts.signer_account().pubkey(),
            &[ix],
            &[],
            recent_blockhash
        )?;

        let versioned_message = VersionedMessage::V0(message);

        let tx = solana_sdk::transaction::VersionedTransaction::try_new(
            versioned_message,
            accounts
        )?;

        let signature: solana_sdk::signature::Signature = self.connection.send_and_confirm_transaction(
            &tx
        ).await?;
        Ok(signature)
    }

    pub async fn transact(
        &self,
        accounts: &GenericAccounts<AccountsTransact>,
        transact_instruction_data: TransactInstructionDataLayout
    ) -> anyhow::Result<solana_sdk::signature::Signature> {
        let ix = (ProgramInstruction::Transact {
            accounts,
            data: transact_instruction_data,
        }).new(self.program_id);

        let recent_blockhash = self.connection.get_latest_blockhash().await?;

        let message = Message::try_compile(
            &accounts.signer_account().pubkey(),
            &[ix],
            &[],
            recent_blockhash
        )?;

        let versioned_message = VersionedMessage::V0(message);

        let tx = solana_sdk::transaction::VersionedTransaction::try_new(
            versioned_message,
            accounts
        )?;

        let signature: solana_sdk::signature::Signature = self.connection.send_and_confirm_transaction(
            &tx
        ).await?;
        Ok(signature)
    }
}
