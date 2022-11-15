use std::{path::PathBuf, env};

fn main(){
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let lib_dir = out_path.join("lib");
    println!("{}", lib_dir.as_path().display().to_string());

}