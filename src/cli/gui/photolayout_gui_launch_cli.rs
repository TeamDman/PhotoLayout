use facet::Facet;

#[derive(Facet, Debug)]
pub struct GuiArgs {}

impl GuiArgs {
    pub fn invoke(self) -> eyre::Result<()> {
        crate::gui::run()
    }
}
