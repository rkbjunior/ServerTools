use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/scripts/<sub_folder>/<file..>")]
pub fn file(file: PathBuf, sub_folder: String) -> Option<NamedFile> {
	let path = format!("scripts/{}/", sub_folder);
	NamedFile::open(Path::new(&path).join(file)).ok()
}