use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    bracketed, parse,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Bracket, Colon, Comma},
    Expr, Ident, ItemEnum, Token,
};

mod kw {
    use syn::custom_keyword;
    custom_keyword!(module);
    custom_keyword!(function);
    custom_keyword!(matches);
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub struct VariantsInput {
    pub module_field: kw::module,
    pub module_colon: Token![:],
    pub module_name: Ident,
    pub module_separator: Token![,],
    pub function_field: kw::function,
    pub function_colon: Token![:],
    pub function_name: Ident,
    pub function_separator: Token![,],
    pub matches_field: kw::matches,
    pub matches_colon: Token![:],
    pub list_brackets: Bracket,
    pub list: Punctuated<TestExpression, Token![,]>,
}

impl VariantsInput {
    fn validate(input: TokenStream, definition: &ItemEnum) -> Self {
        let input: VariantsInput = parse(input).expect("Failed to parse");
        let variant_count = definition.variants.len();
        let length = input.list.len();
        if length != variant_count {
            let (item_or_items, were_or_was, variant_or_variants) = match length == 1 {
                true => ("item", "was", "variant"),
                false => ("items", "were", "variants"),
            };
            panic!("{length} {item_or_items} {were_or_was} supplied but the enum has {variant_count} {variant_or_variants}")
        }
        input
    }

    ///Common functionality between the eq and neq tests
    pub fn parse_for_equality(
        input: TokenStream,
        item: TokenStream,
        assertion: Assertion,
    ) -> TokenStream {
        let enum_definition =
            parse::<ItemEnum>(item).expect("test_for_variants may only be used with enums.");

        let variants_input = VariantsInput::validate(input, &enum_definition);

        let pairs_length = variants_input.list.len();
        let tested_function = variants_input.function_name;

        let mut variant_names = Vec::with_capacity(pairs_length);
        let mut test_names = Vec::with_capacity(pairs_length);
        let mut expected_values = Vec::with_capacity(pairs_length);

        for elem in variants_input.list.into_iter() {
            let variant_name_string = elem.input.clone().to_token_stream().to_string();
            let tested_function = tested_function.clone();

            test_names.push(format_ident!("{tested_function}_{variant_name_string}"));
            variant_names.push(elem.input);
            expected_values.push(elem.pattern);
        }
        let enum_name = enum_definition.ident.clone();
        //We add some randomness to the test module name so that users can generate mulitple sets of tests for the same enum.
        let module = variants_input.module_name;

        quote! {
            #enum_definition

            ///this generates non snake case test names. I could write formatting to conver it but, like, why?
            #[allow(non_snake_case)]
            #[cfg(test)]
            mod #module{
                use super::*;
                #(
                    #[test]
                    fn #test_names(){
                        #assertion(#tested_function(#enum_name::#variant_names), #expected_values);
                    }
                )*
            }
        }
        .into()
    }
}

impl Parse for VariantsInput {
    #[allow(unused_variables)]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let list_buffer;
        Ok(VariantsInput {
            module_field: kw::module::parse(input).expect("expected \"module\" field first"),
            module_colon: Colon::parse(input)
                .expect("expected colon between the \"module\" field and the module name"),
            module_name: Ident::parse(input).expect("expected an ident for the module name"),
            module_separator: Comma::parse(input).expect("expected comma after function name"),
            function_field: kw::function::parse(input).expect("expected \"function\" field first"),
            function_colon: Colon::parse(input)
                .expect("expected colon between the \"function\" field and the function name"),
            function_name: Ident::parse(input).expect("expected an ident for the function name"),
            function_separator: Comma::parse(input).expect("expected comma after module name"),
            matches_field: kw::matches::parse(input).expect("expected \"matches\" field"),
            matches_colon: Colon::parse(input)
                .expect("expected colon between the \"matches\" field and the list of matches"),
            list_brackets: bracketed!(list_buffer in input),
            list: Punctuated::parse_terminated(&list_buffer).expect("Parsing list failed"),
        })
    }
}

pub enum Assertion {
    Eq,
    Ne,
}

impl ToTokens for Assertion {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Assertion::Eq => tokens.extend(quote! {assert_eq!}),
            Assertion::Ne => tokens.extend(quote! {assert_ne!}),
        }
    }
}
