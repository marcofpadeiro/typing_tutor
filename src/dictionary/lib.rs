use dirs;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[allow(dead_code)]
pub const QUEUE_SIZE: usize = 2;

fn get_cache_path() -> PathBuf {
    let mut path = dirs::cache_dir().expect("Could not find cache directory");
    path.push("type_tutor");

    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create cache directory");
    }

    path.push("dictionary.txt");
    path
}

fn load_dictionary(size_key: &str) -> Vec<String> {
    let mut path = dirs::cache_dir().expect("Cache dir not found");
    path.push("type_tutor");
    if !path.exists() {
        fs::create_dir_all(&path).ok();
    }

    // Create a unique filename for each mapping: dictionary_10k.txt
    let file_name = format!("dictionary_{}.txt", size_key);
    path.push(file_name);

    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .collect();
    }

    // Download logic
    let url = get_dict_url(size_key);
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();

    fs::write(&path, &response).ok();

    response.lines().map(|s| s.to_string()).collect()
}
