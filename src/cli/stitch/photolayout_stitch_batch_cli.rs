use crate::stitch::StitchRequest;
use facet::Facet;
use figue as args;
use std::path::PathBuf;

#[derive(Facet, Debug)]
pub struct StitchArgs {
    /// Existing directory for the output JPEGs.
    #[facet(args::named)]
    output_dir: PathBuf,

    /// Text inserted between the original file stems.
    #[facet(args::named, default = String::from("+"))]
    separator: String,

    /// Input JPEG or PNG files in pairing order.
    #[facet(args::positional)]
    inputs: Vec<PathBuf>,
}

impl StitchArgs {
    pub fn invoke(self) -> eyre::Result<()> {
        let request = StitchRequest {
            inputs: self.inputs,
            output_dir: self.output_dir,
            separator: self.separator,
        };
        let report = request.run(|_| {})?;
        for output in report.written {
            println!("{}", output.display());
        }
        if let Some(path) = report.unpaired {
            eprintln!("Skipped unpaired input: {}", path.display());
        }
        for failure in &report.failures {
            eprintln!("{failure}");
        }
        eyre::ensure!(
            report.failures.is_empty(),
            "{} pair(s) failed",
            report.failures.len()
        );
        Ok(())
    }
}
