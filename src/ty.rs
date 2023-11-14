#[derive(Debug, PartialEq, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub ptr_to: Option<Box<Type>>,
    pub size: u16,
    pub members: Option<Vec<Member>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeKind {
    Int,
    Char,
    Pointer,
    Array,
    Struct,
    Func,
}

// Helper function
pub fn create_int_type() -> Type {
    Type {
        kind: TypeKind::Int,
        ptr_to: None,
        size: 2,
        members: None,
    }
}

pub fn create_char_type() -> Type {
    Type {
        kind: TypeKind::Char,
        ptr_to: None,
        size: 1,
        members: None,
    }
}

pub fn create_func_type() -> Type {
    Type {
        kind: TypeKind::Func,
        ptr_to: None,
        size: 2,
        members: None,
    }
}

// Struct member
#[derive(Debug, PartialEq, Clone)]
pub struct Member {
    pub ty: Box<Type>,
    pub name: String,
    pub offset: u16,
}
