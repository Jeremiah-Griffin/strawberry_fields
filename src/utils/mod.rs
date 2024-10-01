//#[cfg(test)]
mod test {
    use strawberry_fields_macros::test_variants_eq;

    fn convert(value: YooEight) -> YooSixteen {
        match value {
            YooEight::Zero => YooSixteen::Zero,
            YooEight::One => YooSixteen::One,
            YooEight::Ten => YooSixteen::Ten,
            YooEight::OneHundred => YooSixteen::OneHundred,
        }
    }

    #[repr(u8)]
    #[derive(Debug, PartialEq, Eq)]
    /*
    #[test_variants_eq(
        convert,
        [
            Zero =>  YooSixteen::Zero,
            One =>  YooSixteen::One,
            Ten =>  YooSixteen::Ten,
            OneHundred =>  YooSixteen::OneHundred,
        ]
    )]*/
    #[test_variants_eq(test, convert)]
    enum YooEight {
        Zero = 0,
        One = 1,
        Ten = 10,
        OneHundred = 100,
    }
    #[repr(u16)]
    #[derive(Debug, PartialEq, Eq)]

    enum YooSixteen {
        Zero = 0,
        One = 1,
        Ten = 10,
        OneHundred = 100,
    }

    impl From<YooEight> for YooSixteen {
        fn from(value: YooEight) -> Self {
            match value {
                YooEight::Zero => YooSixteen::Zero,
                YooEight::One => YooSixteen::One,
                YooEight::Ten => YooSixteen::Ten,
                YooEight::OneHundred => YooSixteen::OneHundred,
            }
        }
    }
}
