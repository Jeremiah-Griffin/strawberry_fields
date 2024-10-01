mod test {}
use proc_macro::TokenStream;
use syn::{
    bracketed, parse,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Bracket, Comma},
    Expr, Ident, Token,
};

pub struct TestExpression {
    pub input: Expr,
    pub fat_arrow: Token![=>],
    pub pattern: Expr,
}

impl Parse for TestExpression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            input: Expr::parse(input).expect("could not parse input pattern"),
            fat_arrow: <Token![=>]>::parse(input)?,
            pattern: Expr::parse(input)?,
        })
    }
}

pub struct VariantsInput {
    pub module_name: Ident,
    pub module_separator: Token![,],
    pub function_name: Ident,
    pub function_separator: Token![,],
    pub list_brackets: Bracket,
    pub list: Punctuated<TestExpression, Token![,]>,
}

impl VariantsInput {
    pub fn validate(input: TokenStream, variant_count: usize) -> Self {
        let input: VariantsInput = parse(input).expect("Failed to parse");

        let input_test_length = input.list.len();

        if input_test_length != variant_count {
            let item_or_items = match input_test_length == 1 {
                true => "item",
                false => "items",
            };

            let variant_or_variants = match variant_count == 1 {
                true => "variant",
                false => "variants",
            };

            panic!("{input_test_length } {item_or_items} were supplied but the enum has {variant_count} {variant_or_variants}")
        }

        input
    }
}

impl Parse for VariantsInput {
    #[allow(unused_variables)]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let list_buffer;
        Ok(VariantsInput {
            module_name: Ident::parse(input).expect("expected an ident for the module name"),
            module_separator: Comma::parse(input).expect("expected comma after function name"),
            function_name: Ident::parse(input).expect("expected an ident for the function name"),
            function_separator: Comma::parse(input).expect("expected comma after module name"), //list_brackets: bracketed!(list_buffer in input),
            list_brackets: bracketed!(list_buffer in input),
            list: Punctuated::parse_terminated(&list_buffer).expect("Parsing list failed"),
        })
    }
}
