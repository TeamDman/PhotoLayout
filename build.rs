fn main() {
    println!("cargo:rerun-if-changed=resources/photolayout.rc");
    println!("cargo:rerun-if-changed=resources/photolayout.ico");

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        embed_resource::compile("resources/photolayout.rc", embed_resource::NONE)
            .manifest_required()
            .expect("failed to embed the PhotoLayout executable icon");
    }
}
