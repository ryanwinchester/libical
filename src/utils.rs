use std::path::{Path,PathBuf};
use std::io::prelude::*;
use std::fs;
use std::io;
use std::iter;

pub fn file_iter(dir: &Path) -> Box<Iterator<Item = PathBuf>> {
  if let Ok(entries) = fs::read_dir(dir) {
      let valid_entries = entries.filter(|x| x.is_ok());
      let extracted_paths = valid_entries.map(move |x| x.unwrap().path());
      Box::new(extracted_paths)
  } else {
      Box::new(iter::empty())
  }
}

pub fn vec_from_string(str: String) -> Vec<String> {
  let mut vec: Vec<String> = Vec::new();
  vec.push(str);
  vec
}

pub fn write_file(filename: &String, contents: String) -> Result<(), io::Error> {
  let mut filepath: String = "Index/".to_owned();
  filepath.push_str(&filename);
  let mut file = fs::File::create(filepath)?;
  file.write_all(contents.as_bytes())?;
  Ok(())
}

pub fn read_file_to_string(path: &Path) -> Result<String, String> {
  if let Ok(mut file) = fs::File::open(&path) {
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_ok() {
      Ok(contents)
    } else {
      //println!("something went wrong reading the file");
      Err("something went wrong reading the file".to_string())
    }
  } else {
    //println!("could not open {} for reading", path.display());
    Err(format!("could not open {} for reading", path.display()))
  }
}

