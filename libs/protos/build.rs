//
//use protobuf_build::Builder;
//fn main() {
//    Builder::new()
//        .search_dir_for_protos("proto")
//        .append_to_black_list("eraftpb")
//        .generate()
//}

use std::env::var;
use std::fs::{File, self};
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let includes = &[std::path::PathBuf::from("proto")];
    let mut protos = Vec::new();
    for include in includes {
        for file in std::fs::read_dir(include).unwrap() {
            let file = file.unwrap();
            if file.file_type().unwrap().is_dir() {
                continue;
            }
            let path = file.path();
            if path.extension().unwrap() == "proto" {
                protos.push(path);
            }
        }
    }
    let out_dir = format!("{}", var("OUT_DIR").expect("No OUT_DIR defined"));
    if Path::new(&out_dir).exists() {
        fs::remove_dir_all(&out_dir).unwrap();
    }
    fs::create_dir_all(&out_dir).unwrap();

    tonic_build::configure().out_dir(out_dir.clone())
        .compile(
            &protos,
            &["proto"]
        )?;
    generate_mod_file(out_dir);
    Ok(())
}


fn generate_mod_file(out_dir: String) {
    let modules = list_rs_files(out_dir.clone()).filter_map(|path| {
        let name = path.file_stem().unwrap().to_str().unwrap();
        if name == "mod"
        {
            return None;
        }
        Some((name.replace('-', "_"), name.to_owned()))
    });
    let mut f = File::create(format!("{}/mod.rs", out_dir)).unwrap();

    for (module, file_name) in modules {
        let mut level = 0;
        for part in module.split('.') {
            writeln!(f, "pub mod {} {{", part).unwrap();
            level += 1;
        }
        writeln!(f, "tonic::include_proto!(\"{}\");", file_name,).unwrap();
        writeln!(f, "{}", "}\n".repeat(level)).unwrap();
    }
}

fn list_rs_files(out_dir: String) -> impl Iterator<Item = PathBuf> {
    fs::read_dir(&out_dir)
        .expect("Couldn't read directory")
        .filter_map(|e| {
            let path = e.expect("Couldn't list file").path();
            if path.extension() == Some(std::ffi::OsStr::new("rs")) {
                Some(path)
            } else {
                None
            }
        })
}