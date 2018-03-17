extern crate svd_parser as svd;
extern crate failure;

use svd::error as err;
use failure::Fail;

#[test]
#[should_panic]
fn arm_sample_faulty() {
    let xml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/ARM_Sample_faulty.svd"));
    if let Err(e) = svd::parse(xml) {
        for e in e.causes() {
            println!("{}", e);
        }
        panic!()
    }
}

