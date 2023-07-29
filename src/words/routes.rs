use super::{
    append_to_the_file, create_file_if_not_exists, read_file, read_last_line_of_file,delete_file,read_files_in_directory, Word,
};
use actix_web::{get, post,delete, web, HttpResponse, Responder};
use chrono::Utc;
use log::{info, error};
use serde_json;

#[post("/v1/public/words")]
pub async fn add_word(_word: web::Json<Word>) -> impl Responder {
    // steps to write logic
    // 1- read post data
    // 2- deserialize that data into word struct object
    // 3- check if there is a file with the name of the word in file system
    // 4- if file exists say word already existed
    // 5- if not create a file and store json data in the file

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
                let parts: Vec<&str> = lastline.splitn(3, ',').collect();
                match parts[0].parse::<i32>() {
                    Ok(v) => counter = v + 1,
                    Err(e) => {
                        error!("{:?}",e.to_string());
                        return HttpResponse::BadRequest().body(e.to_string());
                    }
                }
            }
        }
        Err(e) => {
            error!("{:?}",e.to_string());
            return HttpResponse::BadRequest().body(e.to_string());
        }
    }

    let mut word = _word.into_inner(); // 1
    word.last_modified = Utc::now().timestamp(); // update last_modified time
    if word.status == "active" {
        word.status = "inactive".to_string();
    }
    word.id = counter; // add counter to the word

    let word_json = serde_json::to_string(&word).unwrap();

    println!("{}", word_json);
    //let ws: String = String::from("datastore/") + &word.word.clone() + &String::from(".ws");
    let ws: String = String::from("datastore/") + &counter.to_string();

    // match create_file_if_not_exists(&word_json.to_string(), &ws) {
    match create_file_if_not_exists(&word_json.to_string(), &ws) {
        Ok(_) => {
            let counterline: String =
                counter.to_string() + "," + &word.word.clone() + "," + &word.meaning;
            match append_to_the_file(&counterline, "datastore/word.index") {
                Err(e) => {
                    error!("{:?}",e.to_string());
                    return HttpResponse::BadRequest().body(e.to_string());
                }
                Ok(_) => {}
            }

            return HttpResponse::Ok().json(word);
        }
        Err(e) => {
            error!("{:?}",e.to_string());
            return HttpResponse::BadRequest().body(e.to_string());
        }
    }
}

#[get("/v1/public/words/{id}")]
pub async fn get_word(id: web::Path<String>) -> impl Responder {
    //  let js=r#"{"id":1,"word":"Hello","word_type":"expression","meaning":"to say hi","context_clip":null,"image":null,"status":"inactive","last_modified":1690549290}"#;

    // let word:Word= serde_json::from_str(js).unwrap();
    //      info!("{:?}",word);

    let file_path = "datastore/".to_string() + &id;
    info!(
        "{:?}{}",
        "reading the file from the following file path->", file_path
    );
    match read_file(&file_path) {
        Ok(contents) => {
            let word:Result<Word, serde_json::Error> = serde_json::from_str(&contents);
            match word {
                Ok(_word) => HttpResponse::Ok().json(&_word),
                Err(e) => {
                    error!("{:?}",e.to_string());
                     HttpResponse::BadRequest().body(e.to_string())
                }
            }
        }
        Err(e) => {
            error!("{:?}",e.to_string());
             HttpResponse::BadRequest().body(e.to_string())
        }
    }

    // HttpResponse::Ok().body("Hello World")
}


#[delete("/v1/public/words/{id}")]
pub async fn delete_word(id:web::Path<String>)-> impl Responder{
    let file_path = "datastore/".to_string() + &id;
    info!(
        "{:?}{}",
        "reading the file from the following file path->", file_path
    );
    match delete_file(&file_path){
        Ok(_)=>{
            HttpResponse::Accepted().body("file successfully deleted")
        }
        Err(e)=>{
            error!("{:?}",e.to_string());
            HttpResponse::BadRequest().body(e.to_string())
        }
    }
}

#[get("/v1/public/words/")]
pub async fn get_words() -> impl Responder {
    //  let js=r#"{"id":1,"word":"Hello","word_type":"expression","meaning":"to say hi","context_clip":null,"image":null,"status":"inactive","last_modified":1690549290}"#;

    // let word:Word= serde_json::from_str(js).unwrap();
    //      info!("{:?}",word);

    let file_path = "datastore/".to_string();
    info!(
        "{:?}{}",
        "reading the file from the following file path->", file_path
    );
    let mut words:Vec<Word>=Vec::new();
    match read_files_in_directory(&file_path) {
        Ok(wordcontents) => {
           for wordcontent in wordcontents{
            match serde_json::from_str::<Word>(&wordcontent){
                Ok(word)=>{
                    info!("{:?}{:?}{:?}","adding a word",word.word, "to the words vector");
                    words.push(word);
                },
                Err(e)=>{
                    error!("{:?}",e.to_string());
                  // return HttpResponse::BadRequest().body(e.to_string());
                }
            }
           }
           info!("{:?}","successfully words data returned");
           HttpResponse::Ok().json(&words)
        }
        Err(e) => {
            error!("{:?}",e.to_string());
             HttpResponse::BadRequest().body(e.to_string())
        }
    }

    // HttpResponse::Ok().body("Hello World")
}
