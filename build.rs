use flatc_rust;

use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=chess_flat_buffer/chess.fbs");
    flatc_rust::run(flatc_rust::Args {
        inputs: &[Path::new("chess_flat_buffer/chess.fbs")],
        out_dir: Path::new("target/flatbuffers/"),
        ..Default::default()
    }).expect("flatc");
}