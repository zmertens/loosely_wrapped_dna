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
        fn new_dawg_app(argc: u64, argv: Vec<String>) -> UniquePtr<dawg_app>;
        fn run(&self) -> u64;
    }
}

fn main() {
    let argv =  vec![String::from("loosely_wrapped_dna"),
        String::from("--help-trick")];
        // String::from("--seed=101"),
        // String::from("basic-dna.dawg"),
        // String::from("-o"),
        // String::from("basic-dna_101.fasta")];
    let dawg_app = ffi::new_dawg_app(argv.len() as u64, argv);
    let run_result = dawg_app.run();
}
