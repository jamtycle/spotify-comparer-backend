use crate::{models::user::User, db::spotify_db::SpotifyDB};
use mongodb::results::UpdateResult;
use rocket::{http::Status, serde::json::Json, State};

#[get("/getuser/<user_id>")]
pub fn getuser(db: &State<SpotifyDB>, user_id: String) -> Result<Json<User>, Status> {
    if user_id.is_empty() {
        return Err(Status::BadRequest);
    }

    let data = db.get_user(&user_id);
    match data {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/adduser", data = "<new_user>")]
pub fn adduser(db: &State<SpotifyDB>, new_user: Json<User>) -> Result<Json<UpdateResult>, Status> {
    let data = User {
        _id: None,
        id: new_user.id.to_owned(),
        display_name: new_user.display_name.to_owned(),
        country: new_user.country.to_owned(),
        image: new_user.image.to_owned(),
        product: new_user.product.to_owned(),
        email: new_user.email.to_owned(),
        friends: new_user.friends.to_owned(),
    };

    let user_detail = db.create_user(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
} 

#[delete("/deleteuser", data = "<_id>")]
pub fn deleteuser(db: &State<SpotifyDB>, _id: String) -> Result<Json<&str>, Status> {
    if _id.is_empty() {
        return Err(Status::BadRequest);
    }

    let result = db.remove_user(&_id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("User successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}