extern crate build_tests;

use build_tests::*;

fn main() {
    let template = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/test-template.rs"));

    for mut builder in TestSuiteBuilder::new("rustup-upgrade-tests") {
        builder.test(template);
        return;
    }
}
