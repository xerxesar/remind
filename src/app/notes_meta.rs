use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NoteMeta {
    pub title: String,
    pub pos: Vec<usize>,
    pub size: Vec<usize>,
    pub z_index: usize
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NotesMetaData {
    pub notes: Vec<NoteMeta>
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NotesMeta {
    dir_path: String,
    file_name: String,
    file_path: String,
    pub data: NotesMetaData
}

impl NotesMeta{
    pub fn new() -> NotesMeta {
        let home_dir_path = home::home_dir()
        .expect("Failed to get home_dir!");

        let meta_dir_path: String = format!("{}/{}",home_dir_path.display(),".remind");
        let meta_file_name: String = String::from("notes_meta.json");
        let meta_file_path: String = format!("{}/{}", meta_dir_path, "notes_meta.json");

        NotesMeta {
             dir_path: meta_dir_path,
             file_name: meta_file_name,
             file_path: meta_file_path,
             data: NotesMetaData {
                notes: vec![] 
             }
        }
    }

    pub fn load_notes_meta(&mut self) -> &mut NotesMeta {
        let meta_str = self.read_meta_file();
        let data: NotesMetaData = serde_json::from_str(meta_str.as_str()).expect("Failed to parse json string!");
        self.data.notes = data.notes;
        return self;
    }
    pub fn save_notes_meta(&mut self) {
        let json = serde_json::to_string_pretty(&self.data).expect("Failed to stringify note metas!");
        println!("{}", json);
        self.write_to_meta_file(json)
    }
    fn read_meta_file(&mut self) -> String {
        match fs::read_to_string(self.file_path.to_owned()) {
            Ok(str) => str.to_string(),
            Err(_) => {
                fs::create_dir(self.dir_path.to_owned()).expect(format!("Failed to create remind home directory at {}", self.dir_path).as_str());
                fs::write(self.file_path.to_owned(), "{\"notes\":[]}").expect("Failed to initialize meta file!");
                self.data.notes = vec![];
                return serde_json::to_string_pretty(&self.data).unwrap();
            }
        }
    }
    fn write_to_meta_file(&mut self, content: String) {
        fs::write(self.file_path.to_owned(), content).expect("Failed to write meta info to meta file!");
    }

}