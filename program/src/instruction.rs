use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum ScamFundInstruction {
    Donate { amount: u64 },
    Scam { admin_address: [u8; 32], amount: u64 }
}