/*
    Simulate DNA strings using DAWG
    Wrapped from a CXX library using the Crate-CXX library
    
    DAWG files were placed here by hand, not copied by a script

    > cargo run -- --seed=101 basic-dna.dawg -o basc-dna_101.fasta
*/

#[cxx::bridge(namespace="dawg_app")]
mod ffi {


    unsafe extern "C++" {
        include!("loosely_wrapped_dna/include/dawg_app.h");

        type dawg_app;

        fn new_dawg_app() -> UniquePtr<dawg_app>;
        fn run(&self, argc: u64) -> u64;
    }
}

fn main() {
    let dawg_app = ffi::new_dawg_app();
    dawg_app.run(1);
}
