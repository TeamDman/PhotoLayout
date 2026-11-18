use eframe::egui;
use std::{
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver, TryRecvError},
};

#[derive(Default)]
pub(super) struct PhotoPreview {
    selected: Option<PathBuf>,
    loading: Option<(PathBuf, Receiver<eyre::Result<egui::ColorImage>>)>,
    texture: Option<eyre::Result<egui::TextureHandle>>,
}

impl PhotoPreview {
    pub fn selected(&self) -> Option<&Path> {
        self.selected.as_deref()
    }

    pub fn select(&mut self, path: &Path, ctx: &egui::Context) {
        self.selected = Some(path.to_owned());
        self.texture = None;
        self.start_loading(ctx);
    }

    pub fn clear(&mut self) {
        self.selected = None;
        self.texture = None;
    }

    fn start_loading(&mut self, ctx: &egui::Context) {
        if self.loading.is_some() || self.texture.is_some() {
            return;
        }
        let Some(path) = self.selected.clone() else {
            return;
        };
        let (sender, receiver) = mpsc::channel();
        let worker_path = path.clone();
        let ctx = ctx.clone();
        match std::thread::Builder::new().spawn(move || {
            let _ = sender.send(Self::load(&worker_path));
            ctx.request_repaint();
        }) {
            Ok(_) => self.loading = Some((path, receiver)),
            Err(error) => self.texture = Some(Err(error.into())),
        }
    }

    pub fn poll(&mut self, ctx: &egui::Context) {
        let Some((path, receiver)) = &self.loading else {
            return;
        };
        let result = match receiver.try_recv() {
            Ok(result) => result,
            Err(TryRecvError::Empty) => return,
            Err(TryRecvError::Disconnected) => {
                Err(eyre::eyre!("Preview loading stopped unexpectedly"))
            }
        };
        if self.selected.as_ref() == Some(path) {
            self.texture = Some(result.map(|image| {
                ctx.load_texture("selected photo", image, egui::TextureOptions::LINEAR)
            }));
        }
        self.loading = None;
        self.start_loading(ctx);
    }

    fn load(path: &Path) -> eyre::Result<egui::ColorImage> {
        let mut image = image::ImageReader::open(path)?
            .with_guessed_format()?
            .decode()?;
        if image.width() > 1024 || image.height() > 1024 {
            image = image.thumbnail(1024, 1024);
        }
        let pixels = image.into_rgba8();
        let size = [pixels.width() as usize, pixels.height() as usize];
        Ok(egui::ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_raw(),
        ))
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        let Some(path) = &self.selected else {
            ui.label("Select a filename to preview the photo.");
            return;
        };
        ui.label(
            path.file_name()
                .unwrap_or(path.as_os_str())
                .to_string_lossy(),
        );
        match &self.texture {
            Some(Ok(texture)) => {
                let size = texture.size_vec2();
                let available = ui.available_size();
                let scale = (available.x / size.x).min(available.y / size.y);
                if scale > 0.0 {
                    ui.image((texture.id(), size * scale));
                }
            }
            Some(Err(error)) => {
                ui.label(format!("Could not preview this photo: {error}"));
            }
            None => {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Loading preview…");
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_old_preview_is_discarded_after_selection_changes() {
        let ctx = egui::Context::default();
        let dir = tempfile::tempdir().unwrap();
        let old = dir.path().join("old.png");
        let latest = dir.path().join("latest.png");
        let (sender, receiver) = mpsc::channel();
        let mut preview = PhotoPreview {
            selected: Some(old.clone()),
            loading: Some((old, receiver)),
            texture: None,
        };

        preview.select(&latest, &ctx);
        preview.poll(&ctx);
        assert!(preview.texture.is_none());

        sender
            .send(Ok(egui::ColorImage::filled([1, 1], egui::Color32::RED)))
            .unwrap();
        preview.poll(&ctx);
        assert!(preview.texture.is_none());
        assert_eq!(
            preview.loading.as_ref().map(|(path, _)| path),
            Some(&latest)
        );
    }
}
