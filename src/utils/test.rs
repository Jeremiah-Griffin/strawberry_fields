use strawberry_fields_macros::{test_variants_eq, test_variants_ne};

fn convert(value: YooEight) -> YooSixteen {
    match value {
        YooEight::Zero => YooSixteen::Zero,
        YooEight::One => YooSixteen::One,
        YooEight::Ten => YooSixteen::Ten,
        YooEight::OneHundred => YooSixteen::OneHundred,
    }
}

#[derive(Debug, PartialEq, Eq)]
#[test_variants_eq{
    module: test_eq_simple, 
    function: convert, 
    matches: 
        [
            Zero => YooSixteen::Zero,
            One => YooSixteen::One,
            Ten => YooSixteen::Ten,
            OneHundred => YooSixteen::OneHundred,
        ]
    }
 ]


#[test_variants_ne{
    module: test_ne_simple, 
    function: convert, 
    matches: 
        [
            Zero => YooSixteen::OneHundred,
            One => YooSixteen::Ten,
            Ten => YooSixteen::One,
            OneHundred => YooSixteen::Zero,
        ]
    }
 ]

/*
#[test_variants_neq(test_neq_simple, convert, [
    Zero => YooSixteen::OneHundred,
    One => YooSixteen::Ten,
    Ten => YooSixteen::One,
    OneHundred => YooSixteen::Zero,

])]*/
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
