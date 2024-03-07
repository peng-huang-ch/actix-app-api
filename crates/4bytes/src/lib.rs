pub mod bytes;
pub mod visit;

#[cfg(test)]
mod tests {

    use srv_storage::models::signatures::{self, Signature};
    #[tokio::main]
    #[test]
    async fn test_import_bytes() -> Result<(), Box<dyn std::error::Error>> {
        use crate::bytes::get_signature_bytes;
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set");
        let db = srv_storage::init_db(database_url.as_str()).await;
        let mut conn = db.get().await?;

        let signature_list = get_signature_bytes(None).await?;

        let records = signature_list
            .results
            .into_iter()
            .map(|text| Signature {
                hash: text.hex_signature,
                text: text.text_signature,
                abi: None,
            })
            .collect::<Vec<_>>();

        let hash = records[0].hash.clone();
        let _ = signatures::create_signature(&mut conn, records).await?;

        let signature = signatures::get_signature(&mut conn, hash).await?;
        assert!(signature.is_some());

        Ok(())
    }
}
