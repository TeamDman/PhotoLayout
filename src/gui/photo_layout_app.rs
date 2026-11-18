use super::{
    batch_progress::BatchProgress, photo_preview::PhotoPreview, picker_locations::PickerLocations,
};
use crate::stitch::{StitchReport, StitchRequest};
use eframe::egui;
use std::{collections::HashSet, path::PathBuf, sync::mpsc};

struct Batch {
    worker: std::thread::JoinHandle<eyre::Result<(StitchReport, String)>>,
    progress: mpsc::Receiver<BatchProgress>,
    status: Option<BatchProgress>,
    total: usize,
}

pub(super) struct PhotoLayoutApp {
    inputs: Vec<PathBuf>,
    locations: PickerLocations,
    preview: PhotoPreview,
    show_unpaired_alert: bool,
    skipped_duplicates: usize,
    separator: String,
    batch: Option<Batch>,
    messages: Vec<String>,
}

impl PhotoLayoutApp {
    pub fn new(storage: Option<&dyn eframe::Storage>) -> Self {
        Self {
            inputs: Vec::new(),
            locations: PickerLocations::load(storage),
            preview: PhotoPreview::default(),
            show_unpaired_alert: false,
            skipped_duplicates: 0,
            separator: "+".into(),
            batch: None,
            messages: Vec::new(),
        }
    }

    fn add_inputs(&mut self, paths: Vec<PathBuf>) {
        if paths.is_empty() {
            return;
        }
        if let Some(directory) = paths.last().and_then(|path| path.parent()) {
            self.locations.input_dir = Some(directory.to_owned());
        }
        let identity = |path: &PathBuf| path.canonicalize().unwrap_or_else(|_| path.clone());
        let mut seen: HashSet<_> = self.inputs.iter().map(identity).collect();
        for path in paths {
            if seen.insert(identity(&path)) {
                self.inputs.push(path);
            } else {
                self.skipped_duplicates += 1;
            }
        }
    }

    fn begin(&mut self, ctx: &egui::Context) {
        if !self.inputs.len().is_multiple_of(2) {
            self.show_unpaired_alert = true;
        } else if let Some(path) = self.locations.output_dir.clone() {
            self.start(path, ctx.clone());
        } else {
            self.messages = vec!["Choose an output folder before starting.".into()];
        }
    }

    fn start(&mut self, output_dir: PathBuf, ctx: egui::Context) {
        let request = StitchRequest {
            inputs: self.inputs.clone(),
            output_dir,
            separator: self.separator.clone(),
        };
        let total = request.inputs.len() / 2;
        let (sender, progress) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            let started = std::time::Instant::now();
            let pair_sizes = BatchProgress::pair_sizes(&request.inputs);
            let mut status = BatchProgress::new(pair_sizes.as_deref());
            let _ = sender.send(status);
            ctx.request_repaint();
            let result = request.run(|index| {
                status.complete_pair(index, pair_sizes.as_deref());
                let _ = sender.send(status);
                ctx.request_repaint();
            });
            ctx.request_repaint();
            result.map(|report| {
                let summary = status.summary(started.elapsed(), !report.failures.is_empty());
                (report, summary)
            })
        });
        self.messages.clear();
        self.batch = Some(Batch {
            worker,
            progress,
            status: None,
            total,
        });
    }

    fn poll_batch(&mut self, ctx: &egui::Context) {
        let Some(batch) = &mut self.batch else { return };
        if let Some(status) = batch.progress.try_iter().last() {
            batch.status = Some(status);
        }
        if !batch.worker.is_finished() {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
            return;
        }
        let batch = self.batch.take().expect("batch is running");
        match batch.worker.join() {
            Ok(Ok((report, summary))) => {
                self.messages.push(format!(
                    "Wrote {} of {} JPEGs.",
                    report.written.len(),
                    batch.total
                ));

                self.messages.push(summary);
                self.messages
                    .push(format!("Output folder: {}", report.output_dir.display()));
                self.messages.extend(report.failures);
            }
            Ok(Err(error)) => self.messages.push(format!("{error:#}")),
            Err(_) => self
                .messages
                .push("Image processing stopped unexpectedly.".into()),
        }
    }
}

