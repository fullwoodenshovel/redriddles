use crate::{node::user_inputs::{shortcut_to_string, string_to_shortcut}, ui::PixelArray};

use super::*;
use bimap::Overwritten;
use serde::{Serialize, Deserialize};
use serde_json::{from_str, to_string_pretty};
use std::{collections::{BTreeMap, VecDeque}, env, fmt::Display, fs, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct SaveDataStoreV1_0_2 {
    shortcuts: BTreeMap<String, ShortcutInstruction>,
    cached_dirs: VecDeque<PathBuf>
}

#[derive(Serialize, Deserialize)]
pub struct WorkSpaceStore { // IF CHANGING THIS BETWEEN VERSIONS, ADD SUPPORT FOR IT
    saved_cols: Vec<[f32; 4]>,
    current_col: [f32; 4],
    pos: [f32; 2],
    scale: f32,
    cached_dirs: VecDeque<PathBuf>,
}

#[derive(Serialize, Deserialize)]
pub struct SaveDataStore {
    shortcuts: BTreeMap<String, ShortcutInstruction>,
}

#[derive(Default)]
pub struct SaveData {
    pub shortcuts: Shortcuts,
    pub workspaces: Vec<PathBuf>
}

pub struct WorkSpace {
    pixels: Option<PixelArray>,
    saved_cols: Vec<[f32; 4]>,
    current_col: [f32; 4],
    pos: [f32; 2],
    scale: f32,
    cached_dirs: VecDeque<PathBuf>,
}

#[derive(Debug)]
enum Error {
    Syntax {
        details: String,
        malformed_file: String
    },
    Path {
        malformed_file: String
    },
    GenericFile {
        details: String,
        malformed_file: String
    },
    Read {
        details: String,
        malformed_file: String
    },
    Parse {
        details: String,
        malformed_file: String
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Error::Syntax { details, malformed_file } => format!("ERROR WITH SAVE INFORMATION!\n\
                    Your save file `{malformed_file}` is malformed and cannot be parsed. \
                    Either fix the fix the file, move it elsewhere or delete it. \
                    Moving / deleting the file will generate a new one. \
                    If you want an example of what a correctly formatted file looks like, \
                    move the file and run the code. You can use the generated file for reference. \
                    After cross referencing, you can put the original back and run again. \
                    Here's the error:\n\n{details}\n\n{FINAL_MESSAGE}"),
                Error::Path { malformed_file } => format!("ERROR WITH SAVE INFORMATION!\n\
                    The path to the `{malformed_file}` cannot be determined. \
                    This happens either because the executable is in the root, \
                    access is denied or other technical reasons.\n{FINAL_MESSAGE}"),// a file / folder
                Error::GenericFile { details, malformed_file } => format!("ERROR WITH SAVE INFORMATION!\n\
                    Attempting to fetch `{malformed_file}` led to an error. \
                    This could be because of permission errors, \
                    or many other technical reasons. \
                    Here's the error:\n\n{details}\n\n{FINAL_MESSAGE}"),
                Error::Read { details, malformed_file } => format!("ERROR WITH SAVE INFORMATION!\n\
                    Attempting to read `{malformed_file}` led to an error. \
                    This could be because of permission errors, \
                    because the file is corrupt, invalid UTF-8, \
                    or many other technical reasons. \
                    Here's the error:\n\n{details}\n\n{FINAL_MESSAGE}"),
                Error::Parse { details, malformed_file } => format!("ERROR WITH SAVE INFORMATION!\n\
                    Your save file `{malformed_file}` is malformed and cannot be parsed. \
                    Either fix the fix the file, move it elsewhere or delete it. \
                    Moving / deleting the file will generate a new one. \
                    This error is due to logical errors in your file, not syntactical errors. \
                    The reason is below:\n\n{details}\n\n{FINAL_MESSAGE}"),
            }
        )
    }
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        value.to_string()
    }
}

struct ErrorGen {
    file: Option<String>
}

impl ErrorGen {
    fn new() -> Self {
        Self { file: None }
    }

    fn set_file(&mut self, file: String) {
        self.file = Some(file);
    }

    fn reset_file(&mut self) {
        self.file = None;
    }

    fn get_file(&self) -> String {
        self.file.clone().unwrap_or("None".to_string())
    }

