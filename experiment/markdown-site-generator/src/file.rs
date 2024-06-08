use std::{fs, io};
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::Read;

pub fn reader(path: &str) -> io::Result<Vec<(String, String)>> {
    let file_paths = file_path_reader(&PathBuf::from(path))?;
    let mut files = vec![];

    for file_path in file_paths {
        let mut f = File::open(&file_path)?;
        let mut buffer = String::new();

        f.read_to_string(&mut buffer)?;
        let stripped_file_path = file_path.as_path().strip_prefix(path).unwrap();
        files.push((stripped_file_path.to_str().unwrap().to_string(), buffer));
    }

    Ok(files)
}

fn file_path_reader(path: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut files = vec![];

    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for entry in &entries {
        if entry.is_dir() {
            files.append(&mut file_path_reader(entry)?);
        } else {
            files.push(entry.clone())
        }
    }

    Ok(files)
}


pub fn writer(base_dir: &str, files: Vec<(String, String)>) -> io::Result<()> {
    for (path, html) in files {
        let mut file_path = PathBuf::from(&path);
        file_path.set_extension("");
        file_path.set_extension("html");

        let write_path = Path::new(base_dir).join(file_path);

        if !write_path.is_file() {
            fs::create_dir_all(write_path.clone().parent().unwrap())?;
        }


        file_writer(&write_path, html).unwrap();
    }

    Ok(())
}

pub fn file_writer(path: &PathBuf, html: String) -> io::Result<()> {
    fs::write(path, html)?;
    Ok(())
}