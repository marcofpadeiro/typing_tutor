use crate::dictionary::sources::DictSource;
use dirs;
use std::fs;
use std::path::PathBuf;

mod sources;

#[allow(dead_code)]
pub const QUEUE_SIZE: usize = 2;
pub const MIN_WORD_SIZE: usize = 2;

#[allow(dead_code)]
pub fn load_dictionary(input: &str, filter: Option<String>) -> Vec<String> {
    let source = DictSource::parse(input);
    let cache_dir = dirs::cache_dir()
        .expect("Cache dir not found")
        .join("typing_tutor");

    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).ok();
    }

    let (content, _) = match source {
        DictSource::LocalPath(p) => (
            fs::read_to_string(p).expect("Failed to read local file"),
            None,
        ),
        DictSource::Predefined(size) => {
            let path = cache_dir.join(format!("{}.txt", size));
            (fetch_or_cache(size.get_url(), &path), Some(path))
        }
        DictSource::Url { url, filename } => {
            let path = cache_dir.join(filename);
            (fetch_or_cache(url, &path), Some(path))
        }
    };

    let mut dictionary: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    dictionary.retain(|w| w.len() > MIN_WORD_SIZE);

    if let Some(filter) = filter {
        let mut allowed = [false; 256];
        for b in filter.bytes() {
            allowed[b as usize] = true;
        }

        dictionary.retain(|word| word.bytes().all(|b| allowed[b as usize]));
    }
    dictionary
}

#[allow(dead_code)]
fn fetch_or_cache(url: String, path: &PathBuf) -> String {
    if path.exists() {
        return fs::read_to_string(path).expect("Read failed");
    }

    println!("Downloading dictionary from {}", url);
    let response = reqwest::blocking::get(url)
        .expect("Network request failed")
        .text()
        .expect("Failed to parse body");

    fs::write(path, &response).ok();
    response
}
