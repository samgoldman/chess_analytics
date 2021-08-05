use flatc_rust::run;

use mktemp::Temp;
use std::{fs, fs::File, io, io::prelude::*, io::BufReader, io::Write, path::Path};

fn generate_chess_flatbuff() -> Result<(), std::io::Error> {
    run(flatc_rust::Args {
        inputs: &[Path::new("chess_flat_buffer/chess.fbs")],
        out_dir: Path::new("target/flatbuffers/"),
        ..Default::default()
    })
    .expect("flatc");

    let data = "// Force clippy and checks to ignore this file\n#![allow(clippy::all)]\n#![allow(unknown_lints)]\n#![allow(unused_imports)]\n#![allow(clippy::cognitive_complexity)]\n\n";

    let file_path = Path::new("target/flatbuffers/mod.rs");
    prepend_file(data.as_bytes(), &file_path)?;
    let file_path = Path::new("target/flatbuffers/chess/game_generated.rs");
    prepend_file(data.as_bytes(), &file_path)?;
    let file_path = Path::new("target/flatbuffers/chess/game_list_generated.rs");
    prepend_file(data.as_bytes(), &file_path)?;

    Ok(())
}

fn generate_steps_module() -> Result<(), std::io::Error> {
    let mut module = fs::File::create("src/steps.rs".to_string())?;

    let mut mod_declarations = String::default();
    let mut use_declarations = String::default();
    let mut names = String::default();
    let mut funcs = String::default();

    let paths = fs::read_dir("src/steps")?;

    for path in paths {
        let path = path?.path();

        let file = fs::File::open(path.clone())?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if (&line).starts_with("/// chess_analytics_build::register_step_builder ") {
                let split: Vec<&str> = (&line).split(' ').collect();
                let name = split[2];
                let struct_name = split[3];
                let step_mod_name = path.file_stem().unwrap().to_str().unwrap();
                mod_declarations += format!("mod {};\n", step_mod_name).as_ref();
                use_declarations += format!("use {}::{};\n", step_mod_name, struct_name).as_ref();
                names += format!("\t\t{},\n", name).as_ref();
                funcs += format!("\t\tBox::new({}::try_new),\n", struct_name).as_ref();
                println!("cargo:rerun-if-changed=./src/steps/{}.rs", step_mod_name);
            }
        }
    }

    writeln!(
        module,
        "// THIS FILE AUTO-GENERATED --- DO NOT MODIFY
{}
use crate::workflow_step::*;

use std::collections::HashMap;
use itertools::izip;

{}
pub fn get_step_by_name_and_params(name: &str, params: Vec<&'static str>) -> Result<BoxedStep, String> {{
    let names = vec![
{}    ];

    let funcs: Vec<StepFactory> = vec![
{}    ];

    let builders = izip!(names, funcs).collect::<HashMap<_, _>>();

    let result = builders.get(name);

    match result {{
        Some(step) => (step)(params),
        None => Err(format!(\"Step with name '{{}}' not found\", name)),
    }}
}}",
        mod_declarations, use_declarations, names, funcs
    )?;

    Ok(())
}

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=./build.rs");
    println!("cargo:rerun-if-changed=./Cargo.lock");

    generate_chess_flatbuff()?;

    generate_steps_module()?;

    Ok(())
}

// Modified from https://stackoverflow.com/questions/43441166/prepend-line-to-beginning-of-file
fn prepend_file<P: AsRef<Path>>(data: &[u8], file_path: &P) -> io::Result<()> {
    // Create a temporary file
    let tmp_path = Temp::new_file()?;
    // Open temp file for writing
    let mut tmp = File::create(&tmp_path)?;
    // Open source file for reading
    let mut src = File::open(&file_path)?;
    // Write the data to prepend
    tmp.write_all(data)?;
    // Copy the rest of the source file
    io::copy(&mut src, &mut tmp)?;
    fs::remove_file(&file_path)?;
    fs::copy(&tmp_path, &file_path)?;
    Ok(())
}
