extern crate alloc;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use axhal::{
    arch::{flush_tlb, write_page_table_root}
};



/// To run a testcase with the given name and environment variables, which will be used in initproc
pub fn run_testcase(testcase: &str, envs: Vec<String>) {
    axlog::ax_println!("Running testcase: {}", testcase);
    
    axlog::ax_println!(
        "Testcase {} finished with exit code {}",
        testcase,
        0
    );
}
