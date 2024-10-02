use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    bracketed, parse,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Bracket, Colon, Comma, PathSep},
    Expr, ExprReference, Ident, ItemEnum, Path, Token,
};

mod kw {
    use syn::custom_keyword;
    custom_keyword!(module);
    custom_keyword!(function);
    custom_keyword!(matches);
}

#[derive(Clone)]
pub enum ExprOrReference {
    Expression(Expr),
    Reference(ExprReference),
}

impl Parse for ExprOrReference {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.fork().parse::<ExprReference>().is_ok() {
            true => ExprReference::parse(input).map(|r| Self::Reference(r)),
            false => Expr::parse(input).map(|e| Self::Expression(e)),
        }
    }
}

impl ExprOrReference {
    fn expression_without_reference(&self) -> &Expr {
        match self {
            ExprOrReference::Expression(expr) => expr,
            ExprOrReference::Reference(expr_reference) => expr_reference.expr.as_ref(),
        }
    }
}

#[allow(dead_code)]
pub struct TestExpression {
    //Optional as references are only needed if the type signature of the function needs it.
    pub input: ExprOrReference,
    pub fat_arrow: Token![=>],
    pub pattern: Expr,
}

impl Parse for TestExpression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            input: ExprOrReference::parse(input).expect("could not parse input pattern"),
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
    //using a path rather than an ident is useful for testing associated functions and trait impls.
    pub function_name: Path,
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

        //the variants which will be tested

        //TODO: This is beyond horrible. Collect this into a single vector of a T that implements to tokens
        let mut variant_names: Vec<proc_macro2::TokenStream> = Vec::with_capacity(pairs_length);
        //names generated from the function name and the variant tame which will be given to each test case
        let mut test_names: Vec<Ident> = Vec::with_capacity(pairs_length);
        //The patterns to be matched
        let mut expected_values = Vec::with_capacity(pairs_length);

        for elem in variants_input.list.into_iter() {
            //generate test function names.
            {
                let variant_name_string = elem
                    .input
                    .expression_without_reference()
                    .to_token_stream()
                    .to_string();
                let tested_function = tested_function
                    .clone()
                    .segments
                    .last()
                    .expect("The function (or path) does not have a final segment as expected.")
                    .ident
                    .clone();

                test_names.push(format_ident!("{tested_function}_{variant_name_string}"));
            }

            /*
            I really prefer this but cant get it to build and dont really know why

            {
                let mut test_name = proc_macro2::TokenStream::new();

                elem.input
                    .expression_without_reference()
                    .to_tokens(&mut test_name);
                tested_function
                    .segments
                    .last()
                    .expect("The function (or path) does not have a final segment as expected.")
                    .ident
                    .to_tokens(&mut test_name);
                test_names.push(test_name);
            }
            */

            //format variant name from &Variant to &Fully::Qualified::Path::To::Variant
            {
                let mut variant_name = proc_macro2::TokenStream::new();

                let input_variant_expression = &elem.input;

                if let ExprOrReference::Reference(r) = input_variant_expression {
                    r.and_token.to_tokens(&mut variant_name);
                    if let Some(mutability) = r.mutability {
                        mutability.to_tokens(&mut variant_name);
                    }
                }

                enum_definition.ident.to_tokens(&mut variant_name);
                PathSep::default().to_tokens(&mut variant_name);
                input_variant_expression
                    .expression_without_reference()
                    .to_tokens(&mut variant_name);
                variant_names.push(variant_name);
            }

            expected_values.push(elem.pattern);
        }
        //We add some randomness to the test module name so that users can generate mulitple sets of tests for the same enum.
        let module = variants_input.module_name;

        quote! {
            #enum_definition

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod #module{
                //allows paths relative to parent
                use super::*;
                #(
                    #[test]
                    fn #test_names(){
                        #assertion(#tested_function(#variant_names), #expected_values);
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
            function_name: Path::parse(input).expect("expected an ident for the function name"),
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
