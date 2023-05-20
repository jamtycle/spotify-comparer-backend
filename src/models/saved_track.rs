use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};

use super::track::Track;

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedTracks {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub user_id: String,
    #[serde(with="mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub added_at: DateTime,
    pub track: Track,
}