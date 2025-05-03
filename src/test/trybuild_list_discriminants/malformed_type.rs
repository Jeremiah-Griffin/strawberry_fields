use strawberry_fields::list_discriminants;

#[list_discriminants(type)]
enum Variants {
    One = 1,
    Five = 5,
    Ten = 10,
}

fn main() {}
