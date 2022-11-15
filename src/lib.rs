#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// include!("/home/steve/WorkSpace/ebpf-projs/rust-stapsdt/bindings.rs");


#[cfg(test)]
mod tests {
    use std::{path::PathBuf, env};

    use super::*;

    #[test]
    fn hello() {
        // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        // let lib_dir = out_path.join("lib");
        // println!("{}", lib_dir.as_path().display().to_string());
        // unsafe {
        //     let provider = providerInit("PROVIDER".as_ptr() as *mut _);
        //     let probe = providerAddProbe(provider, "PROBE_NAME".as_ptr() as *const _, 3, ArgType_t_int64, ArgType_t_int64);
        //     providerLoad(provider);
        //     loop {
        //         let mut i = 1;
        //         let mut j = 1;
        //         probeFire(probe, i as i64, j as i64);
        //         i +=1;
        //         j -=1;
        //         println!("i = {i} , j = {j}");
        //     }
        // }
    }
}
