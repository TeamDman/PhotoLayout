use directories_next::UserDirs;
use eframe::Storage;
use std::path::{Path, PathBuf};

const INPUT_DIR: &str = "last_input_dir";
const OUTPUT_DIR: &str = "output_dir";

#[derive(Default)]
pub(super) struct PickerLocations {
    pub input_dir: Option<PathBuf>,
    pub output_dir: Option<PathBuf>,
}

impl PickerLocations {
    pub fn load(storage: Option<&dyn Storage>) -> Self {
        let read = |key| {
            storage
                .and_then(|storage| storage.get_string(key))
                .map(PathBuf::from)
        };
        Self {
            input_dir: read(INPUT_DIR),
            output_dir: read(OUTPUT_DIR),
        }
    }

    pub fn save(&self, storage: &mut dyn Storage) {
        for (key, path) in [(INPUT_DIR, &self.input_dir), (OUTPUT_DIR, &self.output_dir)] {
            if let Some(path) = path.as_deref().and_then(Path::to_str) {
                storage.set_string(key, path.to_owned());
            }
        }
    }

    pub fn input_dialog(&self) -> rfd::FileDialog {
        Self::dialog(self.input_dir.as_deref(), UserDirs::picture_dir)
    }

    pub fn output_dialog(&self) -> rfd::FileDialog {
        Self::dialog(self.output_dir.as_deref(), UserDirs::download_dir)
    }

    fn dialog(saved: Option<&Path>, default: fn(&UserDirs) -> Option<&Path>) -> rfd::FileDialog {
        let user_dirs = UserDirs::new();
        let directory = saved
            .filter(|path| path.is_dir())
            .or_else(|| {
                user_dirs
                    .as_ref()
                    .and_then(default)
                    .filter(|path| path.is_dir())
            })
            .or_else(|| user_dirs.as_ref().map(UserDirs::home_dir));
        let dialog = rfd::FileDialog::new();
        match directory {
            Some(directory) => dialog.set_directory(directory),
            None => dialog,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Default)]
    struct MemoryStorage(HashMap<String, String>);

    impl Storage for MemoryStorage {
        fn get_string(&self, key: &str) -> Option<String> {
            self.0.get(key).cloned()
        }

        fn set_string(&mut self, key: &str, value: String) {
            self.0.insert(key.into(), value);
        }

        fn flush(&mut self) {}
    }

    #[test]
    fn picker_locations_survive_reload_independently() {
        let mut storage = MemoryStorage::default();
        let mut locations = PickerLocations::load(None);
        locations.input_dir = Some(PathBuf::from("album"));
        locations.save(&mut storage);

        let mut locations = PickerLocations::load(Some(&storage));
        locations.output_dir = Some(PathBuf::from("exports"));
        locations.save(&mut storage);

        let locations = PickerLocations::load(Some(&storage));
        assert_eq!(locations.input_dir, Some(PathBuf::from("album")));
        assert_eq!(locations.output_dir, Some(PathBuf::from("exports")));
    }
}
