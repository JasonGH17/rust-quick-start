use log::LevelFilter;

#[derive(Clone)]
pub enum Output {
    Stdout,
    File(String),
}

pub struct Config {
    pub(crate) level: LevelFilter,
    pub(crate) output: Output,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            level: LevelFilter::Trace,
            output: Output::Stdout,
        }
    }
}

impl Config {
    pub(crate) fn new(level: LevelFilter, output: Output) -> Self {
        Self { level, output }
    }
}
