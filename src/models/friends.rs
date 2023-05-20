use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Friend {
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // pub _id: Option<ObjectId>,

    pub user_id: String,
    pub other_id: String,
    pub requested: bool,
}