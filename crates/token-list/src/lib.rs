pub mod polygon;
pub mod uniswap;

#[cfg(test)]
mod tests {

    use srv_storage::models::tokens;
    #[tokio::main]
    #[test]
    async fn test_import_polygon_token_list() -> Result<(), Box<dyn std::error::Error>> {
        use crate::polygon::get_token_list;
        dotenvy::dotenv().ok();

        const CHAIN_ID: i32 = 137;
        let database_url = std::env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set");
        let db = srv_storage::init_db(database_url.as_str()).await;
        let mut conn = db.get().await?;

        let token_list = get_token_list(None).await?;

        let records = token_list
            .tokens
            .into_iter()
            .map(|mut token| {
                if token.chain_id.is_none() {
                    token.chain_id = Some(CHAIN_ID)
                }
                token
            })
            .collect::<Vec<_>>();

        let address = records[0].address.clone();
        let _ = tokens::create_tokens(&mut conn, records).await?;

        let token = tokens::get_token_by_address(&mut conn, CHAIN_ID, address).await?;
        assert_eq!(token.is_some(), true);

        Ok(())
    }

    #[tokio::test]
    async fn test_import_uniswap_token_list() -> Result<(), Box<dyn std::error::Error>> {
        use crate::uniswap::get_token_list;
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set");
        let db = srv_storage::init_db(database_url.as_str()).await;
        let mut conn = db.get().await?;

        let token_list = get_token_list(None).await?;

        let records = token_list
            .tokens
            .into_iter()
            .filter(|token| token.chain_id.is_some())
            .collect::<Vec<_>>();

        let chain_id = records[0].chain_id.clone().unwrap();
        let address = records[0].address.clone();
        let _ = tokens::create_tokens(&mut conn, records).await?;

        let token = tokens::get_token_by_address(&mut conn, chain_id, address).await?;
        assert_eq!(token.is_some(), true);

        Ok(())
    }
}
