// Integration test for SigmaOS-tools-fivetran
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use SigmaOS_tools_fivetran as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
