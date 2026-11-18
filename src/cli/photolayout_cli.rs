use super::{command::Command, gui::GuiArgs};
use facet::Facet;
use figue::{self as args, FigueBuiltins};

/// Combine pairs of photos into printable JPEGs.
#[derive(Facet, Debug)]
pub struct PhotoLayoutCli {
    #[facet(flatten)]
    builtins: FigueBuiltins,

    #[facet(args::subcommand)]
    command: Option<Command>,
}

impl PhotoLayoutCli {
    pub fn run() -> eyre::Result<()> {
        let cli: Self = figue::Driver::new(
            figue::builder::<Self>()
                .expect("valid CLI schema")
                .cli(|cli| cli.args_os(std::env::args_os().skip(1)).strict())
                .help(|help| help.version(env!("CARGO_PKG_VERSION")))
                .build(),
        )
        .run()
        .unwrap();
        match cli.command.unwrap_or(Command::Gui(GuiArgs {})) {
            Command::Gui(args) => args.invoke(),
            Command::Stitch(args) => args.invoke(),
        }
    }
}
