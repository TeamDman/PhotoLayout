use std::{
    fs,
    process::{Command, Output},
};

use image::{Rgb, RgbImage};

#[test]
fn stitches_in_order_and_preserves_existing_output() {
    let dir = tempfile::tempdir().unwrap();
    let top = dir.path().join("portrait.png");
    let bottom = dir.path().join("landscape.png");
    let extra = dir.path().join("extra.png");
    let mut portrait = RgbImage::from_pixel(30, 40, Rgb([240, 0, 0]));
    for (_, y, pixel) in portrait.enumerate_pixels_mut() {
        if y >= 20 {
            *pixel = Rgb([0, 240, 0]);
        }
    }
    portrait.save(&top).unwrap();
    RgbImage::from_pixel(40, 30, Rgb([0, 0, 240]))
        .save(&bottom)
        .unwrap();
    let run = || -> Output {
        Command::new(env!("CARGO_BIN_EXE_photolayout"))
            .args(["stitch", "--output-dir"])
            .arg(dir.path())
            .args([&top, &bottom, &extra])
            .output()
            .unwrap()
    };

    let root_file = dir.path().join("portrait+landscape.jpg");
    fs::write(&root_file, b"existing output").unwrap();
    let result = run();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(String::from_utf8_lossy(&result.stderr).contains("Skipped unpaired input"));
    let path = dir.path().join("1").join("portrait+landscape.jpg");
    let original = fs::read(&path).unwrap();
    let jpeg = image::open(&path).unwrap().into_rgb8();
    assert_eq!(jpeg.dimensions(), (1200, 1800));
    assert!(jpeg.get_pixel(100, 400)[1] > 220);
    assert!(jpeg.get_pixel(1100, 400)[0] > 220);
    assert!(jpeg.get_pixel(600, 1300)[2] > 220);

    fs::create_dir(dir.path().join("4")).unwrap();
    assert!(run().status.success());
    assert!(
        dir.path()
            .join("5")
            .join("portrait+landscape.jpg")
            .is_file()
    );
    assert_eq!(fs::read(root_file).unwrap(), b"existing output");
    assert_eq!(fs::read(path).unwrap(), original);
}

#[test]
fn parallel_pairs_get_unique_names_for_same_named_inputs() {
    let dir = tempfile::tempdir().unwrap();
    let paths: Vec<_> = (0..4)
        .map(|index| {
            let folder = dir.path().join(format!("source-{index}"));
            fs::create_dir(&folder).unwrap();
            let path = folder.join("image.png");
            let color = if index < 2 {
                Rgb([240, 0, 0])
            } else {
                Rgb([0, 0, 240])
            };
            RgbImage::from_pixel(4, 3, color).save(&path).unwrap();
            path
        })
        .collect();

    let result = Command::new(env!("CARGO_BIN_EXE_photolayout"))
        .env("RAYON_NUM_THREADS", "4")
        .args(["stitch", "--output-dir"])
        .arg(dir.path())
        .args(paths)
        .output()
        .unwrap();

    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let stdout = String::from_utf8_lossy(&result.stdout);
    let outputs: Vec<_> = stdout.lines().collect();
    assert_eq!(outputs.len(), 2);
    assert!(outputs[0].ends_with("image+image.jpg"));
    assert!(outputs[1].ends_with("image+image_1.jpg"));
    let first = image::open(outputs[0]).unwrap().into_rgb8();
    let second = image::open(outputs[1]).unwrap().into_rgb8();
    assert!(first.get_pixel(600, 450)[0] > 220);
    assert!(second.get_pixel(600, 450)[2] > 220);
}
