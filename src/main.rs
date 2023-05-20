mod api;
mod models;
mod db;

#[macro_use]
extern crate rocket;

use api::{users_api::{getuser, adduser, deleteuser}, saved_tracks_api::{addsavedtracks, gettracks, deletetracks}, friends_api::{requestfriend, friendlist, acceptrequest, cancelrequest}};
use db::spotify_db::SpotifyDB;
use models::cors::CORS;

#[launch]
fn rocket() -> _ {
    let db = SpotifyDB::init();
    rocket::build()
        .attach(CORS)
        .manage(db)
        .mount("/", routes![getuser, adduser, deleteuser,
        addsavedtracks, gettracks, deletetracks,
        requestfriend, friendlist, acceptrequest, cancelrequest])
}