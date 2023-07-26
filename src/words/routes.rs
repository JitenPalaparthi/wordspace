use super::{create_file_if_not_exists, Word};
use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use serde_json;

#[post("/v1/public/words")]
pub async fn add_word(_word: web::Json<Word>) -> impl Responder {
    // steps to write logic
    // 1- read post data
    // 2- deserialize that data into word struct object
    // 3- check if there is a file with the name of the word in file system
    // 4- if file exists say word already existed
    // 5- if not create a file and store json data in the file

    let mut word = _word.into_inner(); // 1
    word.last_modified = Utc::now().timestamp(); // update last_modified time
    if word.status == "active" {
        word.status = "inactive".to_string();
    }

    let word_json = serde_json::to_string(&word).unwrap();

    println!("{}", word_json);
    let ws:String= String::from("datastore/")+&word.word.clone()+ &String::from(".ws");

    match create_file_if_not_exists(&word_json.to_string(),&ws) {
        Ok(_) => {
            return HttpResponse::Ok().json(word);
        }
        Err(e) => {
            return HttpResponse::BadRequest().body(e.to_string());
        }
    }
}
