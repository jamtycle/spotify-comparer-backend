use crate::{models::friends::Friend, db::spotify_db::SpotifyDB};
use mongodb::{results::{InsertOneResult, UpdateResult, DeleteResult}, bson::Document};
use rocket::{http::Status, serde::json::Json, State};

#[get("/FriendList/<user_id>")]
pub fn friendlist(db: &State<SpotifyDB>, user_id: String) -> Result<Json<Vec<Document>>, Status> {
    if user_id.is_empty() {
        return Err(Status::BadRequest);
    }

    match db.get_friend_list(&user_id) {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/RequestFriend", data="<fr>")]
pub fn requestfriend(db: &State<SpotifyDB>, fr: Json<Friend>) -> Result<Json<InsertOneResult>, Status> {
    match db.request_friend(&fr.user_id, &fr.other_id) {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/AcceptRequest", data="<fr>")]
pub fn acceptrequest(db: &State<SpotifyDB>, fr: Json<Friend>) -> Result<Json<UpdateResult>, Status> {
    match db.accept_request(&fr.user_id, &fr.other_id) {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/CancelRequest", data="<fr>")]
pub fn cancelrequest(db: &State<SpotifyDB>, fr: Json<Friend>) -> Result<Json<DeleteResult>, Status> {
    match db.cancel_request(&fr.user_id, &fr.other_id) {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError),
    }
}