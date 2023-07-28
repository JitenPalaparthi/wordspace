use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;
use std::io::{self, BufRead};


pub fn save_to_file(data: &str, file_path: &str) -> std::io::Result<()> {
    // Open the file in write mode, creating it if it doesn't exist and truncating (emptying) it if it does.
   
    
    let mut file = File::create(file_path)?;

    // Write the string data to the file.
    file.write_all(data.as_bytes())?;

    Ok(())
}

pub fn create_file_if_not_exists(data: &str,file_path: &str) -> std::io::Result<()> {
    // Open the file with options to create it only if it does not exist
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_path)?;

    // If the file was created successfully or if it already exists, write data to it
    //let data = "This is some sample text.";
    file.write_all(data.as_bytes())?;

    Ok(())
}

pub fn append_to_the_file(data: &str,file_path:&str)-> std::io::Result<()>{
   // Open the file with options to create it only if it does not exist
   let mut file = OpenOptions::new()
   .write(true)
   .append(true)
   .open(file_path)?;
let ldata:String=data.to_string()+"\n";
// If the file was created successfully or if it already exists, write data to it
//let data = "This is some sample text.";
file.write_all(ldata.as_bytes())?;

Ok(())
}


pub fn read_last_line_of_file(file_path:&str)-> std::io::Result<String>{
    let file = File::open(file_path)?;

    let reader=io::BufReader::new(file);
    let mut last_line:String=String::new();
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                last_line=line;
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
    Ok(last_line)
 }