use clap::Parser;
use clack::GameMode;

/// a minimalist terminal typing tutor
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// timer: fixed time, variable words | words: fixed words, variable time
    #[arg(short, long, value_enum, default_value_t = GameMode::Timer)]
    pub mode: GameMode,

    /// time limit in seconds or total word count, depending on mode
    #[arg(short, long, default_value_t = 30)]
    pub quantity: u64,

    /// predefined sets (small, medium, long), a local file path, or a url
    #[arg(short, long, default_value = "medium")]
    pub dictionary: String,

    /// restrict dictionary to words containing only these characters
    #[arg(short, long)]
    pub filter: Option<String>,

    /// number of upcoming words to display in the queue
    #[arg(short, long, default_value_t = 2)]
    pub word_preview: u64,

    /// exclude words shorter than this length
    #[arg(short = 's', long, default_value_t = 2)]
    pub min_word_size: u64,
}
