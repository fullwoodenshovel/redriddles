use crate::node::user_inputs::{shortcut_to_string, string_to_shortcut};

use super::*;
use bimap::Overwritten;
use serde::{Serialize, Deserialize};
use serde_json::{from_str, to_string_pretty};
use std::{collections::{BTreeMap, VecDeque}, env, fs, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct SaveDataStore { // IF CHANGING THIS BETWEEN VERSIONS, ADD SUPPORT FOR IT
    shortcuts: BTreeMap<String, ShortcutInstruction>,
    cached_dirs: VecDeque<PathBuf>
}

#[derive(Default)]
pub struct SaveData {
    pub shortcuts: Shortcuts,
    pub cached_dirs: VecDeque<PathBuf>
}

const SYNTAX_ERROR: &str = "ERROR WITH SAVE FILE!\n\
    Your save file `save_data.json` is malformed and cannot be parsed.\n\
    Either fix the fix the file, move it elsewhere or delete it.\n\
    Moving / deleting the file will generate a new one.\n\
    If you want an example of what a correctly formatted file looks like,\n\
    move the file and run the code. You can use the generated file for reference.\n\
    After cross referencing, you can put the original back and run again.\n\
    This window does nothing. End the program when you're ready.\n\
    Here's the error:\n\n";
    
    const PATH_ERROR: &str = "ERROR WITH SAVE FILE!\n\
    The path to the save file cannot be determined.\n\
    This happens either because the executable is in the root,\n\
    or other technical reasons.\n\
    This window does nothing. End the program when you're ready.";
    
    const GENERIC_FILE_ERROR: &str = "ERROR WITH SAVE FILE!\n\
    Attempting to fetch a file / folder led to an error.\n\
    This could be because of permission errors,\n\
    This window does nothing. End the program when you're ready.\n\
    or many other technical reasons. Here's the error:\n\n";
    
    const READ_ERROR: &str = "ERROR WITH SAVE FILE!\n\
    Attempting to read `save_data.json1 led to an error.\n\
    This could be because of permission errors,
    because the file is corrupt, invalid UTF-8,
    or many other technical reasons.\n\
    This window does nothing. End the program when you're ready.\n\
    Here's the error:\n\n";
    
    const PARSE_ERROR: &str = "ERROR WITH SAVE FILE!\n\
    Your save file `save_data.json` is malformed and cannot be parsed.\n\
    Either fix the fix the file, move it elsewhere or delete it.\n\
    Moving / deleting the file will generate a new one.\n\
    This error is due to logical errors in your file, not syntactical errors.\n\
    This window does nothing. End the program when you're ready.\n\
    The reason is below:\n\n";

fn get_save_path() -> Result<PathBuf, String> {
    let path = match env::current_exe() {
        Ok(path) => {
            if let Some(path) = path.parent() {
                path.to_owned()
            } else {
                return Err(PATH_ERROR.to_string())
            }
        },
        Err(err) => return Err(format!("{GENERIC_FILE_ERROR}{err}"))
    };
    Ok(path.join("data.json"))
}

impl SaveDataStore {
    pub fn recover() -> Result<Option<Self>, String> {
        let path = get_save_path()?;

        if match path.try_exists() {
            Ok(exists) => exists,
            Err(err) => return Err(format!("{GENERIC_FILE_ERROR}{err}"))
        } {
            let json = match fs::read_to_string(path) {
                Ok(json) => json,
                Err(err) => return Err(format!("{READ_ERROR}{err}"))
            };

            match from_str(&json) {
                Ok(result) => Ok(Some(result)),
                Err(err) => Err(format!("{SYNTAX_ERROR}{err}"))
            }
        } else {
            Ok(None)
        }
    }

    fn from(value: &mut SaveData) -> Self {
        let shortcuts = std::mem::take(&mut value.shortcuts.shortcuts);
        Self {
            shortcuts: shortcuts.into_iter().map(|(shortcut, instruction)| (shortcut_to_string(&shortcut), instruction)).collect(),
            cached_dirs: std::mem::take(&mut value.cached_dirs)
        }
    }

    fn into(self) -> Result<SaveData, String> {
        let mut shortcuts = bimap::BiMap::new();

        for (shortcut, instruction) in self.shortcuts.into_iter() {
            let shortcut = match string_to_shortcut(&shortcut) {
                Ok(shortcut) => shortcut,
                Err(err) => return Err(format!("{SYNTAX_ERROR}{err}"))
            };

            let reason = match shortcuts.insert(shortcut, instruction) {
                Overwritten::Neither => {None},
                Overwritten::Left(old_shortcut, old_instruction) =>
                    Some(format!("Two definitions of the same shortcut `{}`.\n\
                    One specifies instruction `{}`, the other `{}`",
                    shortcut_to_string(&old_shortcut), old_instruction, instruction)
                ),
                Overwritten::Right(old_shortcut, old_instruction) =>
                    Some(format!("Two definitions of the same instruction `{}`.\n\
                    One is for shortcut `{}`, the other `{}`",
                    old_instruction, shortcut_to_string(&old_shortcut), shortcut_to_string(shortcuts.get_by_right(&old_instruction).unwrap()))
                ),
                Overwritten::Pair(old_shortcut, old_instruction) =>
                    Some(format!("Two definitions of the same shortcut - instruction pair.\n\
                    The repeated pair is `{}` -> `{}`",
                    shortcut_to_string(&old_shortcut), old_instruction)
                ),
                Overwritten::Both((old_shortcut1, old_instruction1), (old_shortcut2, old_instruction2)) => // todo!() TODO TEST THIS
                    Some(format!("Two definitions of the same shortcut `{}` and the same instruction `{}`.\n\
                    The following three definitions exist:\n\
                    `{}` -> `{}`\n\
                    `{}` -> `{}`\n\
                    `{}` -> `{}`",
                    shortcut_to_string(&old_shortcut1), old_instruction2,
                    shortcut_to_string(&old_shortcut1), old_instruction2,
                    shortcut_to_string(&old_shortcut1), old_instruction1,
                    shortcut_to_string(&old_shortcut2), old_instruction2,
                )),
            };
            if let Some(reason) = reason {
                return Err(format!("{PARSE_ERROR}{reason}"))
            }
        }

        Ok(SaveData {
            shortcuts: Shortcuts {
                shortcuts,
                empty: Vec::new()
            },
            cached_dirs: self.cached_dirs
        })
    }
}

impl Drop for SaveData {
    fn drop(&mut self) {
        let path = get_save_path().expect("Unable to access `data.json` on code end, this is unrecoverable.");

        let json = to_string_pretty(&SaveDataStore::from(self)).unwrap();
        fs::write(path, json).expect("Unable to write to `data.json` on code end, this is unrecoverable.");
    }
}

impl SaveData {
    pub fn recover() -> Result<Self, String> {
        match SaveDataStore::recover()? {
            Some(result) => Ok(result.into()?),
            None => Ok(Self::default())
        }
    }
}