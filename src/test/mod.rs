//don't add cfg test it breaks rustanalyzer and rustfmt in weird ways
use crate::{self as strawberry_fields, StrawberryFields};
use strawberry_fields_macros::strawberry_fields;

#[test]
fn should_fail() {
    use trybuild::TestCases;

    let t = TestCases::new();
    t.compile_fail("src/test/enum.rs");
    t.compile_fail("src/test/tuple_struct.rs");
}

#[test]
fn should_pass() {
    use trybuild::TestCases;
    let t = TestCases::new();
    t.pass("src/test/struct.rs");
}

static EMPTY: Empty = Empty {};
static NOT_EMPTY: NotEmpty = NotEmpty {
    first: 1,
    second: 2,
};

#[strawberry_fields(u32)]
#[derive(Copy, Clone)]
struct Empty {}

#[strawberry_fields(u32)]
#[derive(Copy, Clone)]
struct NotEmpty {
    first: u32,
    second: u32,
}

#[test]
fn all_fields() {
    assert_eq!(NOT_EMPTY.all_fields(|num: u32| num == 0), false);
}

#[test]
fn all_fields_empty() {
    assert_eq!(EMPTY.all_fields(|num: u32| num == 0), true);
}

#[test]
fn all_fields_ref() {
    assert_eq!(NOT_EMPTY.all_fields_ref(|num: &u32| *num == 0), false);
}

#[test]
fn all_fields_ref_empty() {
    assert_eq!(EMPTY.all_fields_ref(|num: &u32| *num == 0), true);
}

#[test]
fn any_fields() {
    assert_eq!(NOT_EMPTY.any_fields(|num: u32| num == 1), true);
}

#[test]
fn any_fields_empty() {
    assert_eq!(EMPTY.any_fields(|num: u32| num == 1), false);
}

#[test]
fn any_fields_ref() {
    assert_eq!(NOT_EMPTY.any_fields_ref(|num: &u32| *num == 1), true);
}

#[test]
fn any_fields_ref_empty() {
    assert_eq!(EMPTY.any_fields_ref(|num: &u32| *num == 1), false);
}

#[test]
fn fold_fields() {
    assert_eq!(NOT_EMPTY.fold_fields(0, |i, acc| acc + i), 3);
}

#[test]
fn fold_fields_ref() {
    assert_eq!(NOT_EMPTY.fold_fields(0, |i, acc| acc + i), 3);
}

#[test]
fn fold_fields_empty() {
    assert_eq!(EMPTY.fold_fields(0, |i, acc| acc + i), 0);
}

#[test]
fn fold_fields_ref_empty() {
    assert_eq!(EMPTY.fold_fields(0, |i, acc| acc + i), 0);
}
