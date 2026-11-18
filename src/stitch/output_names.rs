use std::{
    collections::{HashMap, HashSet},
    ffi::{OsStr, OsString},
    path::Path,
};

#[derive(Default)]
pub(super) struct OutputNames {
    reserved: HashSet<OsString>,
    next_suffix: HashMap<OsString, u64>,
}

impl OutputNames {
    pub fn reserve(&mut self, name: &OsStr) {
        self.reserved.insert(Self::key(name));
    }

    pub fn assign(&mut self, name: OsString) -> OsString {
        let suffix = self.next_suffix.entry(Self::key(&name)).or_insert(0);
        if *suffix == 0 {
            *suffix = 1;
            return name;
        }
        let stem = Path::new(&name)
            .file_stem()
            .expect("JPEG filename has a stem");
        loop {
            let mut candidate = stem.to_os_string();
            candidate.push(format!("_{suffix}.jpg"));
            *suffix += 1;
            if self.reserved.insert(Self::key(&candidate)) {
                return candidate;
            }
        }
    }

    fn key(name: &OsStr) -> OsString {
        if cfg!(windows) || cfg!(target_os = "macos") {
            name.to_string_lossy().to_uppercase().into()
        } else {
            name.to_owned()
        }
    }
}
