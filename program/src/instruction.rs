use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum ScamFundInstruction {
  Init { admin_address: [u8; 32] },   
  Donate { amount: u64 },
  Scam { amount: u64 }
}