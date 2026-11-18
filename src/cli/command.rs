use super::{gui::GuiArgs, stitch::StitchArgs};
use facet::Facet;

#[derive(Facet, Debug)]
#[repr(u8)]
pub enum Command {
    /// Open the desktop application.
    Gui(GuiArgs),
    /// Combine consecutive pairs of photos.
    Stitch(StitchArgs),
}
