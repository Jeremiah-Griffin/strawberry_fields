Strawberry Fields allows for the "iteration" of struct fields without allocation or 
manipulating the memory layout of the type. 

This crate's api mirrors the `Iterator` trait from the standard library, with the primary distinction being that
there are seperate methods for owned, shared and unique reference types.


Additionally, in the `utils` module (will) live tools to extend the utility of structs, enums, whatever. Right now these are a bit messy and poorly documented; 
read the `test` module of `strawberry_fields_macros` to see how they work. 

# FAQ

Q: Why not collect all fields in an array?

A: For very simple structs this may work, but for types which
may contain a mix of concrete types and generics, developers would be required to
introduce dynamic dispatch or enumeration: this incurs a runtime penalty.
