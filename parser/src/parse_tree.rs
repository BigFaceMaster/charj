use crate::location::{Loc, Location};

#[derive(Debug, PartialEq)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Debug, PartialEq)]
pub enum SourceUnitPart {
    ImportDirective(Import),
    MultipleImportDirective(Vec<Import>),
    PackageDirective(Package),
    StructFunctionDefinition(Box<StructFunctionDefinition>),
    FunctionDefinition(Box<FunctionDefinition>),
    StructDefinition(Box<StructDefinition>),
}

pub type Suite = Vec<Statement>;

#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
    pub loc: Loc,
    pub name: Identifier,
    pub params: Vec<(Loc, Option<Parameter>)>,
    pub body: Suite,
}

#[derive(Debug, PartialEq)]
pub struct StructFunctionDefinition {
    pub loc: Loc,
    pub name: Identifier,
    pub struct_name: Identifier,
    pub params: Vec<(Loc, Option<Parameter>)>,
    pub body: Suite,
}

#[derive(Debug, PartialEq, Default)]
pub struct Parameters {
    pub args: Vec<Parameter>,
}

/// A single formal parameter to a function.
#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub loc: Loc,
    pub ty: Expression,
    pub name: Option<Identifier>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Variable(Identifier),
}

#[derive(Debug, PartialEq)]
pub struct Located<T> {
    pub location: Location,
    pub node: T,
}

pub type Statement = Located<StatementType>;

#[derive(Debug, PartialEq)]
pub enum StatementType {
    Break,
    Continue,
    /// A [`pass`](https://docs.python.org/3/reference/simple_stmts.html#pass) statement.
    ///  thinking in python
    Pass,
    If,
}

#[derive(Debug, PartialEq)]
pub struct StructDefinition {
    pub loc: Loc,
    pub name: Identifier,
    pub fields: Vec<VariableDeclaration>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {}

#[derive(Debug, PartialEq)]
pub enum Package {
    Plain(Identifier),
}

#[derive(Debug, PartialEq)]
pub enum Import {
    Standard(Identifier),
    Remote, // for such github.com/phodal/coca
    GlobalSymbol(StringLiteral, Identifier),
    Rename(StringLiteral, Vec<(Identifier, Option<Identifier>)>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub loc: Loc,
    pub string: String,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Identifier {
    pub loc: Loc,
    pub name: String,
}
