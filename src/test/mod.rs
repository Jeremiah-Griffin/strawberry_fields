#[cfg(test)]
#[test]
fn should_fail() {
    use trybuild::TestCases;

    let t = TestCases::new();
    t.compile_fail("src/test/derive_enum.rs");
    //t.compile_fail("derive_tuple_struct.rs");
}

#[test]
fn should_pass() {}

#[test]
fn all_fields() {}

#[test]
fn all_fields_ref() {}

#[test]
fn any_fields() {}

#[test]
fn any_fields_ref() {}

#[test]
fn fold_fields() {}

#[test]
fn fold_fields_ref() {}