impl eframe::App for PhotoLayoutApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.locations.save(storage);
    }

    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        self.poll_batch(ctx);
        self.preview.poll(ctx);
        let idle = self.batch.is_none();
        if idle {
            self.add_inputs(ctx.input(|input| {
                input
                    .raw
                    .dropped_files
                    .iter()
                    .filter_map(|file| file.path.clone())
                    .collect::<Vec<_>>()
            }));
        }
        egui::SidePanel::right("preview")
            .default_width(340.0)
            .width_range(200.0..=500.0)
            .show(ctx, |ui| {
                ui.heading("Preview");
                self.preview.show(ui);
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PhotoLayout");
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add_enabled(idle, egui::Button::new("Choose output folder"))
                    .clicked()
                    && let Some(path) = self.locations.output_dialog().pick_folder()
                {
                    self.locations.output_dir = Some(path);
                }
                if let Some(path) = &self.locations.output_dir {
                    if ui.button("Open folder").clicked()
                        && let Err(error) = open::that_detached(path)
                    {
                        self.messages = vec![format!("Could not open {}: {error}", path.display())];
                    }
                    ui.label(path.display().to_string());
                }
            });
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(idle, egui::Button::new("Add files"))
                    .clicked()
                    && let Some(paths) = self
                        .locations
                        .input_dialog()
                        .add_filter("Images", &["jpg", "jpeg", "png"])
                        .pick_files()
                {
                    self.add_inputs(paths);
                }
                if ui
                    .add_enabled(idle && !self.inputs.is_empty(), egui::Button::new("Clear"))
                    .clicked()
                {
                    self.inputs.clear();
                    self.preview.clear();
                }
                ui.label("Filename separator");
                ui.add_enabled(
                    idle,
                    egui::TextEdit::singleline(&mut self.separator).desired_width(80.0),
                );
            });
            ui.separator();
            if self.inputs.is_empty() {
                ui.label("Drop JPEG or PNG files here, or use Add files.");
            }
            let mut remove = None;
            egui::ScrollArea::both()
                .id_salt("inputs")
                .auto_shrink([false, true])
                .max_height(260.0)
                .show(ui, |ui| {
                    for (index, path) in self.inputs.iter().enumerate() {
                        ui.horizontal(|ui| {
                            if ui.add_enabled(idle, egui::Button::new("Remove")).clicked() {
                                remove = Some(index);
                            }
                            let position = if index % 2 == 0 { "top" } else { "bottom" };
                            let selected = self.preview.selected() == Some(path.as_path());
                            let label = format!("{} {position}  {}", index / 2 + 1, path.display());
                            if ui
                                .add(
                                    egui::Button::selectable(selected, label)
                                        .wrap_mode(egui::TextWrapMode::Extend),
                                )
                                .clicked()
                                && !selected
                            {
                                self.preview.select(path, ctx);
                            }
                        });
                    }
                });
            if let Some(index) = remove {
                let removed = self.inputs.remove(index);
                if !self.inputs.contains(&removed)
                    && self.preview.selected() == Some(removed.as_path())
                {
                    self.preview.clear();
                }
            }
            ui.separator();
            if ui
                .add_enabled(idle && !self.inputs.is_empty(), egui::Button::new("Begin"))
                .clicked()
            {
                self.begin(ctx);
            }
            if let Some(batch) = &self.batch {
                if let Some(status) = &batch.status {
                    status.show(ui, batch.total);
                } else {
                    ui.label("Reading input sizes…");
                }
            }
            egui::ScrollArea::vertical()
                .id_salt("results")
                .show(ui, |ui| {
                    for message in &self.messages {
                        ui.label(message);
                    }
                });
        });
        if self.show_unpaired_alert {
            let response = egui::Modal::new(egui::Id::new("unpaired_photo")).show(ctx, |ui| {
                ui.heading("Uneven number of photos");
                ui.label(format!("You selected {} photos. Add or remove one photo so every photo has a partner. No JPEGs were written.", self.inputs.len()));
                ui.button("OK").clicked()
            });
            if response.inner || response.should_close() {
                self.show_unpaired_alert = false;
            }
        } else if self.skipped_duplicates > 0 {
            let response = egui::Modal::new(egui::Id::new("duplicate_photos")).show(ctx, |ui| {
                ui.heading("Duplicates skipped");
                let count = self.skipped_duplicates;
                let message = if count == 1 {
                    "Skipped 1 file that was already in the list.".to_owned()
                } else {
                    format!("Skipped {count} files that were already in the list.")
                };
                ui.label(message);
                ui.button("OK").clicked()
            });
            if response.inner || response.should_close() {
                self.skipped_duplicates = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn begin_alerts_before_starting_an_odd_batch() {
        let output = tempfile::tempdir().unwrap();
        for count in [1, 3] {
            let mut app = PhotoLayoutApp::new(None);
            app.locations.output_dir = Some(output.path().to_owned());
            app.inputs = (0..count)
                .map(|n| PathBuf::from(format!("{n}.png")))
                .collect();

            app.begin(&egui::Context::default());

            assert!(app.show_unpaired_alert);
            assert!(app.batch.is_none());
            assert_eq!(output.path().read_dir().unwrap().count(), 0);
        }
    }

    #[test]
    fn adding_photos_skips_repeated_paths_and_keeps_distinct_photos_in_order() {
        let dir = tempfile::tempdir().unwrap();
        let other_dir = dir.path().join("other");
        std::fs::create_dir(&other_dir).unwrap();
        let first = dir.path().join("photo.png");
        let second = other_dir.join("photo.png");
        std::fs::write(&first, []).unwrap();
        std::fs::write(&second, []).unwrap();

        let mut app = PhotoLayoutApp::new(None);
        app.add_inputs(vec![first.clone()]);
        assert_eq!(app.skipped_duplicates, 0);
        app.add_inputs(vec![
            other_dir.join("..").join("photo.png"),
            second.clone(),
            second.clone(),
        ]);
        app.add_inputs(Vec::new());

        assert_eq!(app.inputs, [first, second]);
        assert_eq!(app.skipped_duplicates, 2);
    }
}
