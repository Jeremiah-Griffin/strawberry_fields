use strawberry_fields_macros::{test_variants_eq, test_variants_ne, };

fn convert(value: YooEight) -> YooSixteen {
    match value {
        YooEight::Zero => YooSixteen::Zero,
        YooEight::One => YooSixteen::One,
        YooEight::Ten => YooSixteen::Ten,
        YooEight::OneHundred => YooSixteen::OneHundred,
    }
}

impl From<YooEight> for YooSixteen {
    fn from(value: YooEight) -> Self {
      convert(value)
    }
}


#[derive(Debug, PartialEq, Eq)]
#[test_variants_eq {
    module: eq_simple, 
    function: convert, 
    matches:[
        Zero => YooSixteen::Zero,
        One => YooSixteen::One,
        Ten => YooSixteen::Ten,
        OneHundred => YooSixteen::OneHundred,
    ]
}]


#[test_variants_ne {
    module: ne_simple, 
    function: convert, 
    matches: [
        Zero => YooSixteen::OneHundred,
        One => YooSixteen::Ten,
        Ten => YooSixteen::One,
        OneHundred => YooSixteen::Zero,
    ]
}]

//mostly useful to ensure that associated/trait methods are parsed correctly by the macro
#[test_variants_eq{
    module: onvert_eq_from, 
    function: YooSixteen::from, 
    matches: [
        Zero => convert(YooEight::Zero),
        One => convert(YooEight::One),
        Ten => convert(YooEight::Ten),
        OneHundred => convert(YooEight::OneHundred),
    ]
}]
//mostly useful to ensure that associated/trait methods are parsed correctly by the macro
#[test_variants_ne{
    module: convert_ne_from, 
    function: YooSixteen::from, 
    matches: [
        Zero => convert(YooEight::OneHundred),
        One => convert(YooEight::Ten),
        Ten => convert(YooEight::One),
        OneHundred => convert(YooEight::Zero),
    ]
}]    
enum YooEight {
    Zero = 0,
    One = 1,
    Ten = 10,
    OneHundred = 100,
}
#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[test_variants_eq {
    module: eq_less_simple, 
    function: convert_number_or_not, 
    matches: [
        Zero => Some(0),
        One => Some(1),
        Ten => Some(10),
        OneHundred => Some(100),
    ]
}]
#[test_variants_ne {
    module: ne_less_simple, 
    function: convert_number_or_not, 
    matches: [
        Zero => Some(100),
        One => Some(10),
        Ten => Some(1),
        OneHundred => Some(0),
    ]
}]
enum YooSixteen {
    Zero = 0,
    One = 1,
    Ten = 10,
    OneHundred = 100,
}

fn convert_number_or_not(something: YooSixteen) -> Option<u64>{
    Some(something as u64)
}


#[derive(PartialEq, Eq, Debug, Clone)]
#[test_variants_eq{
    module: ref_eq,
    function: ReferenceTester::test_ref,
    matches: [
        &First => ReferenceTester::First,
        &Second => ReferenceTester::Second,
    ]
}]
#[test_variants_eq{
    module: mut_ref_eq,
    function: ReferenceTester::test_mut_ref,
    matches: [
        &mut First => ReferenceTester::First,
        &mut Second => ReferenceTester::Second,
    ]
}]
#[test_variants_ne{
    module: ref_ne,
    function: ReferenceTester::test_ref,
    matches: [
        &First => ReferenceTester::Second,
        &Second => ReferenceTester::First,
    ]
}]
#[test_variants_ne{
    module: mut_ref_ne,
    function: ReferenceTester::test_mut_ref,
    matches: [
        &mut First => ReferenceTester::Second,
        &mut Second => ReferenceTester::First,
    ]
}]
enum ReferenceTester{
    First,
    Second,
}

impl ReferenceTester{
    fn test_ref(&self) -> ReferenceTester{
        self.clone()
    }

    fn test_mut_ref(&mut self) -> ReferenceTester{
        self.clone()
    }

}


#[test_variants_eq{
    module: has_fields_eq,
    function: HasFields::number,
     matches: [
         Zero => 0,
         Named{a: 10} => 10,
         Unnamed(100) => 100,
     ]   
}]

#[test_variants_ne{
    module: has_fields_ne,
    function: HasFields::number,
     matches: [
         Zero => 42,
         Named{a: 10} => 42,
         Unnamed(100) => 42,
     ]   
}]
enum HasFields{
    Zero,
    Named{a: u8},
    Unnamed(u8),    
}

impl HasFields{
    fn number(self) -> u8{
        match self{
            HasFields::Zero => 0,
            //note the space! keep this here. Useful to test
            HasFields::Named { a } => a,
            HasFields::Unnamed(a) => a,
        }
    
    }
}

struct Byte{inner: u8}

struct Tuple((u8, u8));

#[test_variants_eq{
    module: has_struct_eq,
    function: HasStruct::number,
    matches: [
        Byte(Byte{inner: 0}) => 0,
        Tuple(Tuple((5,5))) => 10,
    ]
}]
#[test_variants_ne{
    module: has_struct_ne,
    function: HasStruct::number,
    matches: [
        Byte(Byte{inner: 0}) => 10,
        Tuple(Tuple((5,5))) => 0,
    ]
}]

enum HasStruct{
    Byte(Byte),
    Tuple(Tuple),
}


impl HasStruct{
    fn number(self) -> u8{
        match self{
            HasStruct::Byte(clike) => clike.inner,
            HasStruct::Tuple(Tuple((first, second))) => first + second,
        }
    }
}

