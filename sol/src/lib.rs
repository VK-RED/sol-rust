use solana_sdk::{program_pack::Pack, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};
use solana_client::nonblocking::rpc_client::RpcClient;
use spl_token::state::Mint;

pub struct SolClient {
    client : RpcClient,
    keypair: Keypair,
}

impl SolClient {

    pub fn new() -> Self{

        dotenv::dotenv().ok();
        let address = "https://api.devnet.solana.com".to_string();
        let client = RpcClient::new(address);
        let key = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
        let keypair = Keypair::from_base58_string(&key);

        Self {
            client,
            keypair
        }
    }

    pub async fn create_token(&self) -> Result<String, String>{
        
        let pubkey = self.keypair.pubkey();
        let token = Keypair::new();

        let lamports = self.client
        .get_minimum_balance_for_rent_exemption(Mint::LEN).await.map_err(|e|e.to_string())?;
        
        let ins_1 = solana_sdk::system_instruction::create_account(
            &pubkey, 
            &token.pubkey(), 
            lamports, 
            Mint::LEN as u64, 
            &spl_token::ID
        );

        let ins_2 = spl_token::instruction::initialize_mint(
            &spl_token::ID, 
            &token.pubkey(), 
            &pubkey, 
            Some(&pubkey), 
            0
        )
        .map_err(|e|e.to_string())?;

        let blockhash = self.client.get_latest_blockhash().await.map_err(|e|e.to_string())?;

        let tx = Transaction::new_signed_with_payer(
            &[
                ins_1,
                ins_2
            ], 
            Some(&pubkey), 
            &[&token,&self.keypair], 
            blockhash
        );

        let res = self.client
        .send_and_confirm_transaction_with_spinner(&tx)
        .await
        .map_err(|e|e.to_string())?;

        let result = format!(
            "mint key : {:?}, tx : {}",
            token.pubkey(),
            res
        );

        Ok(result)
    }

    pub async fn get_balance(&self, address:&str) -> Result<u64, String>{
        let pubkey = Pubkey::from_str_const(address);
        let balance = self.client.get_balance(&pubkey).await.or_else(|e|Err(e.to_string()));
        balance
    }
}

// {"Ok":"mint key : DKwDapwDcjXp7V6fz9u4cGjURsGxE92im27JxrWdbHUs, tx : 4RGh6irStfEYcwyVhqqXG7mZmVA3VsDeQ3VkzWb7er8bDqch8H7Zi2BzCfLqghNdCMAQSsDkPdgjjzo3pSmfZ2Kx"}