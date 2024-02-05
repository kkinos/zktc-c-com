use nom::{
    branch::{alt, permutation},
    bytes::complete::{tag, take_until, take_while1},
    character::{
        complete::{digit1, hex_digit1, multispace0, multispace1},
        is_alphanumeric,
    },
    combinator::{fail, opt},
    error::{context, VerboseError},
    multi::{many0, many0_count, separated_list0},
    sequence::delimited,
    IResult,
};

use crate::ty::{create_char_type, create_func_type, create_int_type, Member, Type, TypeKind};

// AST node
#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub ty: Option<Box<Type>>,
}

// AST node type
#[derive(Debug, PartialEq, Clone)]
pub enum NodeKind {
    Add(Op),              // +
    Sub(Op),              // -
    Mul(Op),              // *
    Div(Op),              // /
    BitAnd(Op),           // &
    BitOr(Op),            // |
    BitXor(Op),           // ^
    Eq(Op),               // ==
    Ne(Op),               // !=
    Lt(Op),               // <
    Le(Op),               // <=
    LogAnd(Log),          // &&
    LogOr(Log),           // ||
    Assign(Op),           // =
    Return(Return),       // return
    If(If),               // if
    While(While),         // while
    For(For),             // for
    Break(Break),         // break
    Continue(Continue),   // continue
    Block(Block),         // {...}
    FuncCall(FuncCall),   // Functon call
    MemAccess(MemAccess), // . Struct member access
    Addr(Addr),           // unary &
    Deref(Deref),         // unary *
    PtrAdd(PtrOp),        //
    PtrSub(PtrOp),        //
    Num(Num),             // Integer
    Var(Var),             // Variable
    Asm(Asm),             // Assembler
    Null,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Num {
    pub val: u16,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Op {
    pub left: Box<Node>,
    pub right: Box<Node>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Log {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub label: u16,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Var {
    pub name: String,
    pub offset: u16,
    pub is_global: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    pub expr: Box<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct If {
    pub cond: Box<Node>,
    pub then: Box<Node>,
    pub els: Option<Box<Node>>,
    pub label: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct While {
    pub cond: Box<Node>,
    pub then: Box<Node>,
    pub label: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct For {
    pub init: Option<Box<Node>>,
    pub cond: Option<Box<Node>>,
    pub inc: Option<Box<Node>>,
    pub then: Option<Box<Node>>,
    pub label: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Break {
    pub label: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Continue {
    pub label: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub body: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FuncCall {
    pub name: String,
    pub args: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Addr {
    pub unary: Box<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Deref {
    pub unary: Box<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PtrOp {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MemAccess {
    pub unary: Box<Node>,
    pub member: Box<Member>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Asm {
    pub asm: String,
}

// Entire input program
#[derive(Debug)]
pub struct Program {
    pub funcs: Vec<Func>,
    pub globals: Vec<Scope>,
}

// Function
#[derive(Debug)]
pub struct Func {
    pub kind: FuncKind,
    pub ty: Box<Type>,
    pub name: String,
    pub args: Vec<Node>,
    pub nodes: Vec<Node>,
    pub local_offset: u16,
    pub is_naked: bool,
}

#[derive(Debug, PartialEq)]
pub enum FuncKind {
    Init,
    Main,
    Other,
}

// Scope
#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    pub kind: ScopeKind,
    pub ty: Option<Box<Type>>,
    pub name: String,
    pub str: Option<String>,
    pub offset: Option<u16>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ScopeKind {
    Var,     // Local variable, global variable
    Str,     // String
    Tag,     // Struct tag
    Typedef, // Typedef
    Null,
}

static mut GLOBALS: Vec<Scope> = Vec::new();
static mut LOCALS: Vec<Scope> = Vec::new();
static mut IS_GLOBAL: bool = false;
static mut STR_LABEL_COUNT: u16 = 0;
static mut CTR_LABEL_COUNT: u16 = 0;
static mut SCOPE_CTR_LABEL: u16 = 0;

fn parse_space_or_comment(text: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let (mut i, _) = multispace0(text)?;
    loop {
        let (j, comment) = parse_comment(i)?;
        if comment.is_empty() {
            break;
        }
        let (j, _) = multispace0(j)?;
        i = j
    }

    Ok((i, ""))
}

fn parse_comment(text: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let (i, s) = opt(tag("//"))(text)?;
    if s.is_some() {
        let (i, comment) = take_until("\n")(i)?;
        let (i, _) = tag("\n")(i)?;
        return Ok((i, comment));
    }
    let (i, s) = opt(tag("/*"))(text)?;
    if s.is_some() {
        let (i, comment) = take_until("*/")(i)?;
        let (i, _) = tag("*/")(i)?;
        return Ok((i, comment));
    }
    Ok((i, ""))
}

fn is_ident(c: char) -> bool {
    is_alphanumeric(c as u8) || c == '_'
}

// declspec = "int" | "char" | struct-decl | "typedef" declspec ident | ident
fn parse_declspec(text: &str) -> IResult<&str, Type, VerboseError<&str>> {
    let (i, type_kind) = opt(alt((
        tag("int"),
        tag("char"),
        tag("struct"),
        tag("typedef"),
    )))(text)?;

    if let Some(type_kind) = type_kind {
        match type_kind {
            "int" => Ok((i, create_int_type())),
            "char" => Ok((i, create_char_type())),
            "struct" => {
                let (i, _) = multispace0(i)?;
                Ok(parse_struct_decl(i)?)
            }
            "typedef" => {
                let (i, _) = multispace1(i)?;
                let (i, ty) = parse_declspec(i)?;
                let (i, _) = multispace1(i)?;
                let (i, (ident, ty)) = parse_declarator(i, ty)?;
                unsafe {
                    if IS_GLOBAL {
                        GLOBALS.push(Scope {
                            kind: ScopeKind::Typedef,
                            ty: Some(Box::new(ty.clone())),
                            name: ident.to_string(),
                            str: None,
                            offset: None,
                        })
                    } else {
                        LOCALS.push(Scope {
                            kind: ScopeKind::Typedef,
                            ty: Some(Box::new(ty.clone())),
                            name: ident.to_string(),
                            str: None,
                            offset: None,
                        })
                    }
                }

                Ok((i, ty))
            }
            _ => {
                unreachable!()
            }
        }
    } else {
        let (i, ident) = take_while1(is_ident)(i)?;
        unsafe {
            if let Some(typedef) = LOCALS
                .iter()
                .find(|&scope| scope.kind == ScopeKind::Typedef && scope.name == ident)
            {
                Ok((i, *typedef.ty.clone().unwrap()))
            } else if let Some(gtypedef) = GLOBALS
                .iter()
                .find(|&scope| scope.kind == ScopeKind::Typedef && scope.name == ident)
            {
                return Ok((i, *gtypedef.ty.clone().unwrap()));
            } else {
                return context("Unknown type", fail)(i);
            }
        }
    }
}

// struct_decl = ident? "{" (struct_member)* "}"
fn parse_struct_decl(text: &str) -> IResult<&str, Type, VerboseError<&str>> {
    let mut members = Vec::new();
    let (i, _) = multispace0(text)?;
    let (i, ident) = opt(take_while1(is_ident))(i)?;
    let (i, _) = multispace0(i)?;
    let (mut t, bracket) = opt(tag("{"))(i)?;
    if let Some(ident) = ident {
        if bracket.is_none() {
            unsafe {
                if let Some(tag) = LOCALS
                    .iter()
                    .find(|&scope| scope.kind == ScopeKind::Tag && scope.name == ident)
                {
                    return Ok((i, *tag.ty.clone().unwrap()));
                } else if let Some(gtag) = GLOBALS
                    .iter()
                    .find(|&scope| scope.kind == ScopeKind::Tag && scope.name == ident)
                {
                    return Ok((i, *gtag.ty.clone().unwrap()));
                } else {
                    return context("Unknown struct type", fail)(i);
                }
            }
        }
    }
    let mut offset = 0;
    loop {
        let (i, _) = multispace0(t)?;
        let (i, s) = opt(tag("}"))(i)?;
        if s.is_some() {
            t = i;
            break;
        }
        let (i, _) = multispace0(i)?;
        let (i, ty) = parse_declspec(i)?;
        let (i, _) = multispace0(i)?;
        let (i, (ident, mem_ty)) = parse_declarator(i, ty)?;
        let (i, _) = multispace0(i)?;
        let (i, _) = tag(";")(i)?;
        members.push(Member {
            ty: Box::new(mem_ty.clone()),
            name: ident.to_string(),
            offset,
        });
        offset += mem_ty.size;
        t = i;
    }

    let ty = Type {
        kind: TypeKind::Struct,
        ptr_to: None,
        size: offset,
        members: Some(members),
    };
    if let Some(ident) = ident {
        unsafe {
            if IS_GLOBAL {
                GLOBALS.push(Scope {
                    kind: ScopeKind::Tag,
                    ty: Some(Box::new(ty.clone())),
                    name: ident.to_string(),
                    str: None,
                    offset: None,
                })
            } else {
                LOCALS.push(Scope {
                    kind: ScopeKind::Tag,
                    ty: Some(Box::new(ty.clone())),
                    name: ident.to_string(),
                    str: None,
                    offset: None,
                })
            }
        }
    }
    Ok((t, ty))
}

// declarator = "*"* ident ("[" num "]")*
fn parse_declarator(text: &str, ty: Type) -> IResult<&str, (&str, Type), VerboseError<&str>> {
    let (i, cnt) = many0_count(tag("*"))(text)?;
    let mut ty = ty;
    for _ in 0..cnt {
        ty = Type {
            kind: TypeKind::Pointer,
            ptr_to: Some(Box::new(ty)),
            size: 2,
            members: None,
        }
    }
    let (i, _) = multispace0(i)?;
    let (i, ident) = take_while1(is_ident)(i)?;

    let (i, _) = multispace0(i)?;
    let (i, num) = many0(delimited(
        tag("["),
        delimited(multispace0, digit1, multispace0),
        tag("]"),
    ))(i)?;
    for n in num.iter().rev() {
        let n = n.parse::<u16>().unwrap();
        ty = Type {
            kind: TypeKind::Array,
            ptr_to: Some(Box::new(ty.clone())),
            size: ty.size * n,
            members: None,
        }
    }

    Ok((i, (ident, ty)))
}

pub fn parse_program(
    mut text: &str,
    globals: Vec<Scope>,
) -> IResult<&str, (Vec<Func>, Vec<Scope>), VerboseError<&str>> {
    let mut funcs: Vec<Func> = Vec::new();
    unsafe { GLOBALS = globals.clone() }
    loop {
        // remove space and comment
        let (i, _) = parse_space_or_comment(text)?;
        if i.is_empty() {
            break;
        }
        let (i, global) = opt(parse_global)(i)?;
        if let Some(global) = global {
            unsafe {
                GLOBALS.push(global);
            }
            text = i;
        } else {
            let (i, func) = parse_function(i)?;
            funcs.push(func);
            text = i;
        }
    }
    Ok((text, (funcs, unsafe { GLOBALS.clone() })))
}

fn parse_global(text: &str) -> IResult<&str, Scope, VerboseError<&str>> {
    unsafe { IS_GLOBAL = true }
    let (i, ty) = parse_declspec(text)?;
    let (i, _) = multispace0(i)?;

    // if declaration only
    let (i, s) = opt(tag(";"))(i)?;
    if s.is_some() {
        return Ok((
            i,
            Scope {
                kind: ScopeKind::Null,
                ty: None,
                name: "".to_string(),
                str: None,
                offset: None,
            },
        ));
    }

    let (i, (ident, ty)) = parse_declarator(i, ty)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = tag(";")(i)?;

    Ok((
        i,
        Scope {
            kind: ScopeKind::Var,
            ty: Some(Box::new(ty.clone())),
            name: ident.to_string(),
            str: None,
            offset: None,
        },
    ))
}

static mut LOCAL_OFFSET: u16 = 0;

fn parse_function(text: &str) -> IResult<&str, Func, VerboseError<&str>> {
    unsafe {
        IS_GLOBAL = false;
        LOCALS = vec![];
        LOCAL_OFFSET = 0;
    }
    let mut nodes: Vec<Node> = Vec::new();
    let (i, naked) = opt(tag("__naked__"))(text)?;
    let mut is_naked = false;
    if naked.is_some() {
        is_naked = true;
    }

    let (i, _) = multispace0(i)?;
    let (i, ty) = parse_declspec(i)?;
    let (i, _) = multispace0(i)?;
    let (i, (ident, _)) = parse_declarator(i, ty)?; // Ignore function return types
    let ty = create_func_type();

    let kind = if ident == "init" {
        FuncKind::Init
    } else if ident == "main" {
        FuncKind::Main
    } else {
        FuncKind::Other
    };

    let (i, _) = multispace0(i)?;
    let (i, _) = tag("(")(i)?;
    let (i, args) = separated_list0(
        permutation((multispace0, tag(","), multispace0)),
        parse_funcparam,
    )(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = tag(")")(i)?;
    let (i, _) = multispace0(i)?;
    let (mut t, _) = tag("{")(i)?;

    loop {
        let (i, _) = parse_space_or_comment(t)?;
        let (i, s) = opt(tag("}"))(i)?;
        if s.is_some() {
            t = i;
            break;
        }
        let (i, node) = parse_stmt(i)?;
        nodes.push(node);
        t = i;
    }

    unsafe {
        Ok((
            t,
            Func {
                kind,
                ty: Box::new(ty),
                name: ident.to_string(),
                args,
                nodes,
                local_offset: LOCAL_OFFSET,
                is_naked,
            },
        ))
    }
}

// func_param = declspec declarator
fn parse_funcparam(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (i, ty) = parse_declspec(text)?;
    let (i, _) = multispace0(i)?;
    let (i, (ident, ty)) = parse_declarator(i, ty)?;
    unsafe {
        LOCAL_OFFSET += ty.size;
        let lvar = Scope {
            kind: ScopeKind::Var,
            ty: Some(Box::new(ty.clone())),
            name: ident.to_string(),
            offset: Some(LOCAL_OFFSET),
            str: None,
        };
        LOCALS.push(lvar);
    }
    Ok((
        i,
        Node {
            kind: NodeKind::Null,
            ty: Some(Box::new(ty)),
        },
    ))
}

// stmt = expr ";"
//       | "{" stmt* "}"
//       | "return" expr ";"
//       | "if" "(" expr ")" stmt ("else" stmt)?
//       | "while" "(" expr ")" stmt
//       | "for" "(" (declaration | expr)? ";" expr? ";" expr? ")" stmt
//       | "break" ";"
//       | "continue" ";"
//       | declaration ";"
//       | "__asm__" "(" assembler ")" ;"
fn parse_stmt(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (i, s) = opt(alt((
        tag("return"),
        tag("if"),
        tag("while"),
        tag("for"),
        tag("break"),
        tag("continue"),
        tag("{"),
        tag("__asm__"),
    )))(text)?;
    if let Some(s) = s {
        match s {
            "return" => {
                let (i, _) = multispace0(i)?;
                let (i, node) = parse_expr(i)?;
                let (i, _) = multispace0(i)?;
                let (i, _) = tag(";")(i)?;
                Ok((
                    i,
                    Node {
                        kind: NodeKind::Return(Return {
                            expr: Box::new(node),
                        }),
                        ty: None,
                    },
                ))
            }
            "if" => unsafe {
                let label = CTR_LABEL_COUNT;
                CTR_LABEL_COUNT += 1;

                let (i, _) = multispace0(i)?;
                let (i, cond) = delimited(
                    tag("("),
                    delimited(multispace0, parse_expr, multispace0),
                    tag(")"),
                )(i)?;
                let (i, _) = multispace0(i)?;
                let (i, then) = parse_stmt(i)?;
                let (i, _) = multispace0(i)?;
                let (i, s) = opt(tag("else"))(i)?;
                if s.is_some() {
                    let (i, _) = multispace0(i)?;
                    let (i, els) = parse_stmt(i)?;
                    Ok((
                        i,
                        Node {
                            kind: NodeKind::If(If {
                                cond: Box::new(cond),
                                then: Box::new(then),
                                els: Some(Box::new(els)),
                                label,
                            }),
                            ty: None,
                        },
                    ))
                } else {
                    Ok((
                        i,
                        Node {
                            kind: NodeKind::If(If {
                                cond: Box::new(cond),
                                then: Box::new(then),
                                els: None,
                                label,
                            }),
                            ty: None,
                        },
                    ))
                }
            },
            "while" => unsafe {
                let prev_label = SCOPE_CTR_LABEL;
                SCOPE_CTR_LABEL = CTR_LABEL_COUNT;
                CTR_LABEL_COUNT += 1;
                let label = SCOPE_CTR_LABEL;

                let (i, _) = multispace0(i)?;
                let (i, cond) = delimited(
                    tag("("),
                    delimited(multispace0, parse_expr, multispace0),
                    tag(")"),
                )(i)?;
                let (i, _) = multispace0(i)?;
                let (i, then) = parse_stmt(i)?;
                SCOPE_CTR_LABEL = prev_label;
                Ok((
                    i,
                    Node {
                        kind: NodeKind::While(While {
                            cond: Box::new(cond),
                            then: Box::new(then),
                            label,
                        }),
                        ty: None,
                    },
                ))
            },
            "for" => unsafe {
                let prev_label = SCOPE_CTR_LABEL;
                SCOPE_CTR_LABEL = CTR_LABEL_COUNT;
                CTR_LABEL_COUNT += 1;
                let label = SCOPE_CTR_LABEL;

                let mut init: Option<Box<Node>> = None;
                let mut cond: Option<Box<Node>> = None;
                let mut inc: Option<Box<Node>> = None;

                let (i, _) = multispace0(i)?;
                let (i, _) = tag("(")(i)?;
                let (i, _) = multispace0(i)?;
                let (mut i, s) = opt(tag(";"))(i)?;
                if s.is_none() {
                    let (j, expr) = alt((parse_declaration, parse_expr))(i)?;
                    init = Some(Box::new(expr));
                    let (j, _) = multispace0(j)?;
                    let (j, _) = tag(";")(j)?;
                    i = j;
                }
                let (i, _) = multispace0(i)?;
                let (mut i, s) = opt(tag(";"))(i)?;
                if s.is_none() {
                    let (j, expr) = parse_expr(i)?;
                    cond = Some(Box::new(expr));
                    let (j, _) = multispace0(j)?;
                    let (j, _) = tag(";")(j)?;
                    i = j;
                }
                let (i, _) = multispace0(i)?;
                let (mut i, s) = opt(tag(")"))(i)?;
                if s.is_none() {
                    let (j, expr) = parse_expr(i)?;
                    inc = Some(Box::new(expr));
                    let (j, _) = multispace0(j)?;
                    let (j, _) = tag(")")(j)?;
                    i = j;
                }
                let (i, _) = multispace0(i)?;
                let (i, stmt) = parse_stmt(i)?;
                let then = Some(Box::new(stmt));
                SCOPE_CTR_LABEL = prev_label;

                Ok((
                    i,
                    Node {
                        kind: NodeKind::For(For {
                            init,
                            cond,
                            inc,
                            then,
                            label,
                        }),
                        ty: None,
                    },
                ))
            },
            "break" => unsafe {
                let (i, _) = multispace0(i)?;
                let (i, _) = tag(";")(i)?;
                Ok((
                    i,
                    Node {
                        kind: NodeKind::Break(Break {
                            label: SCOPE_CTR_LABEL,
                        }),
                        ty: None,
                    },
                ))
            },
            "continue" => unsafe {
                let (i, _) = multispace0(i)?;
                let (i, _) = tag(";")(i)?;
                Ok((
                    i,
                    Node {
                        kind: NodeKind::Continue(Continue {
                            label: SCOPE_CTR_LABEL,
                        }),
                        ty: None,
                    },
                ))
            },
            "{" => {
                let mut t = i;
                let mut body: Vec<Node> = Vec::new();
                loop {
                    let (i, _) = parse_space_or_comment(t)?;
                    let (i, s) = opt(tag("}"))(i)?;
                    if s.is_some() {
                        return Ok((
                            i,
                            Node {
                                kind: NodeKind::Block(Block { body }),
                                ty: None,
                            },
                        ));
                    };
                    let (i, stmt) = parse_stmt(i)?;
                    body.push(stmt);
                    t = i;
                }
            }
            "__asm__" => {
                let (i, _) = tag("(")(i)?;
                let (i, _) = tag("\"")(i)?;
                let (i, asm) = take_until("\"")(i)?;
                let (i, _) = tag("\"")(i)?;
                let (i, _) = tag(")")(i)?;
                let (i, _) = multispace0(i)?;
                let (i, _) = tag(";")(i)?;
                Ok((
                    i,
                    Node {
                        kind: NodeKind::Asm(Asm {
                            asm: asm.to_string(),
                        }),
                        ty: None,
                    },
                ))
            }

            _ => {
                unreachable!()
            }
        }
    } else {
        let (i, node) = opt(parse_declaration)(text)?;
        if let Some(node) = node {
            let (i, _) = multispace0(i)?;
            let (i, _) = tag(";")(i)?;
            Ok((i, node))
        } else {
            let (i, node) = parse_expr(i)?;
            let (i, _) = multispace0(i)?;
            let (i, _) = tag(";")(i)?;
            Ok((i, node))
        }
    }
}

// declaration = declspec (ident ("[" num "]")* | ("=" expr ))?
fn parse_declaration(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (i, ty) = parse_declspec(text)?;
    let (i, _) = multispace0(i)?;

    // if declaration only
    let (_, s) = opt(tag(";"))(i)?;
    if s.is_some() {
        return Ok((
            i,
            Node {
                kind: NodeKind::Null,
                ty: None,
            },
        ));
    }

    let (i, (ident, ty)) = parse_declarator(i, ty)?;
    unsafe {
        LOCAL_OFFSET += ty.size;
        let lvar = Scope {
            kind: ScopeKind::Var,
            ty: Some(Box::new(ty.clone())),
            name: ident.to_string(),
            offset: Some(LOCAL_OFFSET),
            str: None,
        };
        LOCALS.push(lvar);
    }

    let (i, _) = multispace0(i)?;
    let (i, s) = opt(tag("="))(i)?;

    if s.is_some() {
        unsafe {
            let left = Node {
                kind: NodeKind::Var(Var {
                    name: ident.to_string(),
                    offset: LOCAL_OFFSET,
                    is_global: false,
                }),
                ty: Some(Box::new(ty.clone())),
            };

            let (i, _) = multispace0(i)?;
            let (i, node) = parse_expr(i)?;

            Ok((
                i,
                Node {
                    kind: NodeKind::Assign(Op {
                        left: Box::new(left),
                        right: Box::new(node),
                    }),
                    ty: Some(Box::new(ty)),
                },
            ))
        }
    } else {
        Ok((
            i,
            Node {
                kind: NodeKind::Null,
                ty: None,
            },
        ))
    }
}

// expr = assign
fn parse_expr(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    parse_assign(text)
}

// assign = logor ("=" assign)?
fn parse_assign(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (i, mut node) = parse_logor(text)?;
    let (i, _) = multispace0(i)?;
    let (i, s) = opt(tag("="))(i)?;
    if s.is_some() {
        let (i, _) = multispace0(i)?;
        let (i, right) = parse_assign(i)?;
        let ty = node.ty.clone();
        node = Node {
            kind: NodeKind::Assign(Op {
                left: Box::new(node),
                right: Box::new(right),
            }),
            ty,
        };
        Ok((i, node))
    } else {
        Ok((i, node))
    }
}

// logor = logand ("||" logand)*
fn parse_logor(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (mut t, mut node) = parse_logand(text)?;

    loop {
        let (i, _) = multispace0(t)?;
        let (i, s) = opt(tag("||"))(i)?;
        if s.is_some() {
            let (i, _) = multispace0(i)?;
            let (i, right) = parse_logand(i)?;
            unsafe {
                let label = CTR_LABEL_COUNT;
                CTR_LABEL_COUNT += 1;
                node = Node {
                    kind: NodeKind::LogOr(Log {
                        left: Box::new(node),
                        right: Box::new(right),
                        label,
                    }),
                    ty: Some(Box::new(create_int_type())),
                };
            }
            t = i;
        } else {
            return Ok((i, node));
        }
    }
}

// logand = bitor ("&&" bitor)*
fn parse_logand(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (mut t, mut node) = parse_bitor(text)?;

    loop {
        let (i, _) = multispace0(t)?;
        let (i, s) = opt(tag("&&"))(i)?;
        if s.is_some() {
            let (i, _) = multispace0(i)?;
            let (i, right) = parse_bitor(i)?;
            unsafe {
                let label = CTR_LABEL_COUNT;
                CTR_LABEL_COUNT += 1;
                node = Node {
                    kind: NodeKind::LogAnd(Log {
                        left: Box::new(node),
                        right: Box::new(right),
                        label,
                    }),
                    ty: Some(Box::new(create_int_type())),
                };
            }
            t = i;
        } else {
            return Ok((i, node));
        }
    }
}

// bitor = bitxor ("|" bitxor)*
fn parse_bitor(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (mut t, mut node) = parse_bitxor(text)?;

    loop {
        let (i, _) = multispace0(t)?;
        let (i, s) = opt(alt((tag("||"), tag("|"))))(i)?;
        if let Some(s) = s {
            if s == "|" {
                let (i, _) = multispace0(i)?;
                let (i, right) = parse_bitxor(i)?;
                node = Node {
                    kind: NodeKind::BitOr(Op {
                        left: Box::new(node),
                        right: Box::new(right),
                    }),
                    ty: Some(Box::new(create_int_type())),
                };
                t = i;
            } else {
                // "||" is logor
                return Ok((t, node));
            }
        } else {
            return Ok((i, node));
        }
    }
}

// bitxor = bitand ("^" bitand)*
fn parse_bitxor(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (mut t, mut node) = parse_bitand(text)?;

    loop {
        let (i, _) = multispace0(t)?;
        let (i, s) = opt(tag("^"))(i)?;
        if s.is_some() {
            let (i, _) = multispace0(i)?;
            let (i, right) = parse_bitand(i)?;
            node = Node {
                kind: NodeKind::BitXor(Op {
                    left: Box::new(node),
                    right: Box::new(right),
                }),
                ty: Some(Box::new(create_int_type())),
            };
            t = i;
        } else {
            return Ok((i, node));
        }
    }
}

// bitand = equality ("&" equality)*
fn parse_bitand(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (mut t, mut node) = parse_equality(text)?;

    loop {
        let (i, _) = multispace0(t)?;
        let (i, s) = opt(alt((tag("&&"), tag("&"))))(i)?;
        if let Some(s) = s {
            if s == "&" {
                let (i, _) = multispace0(i)?;
                let (i, right) = parse_equality(i)?;
                node = Node {
                    kind: NodeKind::BitAnd(Op {
                        left: Box::new(node),
                        right: Box::new(right),
                    }),
                    ty: Some(Box::new(create_int_type())),
                };
                t = i;
            } else {
                // "&&" is logand
                return Ok((t, node));
            }
        } else {
            return Ok((i, node));
        }
    }
}

// equality = relational ("==" relational | "!=" relational)*
fn parse_equality(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (mut t, mut node) = parse_relational(text)?;

    loop {
        let (i, _) = multispace0(t)?;
        let (i, s) = opt(alt((tag("=="), tag("!="))))(i)?;

        if let Some(s) = s {
            let (i, _) = multispace0(i)?;
            let (i, right) = parse_relational(i)?;
            if s == "==" {
                node = Node {
                    kind: NodeKind::Eq(Op {
                        left: Box::new(node),
                        right: Box::new(right),
                    }),
                    ty: Some(Box::new(create_int_type())),
                }
            } else {
                node = Node {
                    kind: NodeKind::Ne(Op {
                        left: Box::new(node),
                        right: Box::new(right),
                    }),
                    ty: Some(Box::new(create_int_type())),
                }
            }
            t = i;
        } else {
            return Ok((i, node));
        }
    }
}

// relational = add ("<" add | "<=" add | ">" add | ">=" add)*
fn parse_relational(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (mut t, mut node) = parse_add(text)?;

    loop {
        let (i, _) = multispace0(t)?;
        let (i, s) = opt(alt((tag("<="), tag(">="), tag("<"), tag(">"))))(i)?;

        if let Some(s) = s {
            let (i, _) = multispace0(i)?;
            let (i, right) = parse_add(i)?;
            if s == "<" {
                node = Node {
                    kind: NodeKind::Lt(Op {
                        left: Box::new(node),
                        right: Box::new(right),
                    }),
                    ty: Some(Box::new(create_int_type())),
                }
            } else if s == "<=" {
                node = Node {
                    kind: NodeKind::Le(Op {
                        left: Box::new(node),
                        right: Box::new(right),
                    }),
                    ty: Some(Box::new(create_int_type())),
                }
            } else if s == ">" {
                node = Node {
                    kind: NodeKind::Lt(Op {
                        left: Box::new(right),
                        right: Box::new(node),
                    }),
                    ty: Some(Box::new(create_int_type())),
                }
            } else {
                node = Node {
                    kind: NodeKind::Le(Op {
                        left: Box::new(right),
                        right: Box::new(node),
                    }),
                    ty: Some(Box::new(create_int_type())),
                }
            }
            t = i;
        } else {
            return Ok((i, node));
        }
    }
}

// add = mul ("+" mul | "-" mul)*
fn parse_add(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (mut t, mut node) = parse_mul(text)?;

    loop {
        let (i, _) = multispace0(t)?;
        let (i, s) = opt(alt((tag("+"), tag("-"))))(i)?;
        if let Some(s) = s {
            let (i, _) = multispace0(i)?;
            let (i, right) = parse_mul(i)?;
            if s == "+" {
                if node.ty.is_none() || right.ty.is_none() {
                    return context("Invalid operands", fail)(i);
                }
                if (node.ty.clone().unwrap().kind == TypeKind::Int
                    || node.ty.clone().unwrap().kind == TypeKind::Char)
                    && (right.ty.clone().unwrap().kind == TypeKind::Int
                        || right.ty.clone().unwrap().kind == TypeKind::Char)
                {
                    node = Node {
                        kind: NodeKind::Add(Op {
                            left: Box::new(node),
                            right: Box::new(right),
                        }),
                        ty: Some(Box::new(create_int_type())),
                    }
                } else if (node.ty.clone().unwrap().kind == TypeKind::Pointer
                    || node.ty.clone().unwrap().kind == TypeKind::Array)
                    && (right.ty.clone().unwrap().kind == TypeKind::Int
                        || right.ty.clone().unwrap().kind == TypeKind::Char)
                {
                    let ty = node.ty.clone();
                    node = Node {
                        kind: NodeKind::PtrAdd(PtrOp {
                            left: Box::new(node),
                            right: Box::new(right),
                        }),
                        ty,
                    }
                } else if (node.ty.clone().unwrap().kind == TypeKind::Int
                    || node.ty.clone().unwrap().kind == TypeKind::Char)
                    && (right.ty.clone().unwrap().kind == TypeKind::Pointer
                        || right.ty.clone().unwrap().kind == TypeKind::Array)
                {
                    let ty = right.ty.clone();
                    node = Node {
                        kind: NodeKind::PtrAdd(PtrOp {
                            left: Box::new(right),
                            right: Box::new(node),
                        }),
                        ty,
                    }
                } else {
                    return context("Invalid operands", fail)(i);
                }
            } else {
                if node.ty.is_none() || right.ty.is_none() {
                    return context("Invalid operands", fail)(i);
                }
                if (node.ty.clone().unwrap().kind == TypeKind::Int
                    || node.ty.clone().unwrap().kind == TypeKind::Char)
                    && (right.ty.clone().unwrap().kind == TypeKind::Int
                        || right.ty.clone().unwrap().kind == TypeKind::Char)
                {
                    node = Node {
                        kind: NodeKind::Sub(Op {
                            left: Box::new(node),
                            right: Box::new(right),
                        }),
                        ty: Some(Box::new(create_int_type())),
                    }
                } else if node.ty.clone().unwrap().kind == TypeKind::Pointer
                    && (right.ty.clone().unwrap().kind == TypeKind::Int
                        || right.ty.clone().unwrap().kind == TypeKind::Char)
                {
                    let ty = node.ty.clone();
                    node = Node {
                        kind: NodeKind::PtrSub(PtrOp {
                            left: Box::new(node),
                            right: Box::new(right),
                        }),
                        ty,
                    }
                } else {
                    return context("Invalid operands", fail)(i);
                }
            }
            t = i;
        } else {
            return Ok((i, node));
        }
    }
}

// mul = unary ("*" unary | "/" unary)*
fn parse_mul(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (mut t, mut node) = parse_unary(text)?;

    loop {
        let (i, _) = multispace0(t)?;
        let (i, s) = opt(alt((tag("*"), tag("/"))))(i)?;
        if let Some(s) = s {
            let (i, _) = multispace0(i)?;
            let (i, right) = parse_unary(i)?;
            if s == "*" {
                node = Node {
                    kind: NodeKind::Mul(Op {
                        left: Box::new(node),
                        right: Box::new(right),
                    }),
                    ty: Some(Box::new(create_int_type())),
                }
            } else {
                node = Node {
                    kind: NodeKind::Div(Op {
                        left: Box::new(node),
                        right: Box::new(right),
                    }),
                    ty: Some(Box::new(create_int_type())),
                }
            }
            t = i;
        } else {
            return Ok((i, node));
        }
    }
}

// unary = ("+" | "-")? primary ("[" expr "]" | "." ident | "->" ident)*
//       | "*" unary
//       | "&" unary
//       | "sizeof" unary
fn parse_unary(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (i, s) = opt(alt((tag("+"), tag("-"), tag("&"), tag("*"), tag("sizeof"))))(text)?;

    if let Some(s) = s {
        if s == "+" {
            Ok(parse_primary(i)?)
        } else if s == "-" {
            let left = Node {
                kind: NodeKind::Num(Num { val: 0 }),
                ty: Some(Box::new(create_int_type())),
            };
            let (i, right) = parse_primary(i)?;
            Ok((
                i,
                Node {
                    kind: NodeKind::Sub(Op {
                        left: Box::new(left),
                        right: Box::new(right),
                    }),
                    ty: Some(Box::new(create_int_type())),
                },
            ))
        } else if s == "&" {
            let (i, unary) = parse_unary(i)?;
            let ty = unary.ty.clone();
            Ok((
                i,
                Node {
                    kind: NodeKind::Addr(Addr {
                        unary: Box::new(unary),
                    }),
                    ty: Some(Box::new(Type {
                        kind: TypeKind::Pointer,
                        ptr_to: ty,
                        size: 2,
                        members: None,
                    })),
                },
            ))
        } else if s == "*" {
            let (i, unary) = parse_unary(i)?;
            if unary.ty.clone().unwrap().kind != TypeKind::Pointer
                && unary.ty.clone().unwrap().kind != TypeKind::Array
            {
                context("Invalid pointer dereference", fail)(i)
            } else {
                let ty = unary.ty.clone();
                Ok((
                    i,
                    Node {
                        kind: NodeKind::Deref(Deref {
                            unary: Box::new(unary),
                        }),
                        ty: ty.unwrap().ptr_to,
                    },
                ))
            }
        } else {
            let (i, _) = multispace0(i)?;
            let (i, node) = parse_unary(i)?;
            let ty = node.ty;
            match ty {
                Some(ty) => Ok((
                    i,
                    Node {
                        kind: NodeKind::Num(Num { val: ty.size }),
                        ty: Some(Box::new(create_int_type())),
                    },
                )),
                None => context("Unable to calculate", fail)(i),
            }
        }
    } else {
        let (mut t, mut node) = parse_primary(i)?;
        loop {
            let (i, _) = multispace0(t)?;
            let (i, s) = opt(alt((tag("."), tag("["), tag("->"))))(i)?;
            if let Some(s) = s {
                match s {
                    "[" => {
                        let (i, _) = multispace0(i)?;
                        let (i, right) = parse_expr(i)?;
                        let ty = node.clone().ty;
                        node = Node {
                            kind: NodeKind::PtrAdd(PtrOp {
                                left: Box::new(node),
                                right: Box::new(right),
                            }),
                            ty,
                        };
                        let ty = node.clone().ty;
                        node = Node {
                            kind: NodeKind::Deref(Deref {
                                unary: Box::new(node),
                            }),
                            ty: ty.unwrap().ptr_to,
                        };
                        let (i, _) = multispace0(i)?;
                        let (i, _) = tag("]")(i)?;
                        t = i;
                    }
                    "." => {
                        let ty = node.clone().ty.unwrap();
                        if ty.kind != TypeKind::Struct {
                            return context("Not a struct", fail)(i);
                        }
                        let (i, ident) = take_while1(is_ident)(i)?;
                        if let Some(mem) = ty
                            .members
                            .unwrap()
                            .iter()
                            .find(|&member| member.name == ident)
                        {
                            node = Node {
                                kind: NodeKind::MemAccess(MemAccess {
                                    unary: Box::new(node),
                                    member: Box::new(mem.clone()),
                                }),
                                ty: Some(mem.ty.clone()),
                            }
                        } else {
                            return context("Not such member", fail)(i);
                        }
                        t = i;
                    }
                    "->" => {
                        let ty = node.clone().ty.unwrap();
                        node = Node {
                            kind: NodeKind::Deref(Deref {
                                unary: Box::new(node),
                            }),
                            ty: ty.clone().ptr_to,
                        };
                        let ty = ty.ptr_to.unwrap();
                        if ty.kind != TypeKind::Struct {
                            return context("Not a struct", fail)(i);
                        }
                        let (i, ident) = take_while1(is_ident)(i)?;
                        if let Some(mem) = ty
                            .members
                            .unwrap()
                            .iter()
                            .find(|&member| member.name == ident)
                        {
                            node = Node {
                                kind: NodeKind::MemAccess(MemAccess {
                                    unary: Box::new(node),
                                    member: Box::new(mem.clone()),
                                }),
                                ty: Some(mem.ty.clone()),
                            }
                        } else {
                            return context("Not such member", fail)(i);
                        }
                        t = i;
                    }
                    _ => {
                        unreachable!()
                    }
                }
            } else {
                return Ok((t, node));
            }
        }
    }
}

// primary = num | str |  ident ( "(" args ")" )? | "(" expr ")"
fn parse_primary(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (_, num) = opt(digit1)(text)?;
    if num.is_some() {
        return parse_num(text);
    }
    let (_, str) = opt(tag("\""))(text)?;
    if str.is_some() {
        return parse_str(text);
    }
    let (_, ident) = opt(take_while1(is_ident))(text)?;
    if ident.is_some() {
        return parse_ident(text);
    }
    let (_, bracket) = opt(tag("("))(text)?;
    if bracket.is_some() {
        return delimited(
            tag("("),
            delimited(multispace0, parse_expr, multispace0),
            tag(")"),
        )(text);
    }
    context("Unexpected symbol", fail)(text)
}

fn parse_num(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (i, s) = opt(alt((tag("0x"), tag("0b"))))(text)?;
    if let Some(s) = s {
        match s {
            "0x" => {
                let (i, val) = hex_digit1(i)?;
                let val: u16 = u16::from_str_radix(val, 16).unwrap();
                Ok((
                    i,
                    Node {
                        kind: NodeKind::Num(Num { val }),
                        ty: Some(Box::new(create_int_type())),
                    },
                ))
            }
            "0b" => {
                let (i, val) = many0(alt((tag("0"), tag("1"))))(i)?;
                let val = val.concat();
                let val: u16 = u16::from_str_radix(&val, 2).unwrap();
                Ok((
                    i,
                    Node {
                        kind: NodeKind::Num(Num { val }),
                        ty: Some(Box::new(create_int_type())),
                    },
                ))
            }
            _ => {
                unreachable!()
            }
        }
    } else {
        let (i, val) = digit1(text)?;
        let val: u16 = val.parse().unwrap();
        Ok((
            i,
            Node {
                kind: NodeKind::Num(Num { val }),
                ty: Some(Box::new(create_int_type())),
            },
        ))
    }
}

fn parse_str(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (i, _) = tag("\"")(text)?;
    let (i, str) = take_until("\"")(i)?;
    let (i, _) = tag("\"")(i)?;

    if !str.is_ascii() {
        return context("Not ascii", fail)(i);
    }
    let mut str = convert_escape_string(str);
    str.push('\0');

    unsafe {
        let label = format!("str{}", STR_LABEL_COUNT);
        STR_LABEL_COUNT += 1;

        let str_len = str.chars().count() as u16;
        let gstr = Scope {
            kind: ScopeKind::Str,
            name: label.clone(),
            ty: Some(Box::new(Type {
                kind: TypeKind::Array,
                ptr_to: Some(Box::new(Type {
                    kind: TypeKind::Char,
                    ptr_to: None,
                    size: 1,
                    members: None,
                })),
                size: str_len,
                members: None,
            })),
            str: Some(str.to_string()),
            offset: None,
        };

        GLOBALS.push(gstr.clone());

        Ok((
            i,
            Node {
                kind: NodeKind::Var(Var {
                    name: label,
                    offset: 0,
                    is_global: true,
                }),
                ty: Some(gstr.ty.unwrap()),
            },
        ))
    }
}

fn convert_escape_string(s: &str) -> String {
    let mut str = String::new();
    let mut is_escape = false;

    for c in s.chars() {
        if is_escape {
            match c {
                'a' => str.push(7 as char),
                'b' => str.push(8 as char),
                't' => str.push('\t'),
                'n' => str.push('\n'),
                'v' => str.push(11 as char),
                'f' => str.push(12 as char),
                'r' => str.push('\r'),
                '0' => str.push('\0'),
                _ => {
                    str.push(c);
                }
            }
            is_escape = false;
        } else if c == '\\' {
            is_escape = true;
        } else {
            str.push(c);
        }
    }
    str
}

fn parse_ident(text: &str) -> IResult<&str, Node, VerboseError<&str>> {
    let (i, ident) = take_while1(is_ident)(text)?;
    let (i, _) = multispace0(i)?;
    let (i, s) = opt(tag("("))(i)?;
    if s.is_some() {
        let (i, args) = separated_list0(
            permutation((multispace0, tag(","), multispace0)),
            parse_assign,
        )(i)?;
        let (i, _) = multispace0(i)?;
        let (i, _) = tag(")")(i)?;
        return Ok((
            i,
            Node {
                kind: NodeKind::FuncCall(FuncCall {
                    name: ident.to_string(),
                    args,
                }),
                ty: Some(Box::new(create_int_type())),
            },
        ));
    }
    unsafe {
        if let Some(lvar) = LOCALS
            .iter()
            .find(|&scope| scope.kind == ScopeKind::Var && scope.name == ident)
        {
            Ok((
                i,
                Node {
                    kind: NodeKind::Var(Var {
                        name: lvar.name.clone(),
                        offset: lvar.offset.unwrap(),
                        is_global: false,
                    }),
                    ty: Some(lvar.ty.clone().unwrap()),
                },
            ))
        } else if let Some(gvar) = GLOBALS
            .iter()
            .find(|&scope| scope.kind == ScopeKind::Var && scope.name == ident)
        {
            Ok((
                i,
                Node {
                    kind: NodeKind::Var(Var {
                        name: gvar.name.clone(),
                        offset: 0,
                        is_global: true,
                    }),
                    ty: Some(gvar.ty.clone().unwrap()),
                },
            ))
        } else {
            context("Undefined variable", fail)(i)
        }
    }
}
