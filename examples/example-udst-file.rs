use core::time;
use std::{thread, ffi::c_char};

use rust_stapsdt::{providerAddProbe, providerInit, providerLoad, probeFire};



fn main(){
    unsafe {
        let string: &str = "PROVIDER_NAME";
        let bytes: Vec<u8> = String::from(string).into_bytes();
        let mut c_chars: Vec<i8> = bytes.iter().map(| c | *c as i8).collect::<Vec<i8>>();

        c_chars.push(0); // null terminator

        let ptr: *mut c_char = c_chars.as_mut_ptr();
        let provider = providerInit(ptr);
        
        
        let string: &str = "PROBE_NAME";
        let bytes: Vec<u8> = String::from(string).into_bytes();
        let mut c_chars: Vec<i8> = bytes.iter().map(| c | *c as i8).collect::<Vec<i8>>();

        c_chars.push(0); // null terminator

        let ptr: *mut c_char = c_chars.as_mut_ptr();
        let probe = providerAddProbe(provider, ptr, 2, rust_stapsdt::ArgType_t_int64, rust_stapsdt::ArgType_t_int64);
        providerLoad(provider);
        loop {
            let mut i = 1;
            let mut j = 1;
            probeFire(probe, i as i64, j as i64);
            i += 1;
            j -= 1;
            println!("i = {i} , j = {j}");
            let ten_millis = time::Duration::from_secs(1);
            
            thread::sleep(ten_millis);
        }
    }
}