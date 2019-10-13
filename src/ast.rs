//! The abstract syntax tree of the program.
//!
//! This is the closest representation to the actual syntax.

mod parser;

pub use parser::Error as ParseError;

#[derive(Debug, PartialEq)]
pub struct Program<'a> {
    pub top_level_module: Module<'a>,
}

impl<'a> Program<'a> {
    pub fn parse(input: &'a str) -> Result<Self, ParseError> {
        Ok(Program {
            top_level_module: parser::parse_module(input)?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Module<'a> {
    pub decls: Vec<Decl<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Decl<'a> {
    Function(Function<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Function<'a> {
    /// The name of the function
    pub name: Ident<'a>,
    /// The type signature of the function
    pub sig: FuncSig<'a>,
    /// The body of the function. Not used if `is_extern` is true.
    pub body: Block<'a>,
    /// True if the function is meant to be linked in externally
    pub is_extern: bool,
}

impl<'a> Function<'a> {
    pub fn new_extern(name: &'a str, sig: FuncSig<'a>) -> Self {
        Self {
            name,
            sig,
            body: Block::default(),
            is_extern: true,
        }
    }
}

/// The type signature of a function
#[derive(Debug, PartialEq)]
pub struct FuncSig<'a> {
    pub return_type: Ty<'a>,
    pub params: Vec<FuncParam<'a>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncParam<'a> {
    pub name: Ident<'a>,
    pub ty: Ty<'a>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Block<'a> {
    pub stmts: Vec<Stmt<'a>>,
    /// The final statement of the block, used as the return value of the block
    pub ret: Option<Expr<'a>>,
}

impl<'a> Block<'a> {
    pub fn is_empty(&self) -> bool {
        let Block {stmts, ret} = self;
        stmts.is_empty() && ret.is_none()
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
    pub ident: Ident<'a>,
    /// The type of the variable (or None if the type is to be inferred)
    pub ty: Option<Ty<'a>>,
    /// The expression for the value to assign to the variable
    pub expr: Expr<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    VarAssign(Box<VarAssign<'a>>),
    MethodCall(Box<MethodCall<'a>>),
    Cond(Box<Cond<'a>>),
    Call(CallExpr<'a>),
    Return(Option<Box<Expr<'a>>>),
    BStrLiteral(Vec<u8>),
    IntegerLiteral(IntegerLiteral<'a>),
    RealLiteral(f64),
    ComplexLiteral(f64),
    BoolLiteral(bool),
    UnitLiteral,
    Var(Ident<'a>),
}

/// An assignment expression in the form `<name> = <value>`
#[derive(Debug, Clone, PartialEq)]
pub struct VarAssign<'a> {
    /// The identifier to assign a value to
    pub ident: Ident<'a>,
    /// The expression for the value to assign to the variable
    pub expr: Expr<'a>,
}

/// A method call in the form `<expr> . <call-expr>`
#[derive(Debug, Clone, PartialEq)]
pub struct MethodCall<'a> {
    /// The expression of the left-hand side of the method call
    pub lhs: Expr<'a>,
    /// The method being called
    pub call: CallExpr<'a>,
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
pub struct CallExpr<'a> {
    pub func_name: Ident<'a>,
    pub args: Vec<Expr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntegerLiteral<'a> {
    pub value: i64,
    /// You can append "int" or "real" to help disambiguate the literal
    /// e.g. 132int or 32real
    pub type_hint: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ty<'a> {
    Unit,
    Named(Ident<'a>),
}

pub type Ident<'a> = &'a str;
