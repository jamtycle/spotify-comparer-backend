use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,

    pub id: String,
    pub display_name: String,
    pub country: String,
    pub image: String,
    pub product: String,
    pub email: String,
    pub friends: Vec<String>,
}