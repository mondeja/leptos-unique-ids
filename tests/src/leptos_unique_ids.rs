//! Tests for `#[leptos_unique_ids()]` attribute macro.

#[test]
fn fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("ui/fail/*.rs");
}

#[test]
fn pass() {
    let t = trybuild::TestCases::new();
    t.pass("ui/pass/*.rs");
}
