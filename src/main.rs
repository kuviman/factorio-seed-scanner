use batbox_la::*;
use std::{collections::BTreeSet, path::Path};

mod map;

use crate::map::Map;

fn generate_preview_image(seed: u32, path: impl AsRef<Path>) {
    println!("Generating preview for seed={seed:?}");
    let factorio_exe =
        std::env::var("FACTORIO").expect("Set FACTORIO env var to point to factorio executable");
    let mut cmd = std::process::Command::new(factorio_exe);
    cmd.arg("--generate-map-preview").arg(path.as_ref());
    cmd.arg("--map-gen-seed").arg(seed.to_string());
    cmd.arg("--map-gen-settings").arg("./map-gen-settings.json");
    cmd.arg("--map-preview-size").arg("2048");
    cmd.stdout(std::process::Stdio::null());
    let status = cmd.status().expect("Failed to run factorio binary");
    if !status.success() {
        panic!("factorio exited with {status}");
    }
}

#[derive(clap::Parser)]
struct Args {
    #[clap(long, default_value = "30")]
    pub safe_dist: i32,
    pub from: u32,
    pub to: u32,
    #[clap(long, default_value = "10")]
    pub top: usize,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct MapResult {
    iron: map::Patches,
    copper: map::Patches,
    coal: map::Patches,
    stone: map::Patches,
}

fn process(args: &Args, seed: u32) -> MapResult {
    std::fs::create_dir_all("./tmp").unwrap();
    let path = Path::new("./tmp/preview.png");
    generate_preview_image(seed, path);
    let image = image::open(path).unwrap().to_rgba8();
    println!("Parsing preview image");
    let map = Map::new(&image);
    println!("Preview image parsed");
    MapResult {
        iron: map.find_patches(&map.iron, args.safe_dist),
        copper: map.find_patches(&map.copper, args.safe_dist),
        coal: map.find_patches(&map.coal, args.safe_dist),
        stone: map.find_patches(&map.stone, args.safe_dist),
    }
}

fn main() {
    let args: Args = clap::Parser::parse();
    let mut results = BTreeSet::new();
    for seed in (args.from..args.to).step_by(2) {
        let result = process(&args, seed);
        println!("seed = {seed}: {result:#?}");
        results.insert((result, seed));
    }
    for (rank, (result, seed)) in results.into_iter().rev().enumerate().take(args.top).rev() {
        let rank = rank + 1;
        println!("#{rank}. {seed} = {result:#?}");
    }
}
