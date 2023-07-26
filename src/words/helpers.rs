use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;

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