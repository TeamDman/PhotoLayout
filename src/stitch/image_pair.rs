use eyre::{WrapErr, eyre};
use image::{DynamicImage, RgbaImage, codecs::jpeg::JpegEncoder, imageops};
use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

const WIDTH: u32 = 1200;
const HALF_HEIGHT: u32 = 900;

#[derive(Debug)]
pub(super) struct ImagePair<'a> {
    pub top: &'a Path,
    pub bottom: &'a Path,
}

impl ImagePair<'_> {
    pub fn filename(&self, separator: &str) -> eyre::Result<OsString> {
        let mut filename = OsString::new();
        filename.push(
            self.top
                .file_stem()
                .ok_or_else(|| eyre!("Missing first filename"))?,
        );
        filename.push(separator);
        filename.push(
            self.bottom
                .file_stem()
                .ok_or_else(|| eyre!("Missing second filename"))?,
        );
        filename.push(".jpg");
        Ok(filename)
    }

    pub fn describe_error(&self, error: &eyre::Report) -> String {
        format!(
            "{} + {}: {error:#}",
            self.top.display(),
            self.bottom.display()
        )
    }

    pub fn save(&self, directory: &Path, filename: &OsStr) -> eyre::Result<PathBuf> {
        let (top, bottom) = rayon::join(
            || Self::load_panel(self.top),
            || Self::load_panel(self.bottom),
        );
        let output = directory.join(filename);
        let mut canvas = RgbaImage::from_pixel(WIDTH, 2 * HALF_HEIGHT, image::Rgba([0, 0, 0, 255]));
        imageops::overlay(&mut canvas, &top?, 0, 0);
        imageops::overlay(&mut canvas, &bottom?, 0, i64::from(HALF_HEIGHT));

        let mut file = tempfile::NamedTempFile::new_in(directory)?;
        JpegEncoder::new_with_quality(file.as_file_mut(), 75)
            .encode_image(&DynamicImage::ImageRgba8(canvas).into_rgb8())?;
        file.persist_noclobber(&output)
            .wrap_err_with(|| format!("Could not create {}", output.display()))?;
        Ok(output)
    }

    fn load_panel(path: &Path) -> eyre::Result<RgbaImage> {
        let mut image = image::ImageReader::open(path)?
            .with_guessed_format()?
            .decode()
            .wrap_err_with(|| format!("Could not decode {}", path.display()))?;
        if image.height() > image.width() {
            image = image.rotate90();
        }
        Ok(image
            .resize_to_fill(WIDTH, HALF_HEIGHT, imageops::FilterType::Triangle)
            .into_rgba8())
    }
}
