use super::{create_file_if_not_exists, read_last_line_of_file,append_to_the_file, Word};
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

    // 1- read last line from wordindex file
    // 2- if there is not line, set the file name as 1
    // 3- if there are lines , then pick the last line , split the file based on =
    // 4- the 0th index must be a number. Take that number and increment it
    // 5- the incremented number must be a new file name

    let file_content = read_last_line_of_file("datastore/word.index");
    let mut counter: i32 = 0;
    match file_content {
        Ok(lastline) => {
            // split here
            // take 0th index and convert to i32
            if lastline == "" {
                counter = 1;
            } else {
                let parts: Vec<&str> = lastline.splitn(2, '=').collect();
                match parts[0].parse::<i32>() {
                    Ok(v) => counter = v+1,
                    Err(e) => {
                        return HttpResponse::BadRequest().body(e.to_string());
                    }
                }
            }
        }
        Err(e) => {
            return HttpResponse::BadRequest().body(e.to_string());
        }
    }

    //let ws: String = String::from("datastore/") + &word.word.clone() + &String::from(".ws");
    let ws: String = String::from("datastore/") + &counter.to_string();

    // match create_file_if_not_exists(&word_json.to_string(), &ws) {
    match create_file_if_not_exists(&word_json.to_string(), &ws) {
        Ok(_) => {

            let counterline:String=counter.to_string()+"="+&word.word.clone();
            match append_to_the_file(&counterline,"datastore/word.index"){
                Err(e) => {
                    return HttpResponse::BadRequest().body(e.to_string());
                }
                Ok(_)=>{}
            }


            return HttpResponse::Ok().json(word);
        }
        Err(e) => {
            return HttpResponse::BadRequest().body(e.to_string());
        }
    }
}
