use std::env;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), io::Error> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let executable = env::var("CMAKE").unwrap_or_else(|_| "cmake".to_owned());
    let profile = env::var("PROFILE").unwrap();
    let mut cmd = Command::new(&executable);
    cmd.current_dir(&out_dir);
    let mut lib_dir = env::current_dir()?;
    lib_dir.pop();
    lib_dir.push("synthizer");
    cmd.arg(lib_dir);
    cmd.args(&["-G", "Ninja"]);
    cmd.arg("-DCMAKE_C_COMPILER=clang");
    cmd.arg("-DCMAKE_CXX_COMPILER=clang++");
    if profile == "debug" {
        cmd.arg("-DCMAKE_BUILD_TYPE=Debug");
    }
    let output = cmd.output()?;
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    let status = cmd.status()?;
    if !status.success() {
        std::process::exit(1);
    }
    let mut cmd = Command::new(&executable);
    cmd.current_dir(&out_dir);
    cmd.arg("--build");
    cmd.arg(&out_dir);
    let output = cmd.output()?;
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    let status = cmd.status()?;
    if !status.success() {
        std::process::exit(1);
    }
    println!("cargo:rustc-link-search=native={}", &out_dir);
    println!("cargo:rustc-link-lib=static=synthizer");
    let _ = bindgen::builder()
        .header("../synthizer/include/synthizer.h")
        .header("../synthizer/include/synthizer_constants.h")
        .generate()
        .unwrap()
        .write_to_file(Path::new(&out_dir).join("synthizer_sys.rs"));
    Ok(())
}
