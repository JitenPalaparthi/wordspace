use actix_web::{App, HttpServer};
use log::debug;
use log::info;
mod routes;
mod words;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("{:?}","The server started and running on port 8082");

    //let file_content=words::read_last_line_of_file("datastore/word1.index").expect("seems to be error");
    //    let file_content=words::read_last_line_of_file("datastore/word.index");

    //    match file_content{
    //     Ok(s)=>{println!(">>>>>>>{}",s)}
    //     Err(e)=>{println!("XXXXXX--->>>>{}",e.to_string())}
    //    }

    let file_content = words::read_last_line_of_file("datastore/word.index").unwrap();
    info!("->>>>>>>>>>>>{:?}", file_content);
    HttpServer::new(|| App::new().service(routes::hello).service(words::add_word))
        .bind(("0.0.0.0", 8082))?
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
