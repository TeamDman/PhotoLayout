mod batch_progress;
mod photo_layout_app;
mod photo_preview;
mod picker_locations;
mod window_icon;

use photo_layout_app::PhotoLayoutApp;

pub fn run() -> eyre::Result<()> {
    let icon = window_icon::load()?;
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_icon(icon)
            .with_inner_size([1000.0, 580.0])
            .with_min_inner_size([620.0, 440.0]),
        ..Default::default()
    };
    eframe::run_native(
        "PhotoLayout",
        options,
        Box::new(|cc| Ok(Box::new(PhotoLayoutApp::new(cc.storage)))),
    )
    .map_err(|error| eyre::eyre!("{error}"))
}
