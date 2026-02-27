use std::{
    fmt,
    path::{Path, PathBuf},
};

const SMALL_URL: &str = "https://raw.githubusercontent.com/first20hours/google-10000-english/master/google-10000-english-usa-no-swears-short.txt";
const MEDIUM_URL: &str = "https://raw.githubusercontent.com/first20hours/google-10000-english/master/google-10000-english-usa-no-swears-medium.txt";
const LONG_URL: &str = "https://raw.githubusercontent.com/first20hours/google-10000-english/master/google-10000-english-usa-no-swears-long.txt";

pub enum DictSource {
    Predefined(WordSize),
    Url { url: String, filename: String },
    LocalPath(PathBuf),
}

// predefined
pub enum WordSize {
    Short,
    Medium,
    Long,
}

impl DictSource {
    pub fn parse(input: &str) -> Self {
        if input.starts_with("http") {
            let filename = input
                .split('/')
                .last()
                .filter(|s| !s.is_empty())
                .unwrap_or("downloaded_dict.txt")
                .to_string();

            DictSource::Url {
                url: input.to_string(),
                filename,
            }
        } else if Path::new(input).exists() {
            DictSource::LocalPath(PathBuf::from(input))
        } else {
            match input.to_lowercase().as_str() {
                "short" => DictSource::Predefined(WordSize::Short),
                "medium" => DictSource::Predefined(WordSize::Medium),
                "long" => DictSource::Predefined(WordSize::Long),
                _ => panic!("Unknown source: Provide a size, a URL, or a valid file path"),
            }
        }
    }
}

impl WordSize {
    pub fn get_url(self) -> String {
        match self {
            Self::Short => SMALL_URL,
            Self::Medium => MEDIUM_URL,
            Self::Long => LONG_URL,
        }
        .to_string()
    }
}

impl fmt::Display for WordSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WordSize::Short => write!(f, "short"),
            WordSize::Medium => write!(f, "medium"),
            WordSize::Long => write!(f, "long"),
        }
    }
}
