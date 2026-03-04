use crate::dictionary::sources::DictSource;
use dirs;
use rand::seq::SliceRandom;
use std::fs;
use std::path::PathBuf;

mod sources;

const PRACTICE_WORD_MIN_SIZE: usize = 3;
const PRACTICE_WORD_MAX_SIZE: usize = 7;

pub enum WordProvider {
    Dictionary(Vec<String>),
    Practice(Vec<char>),
}

impl WordProvider {
    pub fn get_word(&self, rng: &mut impl rand::Rng) -> String {
        match self {
            WordProvider::Dictionary(words) => words
                .choose(rng)
                .cloned()
                .unwrap_or_else(|| "error".to_string()),
            WordProvider::Practice(chars) => {
                generate_random_word(chars, PRACTICE_WORD_MIN_SIZE, PRACTICE_WORD_MAX_SIZE, rng)
            }
        }
    }
}

pub fn load_dictionary(input: &str, min_word_size: usize, filter: &Option<String>) -> Vec<String> {
    let source = DictSource::parse(input);
    let cache_dir = dirs::cache_dir()
        .expect("Cache dir not found")
        .join(env!("CARGO_PKG_NAME"));

    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).ok();
    }

    let mut dictionary = get_dictionary(&cache_dir, &source);
    filter_dictionary(&mut dictionary, min_word_size, &filter);
    dictionary
}

fn generate_random_word(
    chars: &[char],
    min: usize,
    max: usize,
    rng: &mut impl rand::Rng,
) -> String {
    if chars.is_empty() {
        return String::new();
    }

    let len = rng.gen_range(min..=max);

    (0..len)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

fn fetch_or_cache(url: &String, path: &PathBuf) -> String {
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

fn get_dictionary(cache_dir: &PathBuf, source: &DictSource) -> Vec<String> {
    let (content, _) = match source {
        DictSource::LocalPath(p) => (
            fs::read_to_string(p).expect("Failed to read local file"),
            None,
        ),
        DictSource::Predefined(size) => {
            let path = cache_dir.join(format!("{}.txt", size));
            (fetch_or_cache(&size.get_url(), &path), Some(path))
        }
        DictSource::Url { url, filename } => {
            let path = cache_dir.join(filename);
            (fetch_or_cache(url, &path), Some(path))
        }
    };

    content.lines().map(|s| s.to_string()).collect()
}

fn filter_dictionary(dictionary: &mut Vec<String>, min_word_size: usize, filter: &Option<String>) {
    dictionary.retain(|w| w.len() > min_word_size);

    if let Some(filter) = filter {
        let mut allowed = [false; 256];
        for b in filter.bytes() {
            allowed[b as usize] = true;
        }

        dictionary.retain(|word| word.bytes().all(|b| allowed[b as usize]));
    }
}
