use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use srv_storage::models::tokens::Token;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenList {
    name: String,
    pub tokens: Vec<Token>,
}

const UNISWAP_TOKEN_LIST_URL: &str = "https://gateway.ipfs.io/ipns/tokens.uniswap.org";

const USER_AGENT_VALUE: &str = "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Mobile Safari/537.36";

// https://github.com/Uniswap/token-lists
// https://github.com/Uniswap/default-token-list/blob/main/src/tokens/mainnet.json
pub async fn get_token_list(url: Option<String>) -> Result<TokenList, Box<dyn std::error::Error>> {
    let url = url.unwrap_or_else(|| UNISWAP_TOKEN_LIST_URL.to_string());
    let client = reqwest::Client::builder().build()?;
    let response = client
        .get(url)
        .header(USER_AGENT, USER_AGENT_VALUE)
        .send()
        .await?;

    let body = response.json::<TokenList>().await?;
    Ok(body)
}

#[cfg(test)]
mod test {
    use super::get_token_list;
    #[tokio::main]
    #[test]
    async fn test_get_token_list() -> Result<(), Box<dyn std::error::Error>> {
        get_token_list(None).await?;
        Ok(())
    }
}
