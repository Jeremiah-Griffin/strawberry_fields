Strawberry Fields allows for the "iteration" of struct fields without allocation or 
manipulating the memory layout of the type.

# FAQ

Q: Why not collect all fields in an array?

A: For very simple structs this may work, but for types which
may contain a mix of concrete types and generics, developers would be required to
introduce dynamic dispatch or enumeration which may induce a runtime penalty.

