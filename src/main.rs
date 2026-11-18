#![cfg_attr(windows, windows_subsystem = "windows")]

mod cli;
mod gui;
mod stitch;

fn main() -> eyre::Result<()> {
    #[cfg(windows)]
    if std::env::args_os().nth(1).is_some() {
        attach_parent_console();
    }

    color_eyre::install()?;
    cli::PhotoLayoutCli::run()
}

#[cfg(windows)]
fn attach_parent_console() {
    use windows_sys::Win32::{
        Foundation::INVALID_HANDLE_VALUE,
        System::Console::{
            ATTACH_PARENT_PROCESS, AttachConsole, GetStdHandle, STD_ERROR_HANDLE, STD_INPUT_HANDLE,
            STD_OUTPUT_HANDLE, SetStdHandle,
        },
    };

    // Keep inherited pipes and redirected files while filling in missing console handles.
    unsafe {
        let handles = [STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, STD_ERROR_HANDLE]
            .map(|kind| (kind, GetStdHandle(kind)));
        let valid = |handle: windows_sys::Win32::Foundation::HANDLE| {
            !handle.is_null() && handle != INVALID_HANDLE_VALUE
        };
        if handles.iter().any(|&(_, handle)| !valid(handle))
            && AttachConsole(ATTACH_PARENT_PROCESS) != 0
        {
            for (kind, handle) in handles {
                if valid(handle) {
                    SetStdHandle(kind, handle);
                }
            }
        }
    }
}
