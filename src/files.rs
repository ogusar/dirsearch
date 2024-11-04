use std::fs;

pub fn get_pdf_files(folder_path: &String) -> Box<[String]>{
    let files = fs::read_dir(folder_path).unwrap();
    files
        .filter(|file| !file.is_err())
        .map(|file| file.unwrap().file_name().into_string().unwrap())
        .filter(|file| file.ends_with(".pdf"))
        .collect()
}