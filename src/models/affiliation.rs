use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffiliationId(pub i64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Affiliation {
    pub affiliation_id: AffiliationId,
    pub name: String
}