    fn syntax_error<T: ToString>(&self, details: T) -> Error {
        Error::Syntax { details: details.to_string(), malformed_file: self.get_file() }
    }

    fn path_error(&self) -> Error {
        Error::Path { malformed_file: self.get_file() }
    }

    fn generic_file_error<T: ToString>(&self, details: T) -> Error {
        Error::GenericFile { details: details.to_string(), malformed_file: self.get_file() }
    }

    fn read_error<T: ToString>(&self, details: T) -> Error {
        Error::Read { details: details.to_string(), malformed_file: self.get_file() }
    }

    fn parse_error<T: ToString>(&self, details: T) -> Error {
        Error::Parse { details: details.to_string(), malformed_file: self.get_file() }
    }
}

const FINAL_MESSAGE: &str = "This window does nothing. End the program when you are ready.";

fn get_save_path(error_gen: &ErrorGen) -> Result<PathBuf, Error> {
    let path = match env::current_exe() {
        Ok(path) => {
            if let Some(path) = path.parent() {
                path.to_owned()
            } else {
                return Err(Error::Path { malformed_file: format!("../{path:?}") })
            }
        },
        Err(err) => return Err(error_gen.generic_file_error(err))
    };
    Ok(path.join("data.json"))
}

fn get_workspaces(error_gen: &ErrorGen) -> Result<Vec<PathBuf>, Error> {
    let path = match env::current_exe() {
        Ok(path) => {
            if let Some(path) = path.parent() {
                path.to_owned()
            } else {
                return Err(Error::Path { malformed_file: format!("../{path:?}") })
            }
        },
        Err(err) => return Err(error_gen.generic_file_error(err))
    };
    let paths = match fs::read_dir(path.join("workspaces")) {
        Ok(paths) => paths,
        Err(err) => return Err(error_gen.generic_file_error(err))
    };
    let mut result = Vec::new();
    for path in paths {
        result.push(path.map_err(|err| error_gen.generic_file_error(err))?.path());
    }
    Ok(result)
}

impl SaveDataStore {
    pub fn recover() -> Result<Option<Self>, String> {
        let error_gen = ErrorGen::new();
        let path = get_save_path(&error_gen)?;

        if match path.try_exists() {
            Ok(exists) => exists,
            Err(err) => return Err(error_gen.generic_file_error(err).to_string())
        } {
            let json = match fs::read_to_string(path) {
                Ok(json) => json,
                Err(err) => return Err(error_gen.read_error(err).to_string())
            };

            match from_str(&json) {
                Ok(result) => Ok(Some(result)),
                Err(err) => Err(error_gen.syntax_error(err).to_string())
            }
        } else {
            Ok(None)
        }
    }

    fn from(value: &mut SaveData) -> Self {
        let shortcuts = std::mem::take(&mut value.shortcuts.shortcuts);
        Self {
            shortcuts: shortcuts.into_iter().map(|(shortcut, instruction)| (shortcut_to_string(&shortcut), instruction)).collect()
        }
    }

    fn into(self) -> Result<SaveData, Error> {
        let mut error_gen = ErrorGen::new();
        
        error_gen.set_file("data.json".to_string());
        let mut shortcuts = bimap::BiMap::new();

        for (shortcut, instruction) in self.shortcuts.into_iter() {
            let shortcut = match string_to_shortcut(&shortcut) {
                Ok(shortcut) => shortcut,
                Err(err) => return Err(error_gen.syntax_error(err))
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
                Overwritten::Both((old_shortcut1, old_instruction1), (old_shortcut2, old_instruction2)) =>
                    Some(format!("Three definitions of the same shortcut `{}` and the same instruction `{}`.\n\
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
                return Err(error_gen.parse_error(reason))
            }
        }

        error_gen.set_file("workspaces/".to_string());
        let workspaces = get_workspaces(&error_gen)?;

        Ok(SaveData {
            shortcuts: Shortcuts {
                shortcuts,
                empty: Vec::new()
            },
            workspaces
        })
    }
}

impl Drop for SaveData {
    fn drop(&mut self) {
        let path = get_save_path(&ErrorGen { file: None }).expect("Unable to access `data.json` on code end, this is unrecoverable.");

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