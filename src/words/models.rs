use super::CustomError;
use serde::{Deserialize,Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    pub word: String,
    pub word_type: Option<String>,
    pub meaning: String,
    pub context_clip: Option<String>, //url
    pub image: Option<String>,        //url
    pub status: String,
    pub last_modified: i64, //unix time
}

pub fn new_word(
    _word: &str,
    _word_type: &str,
    _meaning: &str,
    _context_clip: &str,
    _image: &str,
    _status: &str,
    _last_modified: i64)-> Result<Word,CustomError> {
        if _word==""{
            return Err(CustomError{message:"invalid word".to_string(),})
        }
        let word= Word {
                        word: _word.to_string(),
                        word_type: Some(_word_type.to_string()),
                        meaning: _meaning.to_string(),
                        context_clip: Some(_context_clip.to_string()),
                        image: Some(_image.to_string()),
                        status: _status.to_string(),
                        last_modified: _last_modified,
                    };
                    Ok(word)
}


