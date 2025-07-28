use crate::error::ProgramSdkResult;
use crate::instruction::{
    ProgramInstruction,
    accounts::{
        AccountsInitializeNewMerkleTree, AccountsInitializeUserAccount, AccountsTransact,
        GenericAccounts, ProgramAccounts,
    },
    data::TransactInstructionDataLayout,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    message::{VersionedMessage, v0::Message},
    pubkey::Pubkey,
    signer::Signer,
};

pub struct MainProgram {
    program_id: Pubkey,
    connection: RpcClient,
}

impl MainProgram {
    pub fn new(program_id: Pubkey, connection: RpcClient) -> Self {
        Self {
            program_id,
            connection,
        }
    }

    pub async fn initialize_new_merkle_tree(
        &self,
        accounts: &GenericAccounts<AccountsInitializeNewMerkleTree>,
    ) -> ProgramSdkResult<solana_sdk::signature::Signature> {
        let ix = ProgramInstruction::InitializeNewMerkleTree(accounts).new(self.program_id);

        let recent_blockhash = self.connection.get_latest_blockhash().await?;

        let message = Message::try_compile(
            &accounts.signer_account().pubkey(),
            &[ix],
            &[],
            recent_blockhash,
        )?;

        let versioned_message = VersionedMessage::V0(message);

        let tx =
            solana_sdk::transaction::VersionedTransaction::try_new(versioned_message, accounts)?;

        let signature: solana_sdk::signature::Signature =
            self.connection.send_and_confirm_transaction(&tx).await?;
        Ok(signature)
    }

    pub async fn initialize_user_account(
        &self,
        accounts: &GenericAccounts<AccountsInitializeUserAccount>,
    ) -> ProgramSdkResult<solana_sdk::signature::Signature> {
        let ix = ProgramInstruction::InitializeUserAccount(accounts).new(self.program_id);

        let recent_blockhash = self.connection.get_latest_blockhash().await?;

        let message = Message::try_compile(
            &accounts.signer_account().pubkey(),
            &[ix],
            &[],
            recent_blockhash,
        )?;

        let versioned_message = VersionedMessage::V0(message);

        let tx =
            solana_sdk::transaction::VersionedTransaction::try_new(versioned_message, accounts)?;

        let signature: solana_sdk::signature::Signature =
            self.connection.send_and_confirm_transaction(&tx).await?;
        Ok(signature)
    }

    pub async fn transact(
        &self,
        accounts: &GenericAccounts<AccountsTransact>,
        transact_instruction_data: TransactInstructionDataLayout,
    ) -> ProgramSdkResult<solana_sdk::signature::Signature> {
        let ix = (ProgramInstruction::Transact {
            accounts,
            data: transact_instruction_data,
        })
        .new(self.program_id);

        let recent_blockhash = self.connection.get_latest_blockhash().await?;

        let message = Message::try_compile(
            &accounts.signer_account().pubkey(),
            &[ix],
            &[],
            recent_blockhash,
        )?;

        let versioned_message = VersionedMessage::V0(message);

        let tx =
            solana_sdk::transaction::VersionedTransaction::try_new(versioned_message, accounts)?;

        let signature: solana_sdk::signature::Signature =
            self.connection.send_and_confirm_transaction(&tx).await?;
        Ok(signature)
    }
}
