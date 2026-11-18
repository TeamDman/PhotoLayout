use eyre::WrapErr;
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

pub(super) fn create_run_directory(parent: &Path) -> eyre::Result<PathBuf> {
    let mut highest = 0_u64;
    for entry in
        fs::read_dir(parent).wrap_err_with(|| format!("Could not read {}", parent.display()))?
    {
        if let Some(number) = entry?
            .file_name()
            .to_str()
            .and_then(|name| name.parse::<u64>().ok())
        {
            highest = highest.max(number);
        }
    }
    loop {
        highest = highest
            .checked_add(1)
            .ok_or_else(|| eyre::eyre!("No run numbers remain"))?;
        let path = parent.join(highest.to_string());
        match fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(error).wrap_err_with(|| format!("Could not create {}", path.display()));
            }
        }
    }
}
