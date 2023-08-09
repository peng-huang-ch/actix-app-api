use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// User details.
#[derive(Queryable, Selectable, Insertable, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::signatures)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Signature {
    pub id: i32,
    pub signature: String,
    pub bytes: String,
    pub abi: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// New user details.
#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::signatures)]
pub struct NewSignature {
    pub signature: String,
    pub bytes: String,
    pub abi: Option<String>,
}

impl NewSignature {
    pub fn new(bytes: &str, signature: &str, abi: Option<&str>) -> Self {
        Self {
            bytes: bytes.to_string(),
            signature: signature.to_string(),
            abi: abi.map(|f| f.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new_signature() {
        NewSignature::new("0x123", "0x456", None);
        NewSignature::new("0x123", "0x456", Some("0x789"));
    }
}
