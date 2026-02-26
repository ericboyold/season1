use chrono::Utc;
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

/// Transaction structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: u64,
}

/// Block structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    index: u64,
    timestamp: i64,
    transactions: Vec<Transaction>,
    proof: u64,          // Proof of work (nonce)
    previous_hash: String,
}

impl Block {
    /// Compute the SHA256 hash of the current block
    fn hash(&self) -> String {
        // Serialize the block into JSON and then hash it
        let json_str = serde_json::to_string(self).expect("序列化失败");
        let mut hasher = Sha256::new();
        hasher.update(json_str.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

/// Blockchain structure
struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
    difficulty: usize,  // Difficulty: number of leading zeros in the hash
}

impl Blockchain {
    /// Create a new blockchain (including the genesis block)
    fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            difficulty,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    /// Create the genesis block
    fn create_genesis_block(&mut self) {
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp(),
            transactions: vec![],
            proof: 0,
            previous_hash: String::from("0"),
        };
        self.chain.push(genesis_block);
    }

    /// Add a pending transaction
    fn add_transaction(&mut self, sender: String, recipient: String, amount: u64) {
        self.pending_transactions.push(Transaction {
            sender,
            recipient,
            amount,
        });
    }

    /// Mining: generate a new block and append it to the chain
    fn mine_block(&mut self) -> Block {
        let last_block = self.chain.last().unwrap();
        let previous_hash = last_block.hash();
        let index = self.chain.len() as u64;
        let timestamp = Utc::now().timestamp();
        let transactions = self.pending_transactions.clone();
        self.pending_transactions.clear();

        // Proof of work: find a proof that satisfies the difficulty
        let proof = self.proof_of_work(index, timestamp, &transactions, &previous_hash);

        let new_block = Block {
            index,
            timestamp,
            transactions,
            proof,
            previous_hash,
        };
        self.chain.push(new_block.clone());
        new_block
    }

    /// Proof of Work (PoW)
    fn proof_of_work(
        &self,
        index: u64,
        timestamp: i64,
        transactions: &[Transaction],
        previous_hash: &str,
    ) -> u64 {
        let mut proof = 0;
        let target = "0".repeat(self.difficulty);

        loop {
            // Build a temporary block to compute its hash
            let block = Block {
                index,
                timestamp,
                transactions: transactions.to_vec(),
                proof,
                previous_hash: previous_hash.to_string(),
            };
            let hash = block.hash();
            if hash.starts_with(&target) {
                println!("✅ 找到有效 proof: {} 哈希: {}", proof, hash);
                return proof;
            }
            proof += 1;
            if proof % 100_000 == 0 {
                println!("⏳ 正在尝试 proof: {}", proof);
            }
        }
    }

    /// 验证整条链的有效性
    fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // 检查 previous_hash 是否正确链接
            if current.previous_hash != previous.hash() {
                println!("❌ 区块 {} 的 previous_hash 不匹配", i);
                return false;
            }

            // 检查当前区块的哈希是否满足难度要求
            let current_hash = current.hash();
            if !current_hash.starts_with(&"0".repeat(self.difficulty)) {
                println!("❌ 区块 {} 的哈希不符合难度要求", i);
                return false;
            }
        }
        println!("✅ 区块链有效");
        true
    }
}

fn main() {
    // Initialize the blockchain with difficulty set to 4 leading zeros
    let mut blockchain = Blockchain::new(4);

    // Add some transactions
    blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 50);
    blockchain.add_transaction("Bob".to_string(), "Charlie".to_string(), 25);

    // Mine the first block
    println!("\n⛏️  开始挖第一个区块...");
    let block1 = blockchain.mine_block();
    println!("📦 生成区块 1：{:#?}", block1);

    // Add more transactions
    blockchain.add_transaction("Charlie".to_string(), "David".to_string(), 10);

    // Mine the second block
    println!("\n⛏️  开始挖第二个区块...");
    let block2 = blockchain.mine_block();
    println!("📦 生成区块 2：{:#?}", block2);

    // Validate the entire chain
    println!("\n🔍 验证区块链...");
    blockchain.is_chain_valid();

    // Demonstrate tampering by modifying the transaction amount in a block
    println!("\n⚠️  模拟篡改：修改第二个区块的交易金额...");
    if let Some(block) = blockchain.chain.get_mut(1) {
        block.transactions[0].amount = 100; // Change the amount of the first transaction to 100
    }
    blockchain.is_chain_valid();
}