

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum FundInstruction {
    Donate,
    Scam
}