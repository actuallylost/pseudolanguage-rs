use pseudolang_types::Type;

pub struct TypedDecl {
    ident: String,
    ty: Type,
    mutable: bool,
}

pub struct Decl {
    constants: Option<Vec<TypedDecl>>,
    variables: Vec<TypedDecl>,
}

pub enum DeclKey {
    Const,
    Var,
}
