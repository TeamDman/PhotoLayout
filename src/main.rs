mod cli;
mod gui;
mod stitch;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    cli::PhotoLayoutCli::run()
}
