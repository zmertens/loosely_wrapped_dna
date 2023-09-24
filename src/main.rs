/*
    Simulate DNA strings using DAWG
    DAWG is from a CXX library and wrapped using the Crate-CXX library
    
    DAWG files were placed here by hand, not copied by a script

    > cargo run -- --seed=101 basic-dna.dawg -o basc-dna_101.fasta
*/

#[cxx::bridge(namespace="dawg_app")]
mod ffi {


    unsafe extern "C++" {
        include!("loosely_wrapped_dna/include/dawg_app.h");

        type dawg_app;

        // fn new_dawg_app() -> UniquePtr<dawg_app>;
        fn new_dawg_app(argc: u64, argv: Vec<&str>) -> UniquePtr<dawg_app>;
        fn run(&self) -> u64;
    }
}

fn main() {
    let argv =  vec!["loosely_wrapped_dna\0",
        "--seed=462\0",
        "--reps=1\0",
        "examples/basic-dna.dawg\0",
        "-o\0",
        "basic-dna_462.fasta\0"];
    let dawg_app = ffi::new_dawg_app(argv.len() as u64, argv);
    let run_result = dawg_app.run();
}
