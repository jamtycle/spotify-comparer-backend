use std::{env};
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc, Document},
    results::{DeleteResult, InsertManyResult, UpdateResult, InsertOneResult},    
    sync::{Client, Collection}, 
    options::*
    //options::{UpdateOptions, InsertManyOptions, WriteConcern},
};
use crate::models::{user::User, saved_track::SavedTracks, friends::Friend};

pub struct SpotifyDB {
    users: Collection<User>,
    saved_tracks: Collection<SavedTracks>,
    friends: Collection<Friend>,
}

impl SpotifyDB {
    pub fn init() -> Self {
        dotenv().ok();

        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("spotify-comparer");
        let users: Collection<User> = db.collection("users");
        let saved_tracks: Collection<SavedTracks> = db.collection("saved-tracks");
        let friends: Collection<Friend> = db.collection("friends");
        SpotifyDB { users, saved_tracks, friends }
    }
}

// Users
impl SpotifyDB {
    pub fn get_user(&self, user_id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(user_id).unwrap();
        let filter = doc! {"_id": obj_id};
        
        let info = self
            .users
            .find_one(filter, None)
            .ok()
            .expect("Error getting user.");

        Ok(info.unwrap())
    }

    pub fn create_user(&self, new_user: User) -> Result<UpdateResult, Error> {
        let id = String::from(new_user.id);
        let filter = doc! { "id": id.clone() }; 
        let new_doc = doc! {
            "$set":
            {
                "display_name": new_user.display_name,
                "country": new_user.country,
                "image": new_user.image,
                "product": new_user.product,
                "email": new_user.email,
                "friends": new_user.friends,
            }   
        };
        // User {
        //     _id: None,
        //     id: new_user.id,
        //     display_name: todo!(),
        //     country: todo!(),
        //     image: todo!(),
        //     product: todo!(),
        // };
        // let new_doc = User {
        //     _id: None,
        //     id: new_user.id,
        //     display_name: new_user.display_name,
        //     country: new_user.country,
        //     image: new_user.image,
        //     product: new_user.product,
        // };
        let opts = UpdateOptions::builder().upsert(Some(true)).build();
        
        // let info = self
        //     .users
        //     .update_one(filter, new_doc, opts)
        //     .ok()
        //     .expect("Error creting user");
        match self.users.update_one(filter, new_doc, opts) {
            Ok(res) => Ok(res),
            Err(e) => Err(Error::DeserializationError { message: e.to_string() })
        }

        // let info = self
        //     .users
        //     .insert_one(new_doc, None)
        //     .ok()
        //     .expect("Error creating user");

        // Ok(info)
    }

    pub fn remove_user(&self, _id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(_id).unwrap();
        let filter = doc! {"_id": obj_id};
        let info = self
            .users
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting users.");

        Ok(info)
    }
}

// SavedTracks
impl SpotifyDB {
    pub fn add_saved_track(&self, _tracks: Vec<SavedTracks>) -> Result<InsertManyResult, Error> {

        let options = InsertManyOptions::builder()
            .ordered(false)
            .write_concern(WriteConcern::builder().journal(true).build())
            .build();
        
        // let info = self
        //     .saved_tracks
        //     .insert_many(_tracks.iter(), options)
        //     // .map_err(|e| print!("\n============================\nError: {e}\n============================\n"))
        //     .ok()
        //     .expect("Could not insert tracks.");

        // Ok(info)

        match self.saved_tracks.insert_many(_tracks.iter(), options) {
            Ok(info) => Ok(info),
            Err(e) => Err(Error::DeserializationError { message: e.to_string() })
        }
        // .ok()
        // .expect("Could not insert tracks.");

        // return if let Some(info) = self.saved_tracks.insert_many(_tracks.iter(), options) {
        //     Ok(info)
        // } else {
        //     Err("Error creating saved tracks.")
        // }

        // Ok(info)
    }
    
    pub fn remove_saved_track(&self, _id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(_id).unwrap();
        let filter = doc! {"_id": obj_id};
        let info = self
            .saved_tracks
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting saved track.");

        Ok(info)
    }

    pub fn get_saved_tracks_by_user(&self, _user_id: &String) -> Result<Vec<SavedTracks>, Error> {
        // let obj_id = ObjectId::parse_str(_user_id).unwrap();
        let filter = doc! {"user_id": _user_id};
        let info = self
            .saved_tracks
            .find(filter, None)
            .ok()
            .expect("Error getting saved track's for.");

        Ok(info.map(|doc| doc.unwrap()).collect())
    }
}


// Friends
impl SpotifyDB {
    pub fn get_friend_list(&self, _id: &String) -> Result<Vec<Document>, Error> {

        let pipeline = 
        // vec! [
        //     doc! { "$match": { "user_id": _id.to_owned() } },
        //     doc! { "$unwind": "$user" },
        //     doc! {
        //         "$lookup": {
        //             "from": "users",
        //             "localField": "other_id",
        //             "foreignField": "id",
        //             "as": "user"
        //           }
        //     },
        //     doc! {
        //         "$project": {
        //             "_id": 0,
        //             "id": "$user.id",
        //             "username": "$user.display_name",
        //             "requested": "$requested"
        //           }
        //     }
        // ];
        vec![
            doc! {
                "$lookup": {
                    "from": "users",
                    "localField": "other_id",
                    "foreignField": "id",
                    "as": "user"
                }
            },
            doc! {
                "$unwind": "$user"
            },
            doc! {
                "$match": {
                    "user_id": _id.to_owned()
                }
            },
            doc! {
                "$project": {
                    "_id": 0,
                    "id": "$other_id",
                    "username": "$user.display_name",
                    "requested": "$requested"
                }
            },
        ];

        match self.friends.aggregate(pipeline, None) {
            Ok(info) => info
                .filter_map(Result::ok) // Filter out any errors
                .map(|doc| {
                    // Ok(doc.to_owned())
                    let id = doc.get_str("id").unwrap().to_string();
                    let username = doc.get_str("username").unwrap().to_string();
                    let requested = doc.get_bool("requested").unwrap();
                    Ok(doc! {
                        "id": id,
                        "username": username,
                        "requested": requested,
                    })
                })
                .collect::<Result<Vec<Document>, Error>>(),
            Err(e) => Err(Error::DeserializationError { message: format!("Failed to get friend list: {}", e) }),
        }

    }

    pub fn request_friend(&self, _id: &String, _other_id: &String) -> Result<InsertOneResult, Error> {

        let friend = Friend {
            user_id: _id.to_owned(),
            other_id: _other_id.to_owned(),
            requested: true,
        };

        match self.friends.insert_one(friend, None) {
            Ok(info) => Ok(info),
            Err(e) => Err(Error::DeserializationError { message: e.to_string() }),
        }
    }
    
    pub fn accept_request(&self, _id: &String, _other_id: &String) -> Result<UpdateResult, Error> {
        let filter = doc! { "user_id": _id, "other_id": _other_id };
        let update_doc = doc! { "requested": true };

        match self.saved_tracks.update_one(filter, update_doc, None) {
            Ok(info) => Ok(info),
            Err(e) => Err(Error::DeserializationError { message: e.to_string() })
        }
    }

    pub fn cancel_request(&self, _id: &String, _other_id: &String) -> Result<DeleteResult, Error> {
        let filter = doc! { "user_id": _id, "other_id": _other_id };

        match self.saved_tracks.delete_one(filter, None) {
            Ok(info) => Ok(info),
            Err(e) => Err(Error::DeserializationError { message: e.to_string() })
        }
        
        // Ok(info.map(|doc| doc.unwrap()).collect())
    }
}