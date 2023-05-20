use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use super::artist::Artist;

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,

    pub id: String,
    pub name: String,
    pub song_link: String,
    pub cover_image: String,
    pub artists: Vec<Artist>,
}