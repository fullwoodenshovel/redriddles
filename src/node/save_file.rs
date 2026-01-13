use super::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SaveData { // IF CHANGING THIS BETWEEN VERSIONS, ADD SUPPORT FOR IT

}

const ERROR_MESSAGE: &str = "ERROR WITH SAVE FILE!\n\
    Your save file `save_data.json` is malformed and cannot be parsed.\n\
    Either fix the fix the file, move it elsewhere or delete it.\n\
    Moving / deleting the file will generate a new one.\n\
    If you want an example of what a correctly formatted file looks like,\n\
    move the file and run the code. You can use the generated file for reference.\n\
    After cross referencing, you can put the original back and run again.\n\
    This window does nothing. End the program when you're ready.";

impl SaveData {
    pub fn recover() -> Result<Self, &'static str> {
        todo!()
    }
}

impl Drop for SaveData {
    fn drop(&mut self) {
        
    }
}