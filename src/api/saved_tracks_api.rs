use crate::{models::{saved_track::SavedTracks, track::Track, artist::Artist}, db::spotify_db::SpotifyDB};
use mongodb::results::InsertManyResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/addsavedtracks", data = "<tracks>")]
pub fn addsavedtracks(db: &State<SpotifyDB>, tracks : Json<Vec<SavedTracks>>) -> Result<Json<InsertManyResult>, Status> {
    let data = tracks.iter().map(|x| SavedTracks {
        id: None,
        user_id: x.user_id.to_owned(),
        added_at: x.added_at.to_owned(),
        track: Track {
            _id: None,
            id: x.track.id.to_owned(),
            cover_image: x.track.cover_image.to_owned(),
            name: x.track.name.to_owned(),
            song_link: x.track.song_link.to_owned(),
            artists: x.track.artists.iter().map(|a| Artist {
                _id: None,
                id: a.id.to_owned(),
                genres: a.genres.to_owned(),
                image: a.image.to_owned(),
                name: a.name.to_owned(),
            }).collect()
        }
    });

    let user_detail = db.add_saved_track(data.collect());
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => {
            // println!("Error {e}");
            Err(Status::InternalServerError)
        },
    }
} 

#[get("/gettracks/<user_id>")]
pub fn gettracks(db: &State<SpotifyDB>, user_id: String) -> Result<Json<Vec<SavedTracks>>, Status> {
    if user_id.is_empty() {
        return Err(Status::BadRequest);
    }

    let data = db.get_saved_tracks_by_user(&user_id);
    match data { 
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/deletetracks", data = "<_id>")]
pub fn deletetracks(db: &State<SpotifyDB>, _id: String) -> Result<Json<&str>, Status> {
    if _id.is_empty() {
        return Err(Status::BadRequest);
    }

    let result = db.remove_saved_track(&_id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Track successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }
}