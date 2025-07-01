#[cfg(test)]
mod tests {
    use solana_sdk::{signature::{Keypair, Signer, read_keypair_file}, pubkey::Pubkey};
    use solana_client::rpc_client::RpcClient;
    use solana_program::{system_instruction::transfer};
    use solana_sdk::transaction::Transaction;
    use std::str::FromStr;

    const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        
        // Establish connection to Solana devnet
        let client = RpcClient::new(RPC_URL);
        
        // Request 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }

    #[test]
    fn transfer_sol() {
        // Load dev wallet
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        
        // Define Turbin3 public key
        let to_pubkey = Pubkey::from_str("CvfeDM1oREcd3hq6JnojkAk6Fq29XWkaKctUzKb4BrJj").unwrap();
        
        // Connect to devnet
        let rpc_client = RpcClient::new(RPC_URL);
        
        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        
        // Create and sign transaction (0.1 SOL = 100,000,000 lamports)
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 100_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        
        // Send transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
            
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    }

    #[test]
    fn empty_wallet() {
        // Load dev wallet
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        
        // Define Turbin3 public key
        let to_pubkey = Pubkey::from_str("CvfeDM1oREcd3hq6JnojkAk6Fq29XWkaKctUzKb4BrJj").unwrap();
        
        // Connect to devnet
        let rpc_client = RpcClient::new(RPC_URL);
        
        // Get current balance
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
            
        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        
        // Build mock transaction to calculate fee
        use solana_sdk::message::Message;
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );
        
        // Get fee
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");
        
        // Create final transaction with balance minus fee
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        
        // Send transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send final transaction");
            
        println!("Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    }

    #[test]
    fn submit_rs() {
        // Load Turbin3 signer keypair
        let signer = read_keypair_file("Turbin3-wallet.json")
            .expect("Couldn't find Turbin3 wallet file");
        
        // Create RPC client
        let rpc_client = RpcClient::new(RPC_URL);
        
        // Define program and account public keys
        let mint = Keypair::new();
        let turbin3_prereq_program = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        let mpl_core_program = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        
        // Get the PDA for prereq account
        let signer_pubkey = signer.pubkey();
        let seeds = &[b"prereqs", signer_pubkey.as_ref()];
        let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);
        
        // Get the authority PDA for collection
        let authority_seeds = &[b"collection", collection.as_ref()];
        let (authority, _authority_bump) = Pubkey::find_program_address(authority_seeds, &turbin3_prereq_program);
        
        // Prepare instruction data (submit_rs discriminator)
        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];
        
        // Define accounts metadata
        use solana_sdk::instruction::{AccountMeta, Instruction};
        use solana_program::system_program;
        
        let accounts = vec![
            AccountMeta::new(signer.pubkey(), true),      // user signer
            AccountMeta::new(prereq_pda, false),          // PDA account
            AccountMeta::new(mint.pubkey(), true),        // mint keypair
            AccountMeta::new(collection, false),          // collection
            AccountMeta::new_readonly(authority, false),  // authority (PDA)
            AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
            AccountMeta::new_readonly(system_program::id(), false), // system program
        ];
        
        // Get recent blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        
        // Build the instruction
        let instruction = Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data,
        };
        
        // Create and sign transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint],
            blockhash,
        );
        
        // Send and confirm transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
            
        println!("Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    }
}