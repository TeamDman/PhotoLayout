use eframe::egui::IconData;

#[cfg(not(windows))]
pub(super) fn load() -> eyre::Result<IconData> {
    Ok(eframe::icon_data::from_png_bytes(include_bytes!(
        "../../resources/photolayout.png"
    ))?)
}

#[cfg(windows)]
pub(super) fn load() -> eyre::Result<IconData> {
    use eyre::{WrapErr, ensure};
    use std::{io, ptr, slice};
    use windows_sys::Win32::{
        System::LibraryLoader::{
            FindResourceW, GetModuleHandleW, LoadResource, LockResource, SizeofResource,
        },
        UI::WindowsAndMessaging::{
            LR_DEFAULTCOLOR, LookupIconIdFromDirectoryEx, RT_GROUP_ICON, RT_ICON,
        },
    };

    const GROUP_ID: usize = 1;

    let icon = unsafe {
        // The current executable owns these resources for the lifetime of the process.
        let module = GetModuleHandleW(ptr::null());
        if module.is_null() {
            return Err(io::Error::last_os_error())
                .wrap_err("Could not locate the executable module");
        }
        let group = FindResourceW(module, GROUP_ID as *const u16, RT_GROUP_ICON);
        if group.is_null() {
            return Err(io::Error::last_os_error())
                .wrap_err("Could not find the PhotoLayout icon group");
        }
        let group_data = LockResource(LoadResource(module, group));
        ensure!(
            !group_data.is_null(),
            "Could not load the PhotoLayout icon group"
        );

        // The resource compiler packs the generated PNG frames into the icon group.
        let id = LookupIconIdFromDirectoryEx(group_data.cast(), 1, 256, 256, LR_DEFAULTCOLOR);
        ensure!(id > 0, "Could not select a PhotoLayout icon frame");
        let frame = FindResourceW(module, id as usize as *const u16, RT_ICON);
        if frame.is_null() {
            return Err(io::Error::last_os_error())
                .wrap_err("Could not find the PhotoLayout icon frame");
        }
        let size = SizeofResource(module, frame);
        let data = LockResource(LoadResource(module, frame));
        ensure!(
            size > 0 && !data.is_null(),
            "Could not load the PhotoLayout icon frame"
        );
        slice::from_raw_parts(data.cast::<u8>(), size as usize)
    };
    eframe::icon_data::from_png_bytes(icon).wrap_err("Could not decode the PhotoLayout icon")
}

#[cfg(test)]
mod tests {
    #[test]
    fn loads_the_window_icon() {
        let icon = super::load().unwrap();
        assert_eq!((icon.width, icon.height), (256, 256));
        assert_eq!(icon.rgba.len(), 256 * 256 * 4);
    }
}
