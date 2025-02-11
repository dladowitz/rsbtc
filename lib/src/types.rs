use create::U256;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct Blockchain {
  pub blocks: Vec<Block>,
}

impl Blockchain {
  pub fn new() -> Self {
    Self { blocks: vec![] }
  }

  pub fn add_block(&mut self, block: Block) {
    self.blocks.push(block);
  }
}

pub struct Block {
  pub header: BlockHeader,
  pub transactions: Vec<Transaction>,
}

impl Block {
  pub fn new(
    header: BlockHeader,
    transactions: Vec<Transaction>
  ) -> Self {
    Self {
      header: header,
      transactions: transactions,
    }
  }

  pub fn hash(&self) -> String {
    unimplemented!()
  }
}

pub struct BlockHeader {
  /// Timestamp of the block
  pub timestamp: DateTime<Utc>,
  /// Nonce used to mine the block
  pub nonce: u64,
  /// Hash of the previous block
  pub prev_block_hash: [u8; 32],
  /// Merkle root of the block's transactions
  pub merkle_root: [u8; 32],
  /// target
  pub target: U256,
}

impl BlockHeader {
  pub fn new(
    timestamp: DateTime<Utc>,
    nonce: u64,
    prev_block_hash: [u8; 32],
    merkle_root: [u8; 32],
    target: U256,
  ) -> Self {
    Self { 
      timestamp: timestamp  ,
      nonce: nonce,
      prev_block_hash: prev_block_hash,
      merkle_root: merkle_root,
      target: target,
    }
  }

  pub fn hash(&self) -> String {
    unimplemented!()
  }
}

pub struct Transaction {
  pub inputs: Vec<TransactionInput>,
  pub outputs: Vec<TransactionOutput>,
}

impl Transaction {
  pub fn new(
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
  ) -> Self {
    // Transaction { (the book uses "Transaction", I think this is a mistake, but also the same as using "Self")
    Self { 
      inputs: inputs, 
      outputs: outputs,
    }
  }

  pub fn hash(&self) -> String {
    unimplemented!()
  }
}

pub struct TransactionInput {
  pub prev_transaction_output_hash: [u8; 32],
  pub signature: [u8; 64],
}

pub struct TransactionOutput {
  pub value: u64,
  pub unique_id: Uuid,
  pub pubkey: [u8; 33],
}
