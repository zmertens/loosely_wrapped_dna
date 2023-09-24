/*
    Simulate DNA strings using DAWG
    DAWG is from a CXX library and wrapped using the Rust Crate called CXX
    
    DAWG files were copied over here manually

    @author zmertens
    @license MIT

    (UTF-8 characters not handling in args... yet)

    > cargo run -- --seed=101 basic-dna.dawg -o basc-dna_101.fasta
*/

use std::{env, ffi::CString, ffi::c_char};

#[cxx::bridge(namespace="dawg_app")]
mod ffi {


    unsafe extern "C++" {
        include!("loosely_wrapped_dna/include/dawg_app.h");

        type dawg_app;

        unsafe fn new_dawg_app(argc: u64, argv: *const *const c_char) -> UniquePtr<dawg_app>;
        unsafe fn run(&self) -> u64;
    }
}

fn main() {
    let args = env::args()
        .map(CString::new)
        .collect::<Result<Vec<CString>, _>>()
        .unwrap();

    let args_as_c_chars = args.iter().map(|s| s.as_ptr()).collect::<Vec<_>>();

    // dbg!(args_as_c_chars);

    // let argv =  vec!["loosely_wrapped_dna\0",
    //     "--seed=462\0",
    //     "--reps=5\0",
    //     "examples/basic-dna.dawg\0",
    //     "-o\0",
    //     "basic-dna_462.fasta\0"];
    unsafe {
        let dawg_app = ffi::new_dawg_app(args_as_c_chars.len() as u64, args_as_c_chars.as_ptr());
        let _run_result = dawg_app.run();
    }
}
