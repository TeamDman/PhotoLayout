# PhotoLayout

PhotoLayout combines JPEG and PNG files in pairs. Each pair produces a 1200 × 1800 JPEG. Portrait inputs rotate clockwise, and each photo is scaled and center-cropped to fill its 1200 × 900 half.

## Desktop

Run `cargo run --release --locked`. Choose an output folder, add or drop files in pairing order, and click **Begin**. The first file in each pair goes on top. Select a filename to preview the photo. If the list has an odd number of photos, Begin asks you to fix it before writing any JPEGs.

The folder pickers remember their locations independently between launches. The output picker starts in Downloads and Add Files starts in Pictures on first use.

## Command line

```powershell
cargo run --release --locked -- stitch --output-dir . --separator "+" first.jpg second.png
```

Each desktop or CLI run creates a new numbered folder inside the chosen output directory, such as `1`, `2`, or `3`. A pair produces `first+second.jpg`. If another pair would use the same name, it gets a suffix such as `first+second_1.jpg`. A final unpaired file is reported and skipped.

The `Java` branch contains the original Swing app. The Rust port uses the Facet/Figue CLI pattern from `teamy-rust-cli` (`cf22fc8`) and CM's eframe and image stack.
