use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct StitchReport {
    pub output_dir: PathBuf,
    pub written: Vec<PathBuf>,
    pub failures: Vec<String>,
    pub unpaired: Option<PathBuf>,
}
