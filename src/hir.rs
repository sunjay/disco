//! High-level IR - Completely Desugared AST

use crate::ast;

/// Represents an entire compilation unit
#[derive(Debug, PartialEq)]
pub struct Package<'a> {
    pub root: Module<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module<'a> {
    pub decls: Vec<Decl<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Decl<'a> {
    Import(ImportPath<'a>),
    Struct(Struct<'a>),
    Impl(Impl<'a>),
    Function(Function<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportPath<'a> {
    pub path: IdentPath<'a>,
    pub selection: ImportSelection<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportSelection<'a> {
    /// A specific list of names being imported
    Names(Vec<Ident<'a>>),
    /// A wildcard import (all items)
    All,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct<'a> {
    /// The name of the struct
    pub name: Ident<'a>,
    /// The fields of the struct
    pub fields: Vec<StructField<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField<'a> {
    pub name: Ident<'a>,
    pub ty: Ty<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Impl<'a> {
    /// The Self type of this impl block
    pub self_ty: Ty<'a>,
    /// The method decls of this impl block
    pub methods: Vec<Function<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a> {
    pub name: Ident<'a>,
    pub sig: FuncSig<'a>,
    pub body: Block<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncSig<'a> {
    pub params: Vec<FuncParam<'a>>,
    pub return_type: Ty<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncParam<'a> {
    pub name: Ident<'a>,
    pub ty: Ty<'a>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Block<'a> {
    pub decls: Vec<Decl<'a>>,
    pub stmts: Vec<Stmt<'a>>,
    /// The final statement of the block, used as the return value of the block
    pub ret: Option<Expr<'a>>,
}

impl<'a> Block<'a> {
    pub fn is_empty(&self) -> bool {
        let Block {decls, stmts, ret} = self;
        decls.is_empty() && stmts.is_empty() && ret.is_none()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<'a> {
    Cond(Cond<'a>),
    WhileLoop(WhileLoop<'a>),
    VarDecl(VarDecl<'a>),
    Expr(Expr<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoop<'a> {
    /// The condition for which the loop is expected to continue
    pub cond: Expr<'a>,
    /// The body of the loop, executed until the condition is false
    pub body: Block<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl<'a> {
    /// The identifier to assign a value to
    pub name: Ident<'a>,
    /// The type of the variable (or None if the type is to be inferred)
    pub ty: Option<Ty<'a>>,
    /// The expression for the value to assign to the variable
    pub expr: Expr<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    Assign(Box<Assign<'a>>),
    MethodCall(Box<MethodCall<'a>>),
    FieldAccess(Box<FieldAccess<'a>>),
    Cond(Box<Cond<'a>>),
    Call(FuncCall<'a>),
    Return(Option<Box<Expr<'a>>>),
    StructLiteral(StructLiteral<'a>),
    BStrLiteral(Vec<u8>),
    IntegerLiteral(IntegerLiteral<'a>),
    RealLiteral(f64),
    ComplexLiteral(f64),
    BoolLiteral(bool),
    UnitLiteral,
    SelfLiteral,
    Var(Ident<'a>),
}

/// An assignment expression in the form `<lvalue> = <value>`
#[derive(Debug, Clone, PartialEq)]
pub struct Assign<'a> {
    /// The left-hand expression to assign a value to
    pub lhs: LValue<'a>,
    /// The expression for the value to assign to the left-hand side
    pub expr: Expr<'a>,
}

/// Expressions that can be on the left-hand side of assignment
#[derive(Debug, Clone, PartialEq)]
pub enum LValue<'a> {
    FieldAccess(FieldAccess<'a>),
    Var(Ident<'a>),
}

/// A method call in the form `<expr> . <call-expr>`
#[derive(Debug, Clone, PartialEq)]
pub struct MethodCall<'a> {
    /// The expression of the left-hand side of the method call
    pub lhs: Expr<'a>,
    /// The method being called
    pub method_name: Ident<'a>,
    /// The arguments to the method call
    pub args: Vec<Expr<'a>>,
}

/// A field access in the form `<expr> . <ident>`
#[derive(Debug, Clone, PartialEq)]
pub struct FieldAccess<'a> {
    /// The expression of the left-hand side of the field access
    pub lhs: Expr<'a>,
    /// The field being accessed
    pub field: Ident<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cond<'a> {
    /// A list of (condition, body) that corresponds to:
    /// if cond1 { body1 } else if cond2 { body2 } ...
    ///
    /// This must be non-empty (or else there would be no condition).
    pub conds: Vec<(Expr<'a>, Block<'a>)>,
    /// The `else` clause (if any)
    pub else_body: Option<Block<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncCall<'a> {
    pub func_name: IdentPath<'a>,
    pub args: Vec<Expr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructLiteral<'a> {
    pub name: NamedTy<'a>,
    pub field_values: Vec<StructFieldValue<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructFieldValue<'a> {
    /// The name of the field
    pub name: Ident<'a>,
    /// The expression being assigned to the field
    pub value: Expr<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntegerLiteral<'a> {
    pub value: i64,
    /// You can append "int" or "real" to help disambiguate the literal
    /// e.g. 132int or 32real
    pub type_hint: Option<&'a str>,
}

/// A type explicitly named with an identifier or path (as opposited to (), [T], etc.)
#[derive(Debug, Clone, PartialEq)]
pub enum NamedTy<'a> {
    SelfType,
    Named(Ident<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ty<'a> {
    Unit,
    SelfType,
    Named(Ident<'a>),
}

impl<'a> From<&'a NamedTy<'a>> for Ty<'a> {
    fn from(ty: &'a NamedTy<'a>) -> Self {
        match ty {
            NamedTy::SelfType => Ty::SelfType,
            NamedTy::Named(name) => Ty::Named(name),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IdentPath<'a> {
    /// An absolute path, relative to either the crate root or some other package
    Absolute(Vec<Ident<'a>>, IdentPathBase<'a>),
    /// A path relative to the current module namespace.
    Relative(Vec<Ident<'a>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentPathBase<'a> {
    /// A package with the given name
    Package(Ident<'a>),
    /// The root of the current package
    Root,
}

pub type Ident<'a> = ast::Ident<'a>;
