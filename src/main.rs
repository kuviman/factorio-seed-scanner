fn generate_preview_image() {
    let factorio_exe =
        std::env::var("FACTORIO").expect("Set FACTORIO env var to point to factorio executable");
    let seed = 123;
    std::fs::create_dir_all("./tmp").unwrap();
    let mut cmd = std::process::Command::new(factorio_exe);
    cmd.arg("--generate-map-preview").arg("./tmp/preview.png");
    cmd.arg("--map-gen-seed").arg(seed.to_string());
    cmd.arg("--map-gen-settings").arg("./map-gen-settings.json");
    cmd.arg("--map-preview-size").arg("2048");
    let status = cmd.status().expect("Failed to run factorio binary");
    if !status.success() {
        panic!("factorio exited with {status}");
    }
}

fn main() {
    generate_preview_image();
}
