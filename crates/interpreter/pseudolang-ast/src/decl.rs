use pseudolang_types::Primitive;

pub struct TypedDecl {
    ident: String,
    ty: Primitive,
    mutable: bool,
}

pub struct Decl {
    constants: Vec<TypedDecl>,
    variables: Vec<TypedDecl>,
}
