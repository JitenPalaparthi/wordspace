use actix_web::{App, HttpServer};
use log::info;
use std::env;
mod routes;
mod words;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let port_result = env::var("WORDSPACE_PORT");
    let mut port = String::new();
    match port_result {
        Ok(p) => {
            port = p;
        }
        Err(_) => {
            info!(
                "{}",
                "env WORDSPACE_PORT not found, hence setting the default port"
            );
            port = "8082".to_string();
        }
    }

    info!("{}{}", "The server started and running on port:", &port);

    //let file_content=words::read_last_line_of_file("datastore/word1.index").expect("seems to be error");
    //    let file_content=words::read_last_line_of_file("datastore/word.index");

    //    match file_content{
    //     Ok(s)=>{println!(">>>>>>>{}",s)}
    //     Err(e)=>{println!("XXXXXX--->>>>{}",e.to_string())}
    //    }

    //let file_content = words::read_last_line_of_file("datastore/word.index").unwrap();
    // info!("->>>>>>>>>>>>{:?}", file_content);
    HttpServer::new(|| App::new().service(routes::hello).service(words::add_word))
        .bind("0.0.0.0:".to_string() + &port)?
        .run()
        .await
}

// use actix_web::cookie::time::macros::datetime;
// use words::new_word;
// use chrono::Utc;
// mod words;
// fn main(){
//     let word=new_word(101, "", "EXCLAMATION", "greeting", "", "", "activ",Utc::now().timestamp() as u64 );
//     println!("{:?}",word);
// }
