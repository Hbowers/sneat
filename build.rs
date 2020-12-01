use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:warning=running");
    let out_dir = env::var_os("OUT_DIR").expect("NO OUT_DIR DEFINED");
    let project_root = env::var_os("CARGO_MANIFEST_DIR").expect("NO CARGO_HOME DEFINED");
    let mut out_dir_buff = PathBuf::from(out_dir);

    // Move to build root
    out_dir_buff.pop();
    out_dir_buff.pop();
    out_dir_buff.pop();

    println!(
        "cargo:warning=copying assets to build dir {:?}",
        out_dir_buff
    );

    let project_root_string = project_root.to_os_string().into_string().unwrap();

    let assets = format!("{}/assets", project_root_string);
    let config = format!("{}/config", project_root_string);
    let levels = format!("{}/levels", project_root_string);

    let build_dir = out_dir_buff.to_str().unwrap();

    println!("cargo:warning=copying assets {:?} to build dir {:?}", assets, build_dir);
    println!("cargo:warning=copying config {:?} to build dir {:?}", config, build_dir);
    println!("cargo:warning=copying levels {:?} to build dir {:?}", levels, build_dir);


    Command::new("cp")
        .args(&["-r", &assets[..], build_dir])
        .status()
        .unwrap();
    Command::new("cp")
        .args(&["-r", &config[..], build_dir])
        .status()
        .unwrap();
    Command::new("cp")
        .args(&["-r", &levels[..], build_dir])
        .status()
        .unwrap();
}
