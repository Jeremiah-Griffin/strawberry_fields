pub use strawberry_fields_macros::test_for_variants_eq;
#[cfg(test)]
mod test {
    use strawberry_fields_macros::test_variants_eq;

    #[repr(u8)]
    #[derive(Debug, PartialEq, Eq)]
    #[test_variants_eq[(Zero, 0), (One, 1), (Ten, 10), (OneHundred, 100)]]
    enum YooEight {
        Zero = 0,
        One = 1,
        Ten = 10,
        OneHundred = 100,
    }
}
