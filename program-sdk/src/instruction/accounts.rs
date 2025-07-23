use solana_sdk::{ instruction::AccountMeta, pubkey::Pubkey, signer::Signer, signers::Signers };

impl<T: ProgramAccounts> Signers for GenericAccounts<T> {
    fn pubkeys(&self) -> Vec<Pubkey> {
        self.process_accounts()
            .iter()
            .map(|meta| meta.pubkey)
            .collect()
    }

    fn try_pubkeys(&self) -> Result<Vec<Pubkey>, solana_sdk::signer::SignerError> {
        Ok(
            self
                .process_accounts()
                .iter()
                .map(|meta| meta.pubkey)
                .collect()
        )
    }

    fn sign_message(&self, message: &[u8]) -> Vec<solana_sdk::signature::Signature> {
        vec![self.signer_account().sign_message(message)]
    }

    fn try_sign_message(
        &self,
        message: &[u8]
    ) -> Result<Vec<solana_sdk::signature::Signature>, solana_sdk::signer::SignerError> {
        Ok(vec![self.signer_account().sign_message(message)])
    }

    fn is_interactive(&self) -> bool {
        false
    }
}

impl<T: ProgramAccounts> ProgramAccounts for GenericAccounts<T> {
    fn signer_account(&self) -> &solana_sdk::signer::keypair::Keypair {
        self.0.signer_account()
    }

    fn process_accounts(&self) -> Vec<AccountMeta> {
        self.0.process_accounts()
    }
}

pub struct GenericAccounts<T: ProgramAccounts>(T);

pub struct AccountsInitializeNewMerkleTree {
    pub signer_account: solana_sdk::signer::keypair::Keypair,
    pub merkle_tree_storage_account: Pubkey,
    pub rent_sysvar_account: Pubkey,
}

pub struct AccountsInitializeUserAccount {
    pub signer_account: solana_sdk::signer::keypair::Keypair,
    pub user_account: Pubkey,
    pub rent_sysvar_account: Pubkey,
}

pub struct AccountsTransact {
    pub signer_account: solana_sdk::signer::keypair::Keypair,
    pub tmp_storage_pda: Pubkey,
}

pub fn process_accounts<T: ProgramAccounts>(accounts: &T) -> Vec<AccountMeta> {
    accounts.process_accounts()
}

pub trait ProgramAccounts {
    fn signer_account(&self) -> &solana_sdk::signer::keypair::Keypair;
    fn process_accounts(&self) -> Vec<AccountMeta>;
}

impl ProgramAccounts for AccountsInitializeNewMerkleTree {
    fn signer_account(&self) -> &solana_sdk::signer::keypair::Keypair {
        &self.signer_account
    }
    fn process_accounts(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.signer_account.pubkey(), true),
            AccountMeta::new(self.merkle_tree_storage_account, false),
            AccountMeta::new_readonly(self.rent_sysvar_account, false)
        ]
    }
}
impl ProgramAccounts for AccountsInitializeUserAccount {
    fn signer_account(&self) -> &solana_sdk::signer::keypair::Keypair {
        &self.signer_account
    }
    fn process_accounts(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.signer_account.pubkey(), true),
            AccountMeta::new(self.user_account, false),
            AccountMeta::new_readonly(self.rent_sysvar_account, false)
        ]
    }
}
impl ProgramAccounts for AccountsTransact {
    fn signer_account(&self) -> &solana_sdk::signer::keypair::Keypair {
        &self.signer_account
    }
    fn process_accounts(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.signer_account.pubkey(), true),
            AccountMeta::new(self.tmp_storage_pda, false)
        ]
    }
}
