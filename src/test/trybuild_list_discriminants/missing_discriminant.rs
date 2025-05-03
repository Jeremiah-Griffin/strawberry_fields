use strawberry_fields::list_discriminants;

#[list_discriminants(u8)]
enum Variants {
    One = 1,
    Five,
    Ten = 10,
}

fn main() {}
