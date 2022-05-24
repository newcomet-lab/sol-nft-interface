use {
  borsh::{
    BorshDeserialize, BorshSerialize,
  },
};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct CreateNFTInterfaceAccountArgs {
  pub token_price_per_nft: u64,
  pub max_supply: u16,
  pub max_token_purchase: u8,
  pub is_sealed: u8,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct ModifyNFTInterfaceAccountArgs {
  pub token_price_per_nft: Option<u64>,
  pub max_supply: Option<u16>,
  pub total_supply: Option<u16>,
  pub max_token_purchase: Option<u8>,
  pub is_sealed: Option<u8>,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct MintNFTInterfaceAccountArgs {
 pub wanted_supply: u16,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct GetFeeNftInterfaceAccountArgs {
 pub wanted_supply: Option<u64>,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum NFTInterfaceInstruction {
  /// Create Nft interface Account
  /// 0. `[writable]` new account key,
  /// 1. `[]` fee receiver account key, 
  /// 1. `[signer]` payer,
  /// 2. `[]` Modify_authority_key,
  /// 3. `[]` System program
  /// 4. `[]` Rent info
  CreateNFTInterfaceAccount(CreateNFTInterfaceAccountArgs),

  /// Modify a nft interface
  /// 0. `[writable]` nft interface account
  /// 1. `[signer]` Modify authority key
  ModifyNFTInterfaceAccount(ModifyNFTInterfaceAccountArgs),

  /// This function perform nft mint 
  /// 0. `[]` nft_interface_account
  /// 1. `[]` update_authority_account for get nft_interface and fee_receiver
  /// 2. `[]` fee_receiver_account
  /// 3. `[]` system program
  MintNFTInterfaceAccount(MintNFTInterfaceAccountArgs),

  /// This function perform feedback function.
  /// 0. `[]` nft_interface_account
  /// 1. `[]` update_authority_account
  /// 2. `[]` fee_receiver_account
  /// 3. `[]` receiver account
  /// 4. `[]` system program
  GetFeeNftInterfaceAccount(GetFeeNftInterfaceAccountArgs),
}