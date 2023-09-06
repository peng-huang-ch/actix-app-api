use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureList {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<TextSignature>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextSignature {
    pub created_at: String,
    pub text_signature: String,
    pub hex_signature: String,
}

const FOUR_BYTE_URL: &str = "https://www.4byte.directory/api/v1/signatures/";

const USER_AGENT_VALUE: &str = "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Mobile Safari/537.36";

pub async fn get_signature_bytes(
    url: Option<String>,
) -> Result<SignatureList, Box<dyn std::error::Error>> {
    let url = url.unwrap_or_else(|| FOUR_BYTE_URL.to_string());
    let client = reqwest::Client::builder().build()?;
    let response = client
        .get(url)
        .header(USER_AGENT, USER_AGENT_VALUE)
        .send()
        .await?;

    let body = response.json::<SignatureList>().await?;
    Ok(body)
}

#[cfg(test)]
mod test {
    use super::get_signature_bytes;
    #[tokio::main]
    #[test]
    async fn test_get_signature_bytes() -> Result<(), Box<dyn std::error::Error>> {
        let body = get_signature_bytes(None).await?;
        println!("{:?}", body);
        Ok(())
    }
}
