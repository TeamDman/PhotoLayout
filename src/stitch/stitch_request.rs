use super::{
    StitchReport, image_pair::ImagePair, output_names::OutputNames,
    run_directory::create_run_directory,
};
use eyre::ensure;
use rayon::prelude::*;
use std::{path::PathBuf, sync::Mutex};

#[derive(Debug)]
pub struct StitchRequest {
    pub inputs: Vec<PathBuf>,
    pub output_dir: PathBuf,
    pub separator: String,
}

impl StitchRequest {
    pub fn run(&self, progress: impl FnMut(usize) + Send) -> eyre::Result<StitchReport> {
        ensure!(self.output_dir.is_dir(), "Choose an existing output folder");
        ensure!(self.inputs.len() >= 2, "Add at least two images");
        ensure!(
            !self
                .separator
                .contains(['/', '\\', '<', '>', ':', '"', '|', '?', '*'])
                && !self.separator.chars().any(char::is_control),
            "The separator contains a character that cannot be used in a filename"
        );

        let output_dir = create_run_directory(&self.output_dir)?;
        let pairs = self.inputs.chunks_exact(2);
        let mut report = StitchReport {
            output_dir,
            unpaired: pairs.remainder().first().cloned(),
            ..StitchReport::default()
        };

        let mut names = OutputNames::default();
        let mut jobs: Vec<_> = pairs
            .map(|paths| {
                let pair = ImagePair {
                    top: &paths[0],
                    bottom: &paths[1],
                };
                let filename = pair.filename(&self.separator);
                if let Ok(filename) = &filename {
                    names.reserve(filename);
                }
                (pair, filename)
            })
            .collect();
        // Allocate names in input order before any parallel job starts.
        for (_, filename) in &mut jobs {
            if let Ok(filename) = filename {
                *filename = names.assign(std::mem::take(filename));
            }
        }

        let progress = Mutex::new(progress);
        let outcomes: Vec<_> = jobs
            .into_par_iter()
            .enumerate()
            .map(|(index, (pair, filename))| {
                let result = filename
                    .and_then(|filename| pair.save(&report.output_dir, &filename))
                    .map_err(|error| pair.describe_error(&error));
                progress.lock().expect("progress callback panicked")(index);
                result
            })
            .collect();

        for result in outcomes {
            match result {
                Ok(path) => report.written.push(path),
                Err(error) => report.failures.push(error),
            }
        }
        Ok(report)
    }
}
