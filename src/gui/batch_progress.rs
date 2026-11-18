use eframe::egui;
use std::{
    fs,
    path::PathBuf,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug)]
pub(super) struct BatchProgress {
    pub pairs_done: usize,
    pub bytes: Option<(u64, u64)>,
    started: Instant,
    sample_elapsed: Duration,
}

impl BatchProgress {
    pub fn pair_sizes(inputs: &[PathBuf]) -> Option<Vec<u64>> {
        inputs
            .chunks_exact(2)
            .map(|pair| {
                let first = fs::metadata(&pair[0]).ok()?.len();
                let second = fs::metadata(&pair[1]).ok()?.len();
                first.checked_add(second)
            })
            .collect()
    }

    pub fn new(pair_sizes: Option<&[u64]>) -> Self {
        let total = pair_sizes.and_then(|sizes| {
            sizes
                .iter()
                .try_fold(0_u64, |sum, size| sum.checked_add(*size))
        });
        Self {
            pairs_done: 0,
            bytes: total.map(|total| (0, total)),
            started: Instant::now(),
            sample_elapsed: Duration::ZERO,
        }
    }

    pub fn complete_pair(&mut self, index: usize, pair_sizes: Option<&[u64]>) {
        self.pairs_done += 1;
        self.sample_elapsed = self.started.elapsed();
        if let (Some((done, _)), Some(sizes)) = (&mut self.bytes, pair_sizes) {
            *done += sizes[index];
        }
    }

    fn estimate(&self, elapsed: Duration) -> Option<(f64, Duration)> {
        let (done, total) = self.bytes?;
        if done == 0 || elapsed.is_zero() {
            return None;
        }
        let rate = done as f64 / elapsed.as_secs_f64();
        let remaining = ((total - done) as f64 / rate).ceil() as u64;
        Some((rate, Duration::from_secs(remaining)))
    }

    pub fn summary(&self, elapsed: Duration, had_failures: bool) -> String {
        let action = if had_failures {
            "Attempted"
        } else {
            "Processed"
        };
        let photos = self.pairs_done * 2;
        let elapsed = format_duration(elapsed);
        match self.bytes {
            Some((done, _)) => format!(
                "{action} {photos} photos ({}) in {elapsed}.",
                format_size(done as f64)
            ),
            None => format!("{action} {photos} photos in {elapsed}. Input size unavailable."),
        }
    }

    pub fn show(&self, ui: &mut egui::Ui, total_pairs: usize) {
        let Some((done, total)) = self.bytes else {
            ui.label(format!(
                "{} of {total_pairs} pairs processed. Input sizes unavailable.",
                self.pairs_done
            ));
            return;
        };
        let fraction = if total == 0 {
            0.0
        } else {
            (done as f64 / total as f64) as f32
        };
        ui.add(egui::ProgressBar::new(fraction).text(format!(
            "{} of {} processed",
            format_size(done as f64),
            format_size(total as f64)
        )));
        ui.horizontal_wrapped(|ui| {
            ui.label(format!("{} remaining", format_size((total - done) as f64)));
            if let Some((rate, eta)) = self.estimate(self.sample_elapsed) {
                ui.separator();
                ui.label(format!("{}/s average", format_size(rate)));
                ui.separator();
                ui.label(format!("ETA {}", format_duration(eta)));
            } else {
                ui.separator();
                ui.label("Estimating throughput and ETA…");
            }
        });
    }
}

fn format_size(mut bytes: f64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];
    let mut unit = 0;
    while bytes >= 1000.0 && unit < units.len() - 1 {
        bytes /= 1000.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes:.0} B")
    } else {
        format!("{bytes:.1} {}", units[unit])
    }
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    match seconds {
        0..60 => format!("{seconds}s"),
        60..3600 => format!("{}m {}s", seconds / 60, seconds % 60),
        3600..86400 => format!("{}h {}m", seconds / 3600, seconds / 60 % 60),
        _ => format!("{}d {}h", seconds / 86400, seconds / 3600 % 24),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eta_weights_pairs_by_input_bytes() {
        let sizes = [5_000, 45_000, 50_000];
        let mut progress = BatchProgress::new(Some(&sizes));
        assert_eq!(progress.estimate(Duration::from_secs(2)), None);

        progress.complete_pair(0, Some(&sizes));
        assert_eq!(progress.bytes, Some((5_000, 100_000)));
        assert_eq!(
            progress.estimate(Duration::from_secs(2)),
            Some((2_500.0, Duration::from_secs(38)))
        );
        assert_eq!(format_size(2_500.0), "2.5 KB");
        assert_eq!(format_duration(Duration::from_secs(95)), "1m 35s");
    }

    #[test]
    fn completion_summary_shows_photos_bytes_and_time() {
        let sizes = [5_000_000; 6];
        let mut progress = BatchProgress::new(Some(&sizes));
        for index in 0..sizes.len() {
            progress.complete_pair(index, Some(&sizes));
        }
        assert_eq!(
            progress.summary(Duration::from_secs(90), false),
            "Processed 12 photos (30.0 MB) in 1m 30s."
        );
    }

    #[test]
    fn byte_progress_accounts_for_out_of_order_completions() {
        let sizes = [5_000, 45_000, 50_000];
        let mut progress = BatchProgress::new(Some(&sizes));
        progress.complete_pair(2, Some(&sizes));
        assert_eq!(progress.pairs_done, 1);
        assert_eq!(progress.bytes, Some((50_000, 100_000)));
        progress.complete_pair(0, Some(&sizes));
        assert_eq!(progress.pairs_done, 2);
        assert_eq!(progress.bytes, Some((55_000, 100_000)));
    }
}
