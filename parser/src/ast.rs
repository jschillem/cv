use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Function(FunctionDeclaration),
    Record(RecordDeclaration),
    Union(UnionDeclaration),
    Patch(PatchDeclaration),
    Statement(Statement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
    pub is_mutable: bool,
    pub is_ref: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordDeclaration {
    pub name: String,
    pub fields: Vec<RecordField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordField {
    pub name: String,
    pub field_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionDeclaration {
    pub name: String,
    pub tyne_parameters: Vec<String>, // for generics
    pub variants: Vec<UnionVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionVariant {
    pub name: String,
    pub variant_type: Option<Type>, // some variants may carry data
}

#[derive(Debug, Clone, PartialEq)]
pub struct PatchDeclaration {
    pub target_type: Type,
    pub methods: Vec<FunctionDeclaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        var_type: Option<Type>,
        is_mutable: bool,
        value: Box<Expression>,
    },
    Expression(Box<Expression>),
    Return(Option<Box<Expression>>),
    Break(Option<Box<Expression>>), // Break with optional value
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOperation {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    UnaryOperation {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    FunctionCall {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    RecordAccess {
        record: Box<Expression>,
        field: String,
    },
    IndexAccess {
        collection: Box<Expression>,
        index: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Option<Box<Expression>>,
    },
    When {
        expression: Box<Expression>,
        branches: Vec<WhenBranch>,
    },
    Block {
        statements: Vec<Statement>,
        final_expression: Option<Box<Expression>>, // Optional final expression
    },
    Loop {
        body: Box<Expression>,
    },
    While {
        condition: Box<Expression>,
        body: Box<Expression>,
    },
    For {
        variable: String,
        iterable: Box<Expression>,
        body: Box<Expression>,
    },
    ArrayLiteral(Vec<Expression>),
    RecordLiteral {
        record_type: Type,
        fields: Vec<(String, Expression)>,
    },
    UnionLiteral {
        union_type: Type,
        variant: String,
        value: Option<Box<Expression>>,
    },
    Reference {
        is_mutable: bool,
        expression: Box<Expression>,
    },
    Dereference(Box<Expression>),
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
        inclusive: bool,
    },
    TypeAnnotation {
        expression: Box<Expression>,
        annotated_type: Type,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhenBranch {
    pub pattern: Pattern,
    pub body: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Identifier(String),
    Literal(Literal),
    Union {
        variant: String,
        binding: Option<String>,
    },
    Wildcard,
    Else,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Char(char),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,

    // Logical
    And,
    Or,

    // Comparison
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    // Assignment
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModulusAssign,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOperator {
    Not,
    Negate,
    Reference,
    MutableReference,
    Dereference,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    ISize,
    USize,
    F32,
    F64,
    Bool,
    Char,
    String,
    Named(String),
    Generic {
        name: String,
        parameters: Vec<Type>,
    },
    Reference {
        is_mutable: bool,
        ref_type: Box<Type>,
    },
    ArrayList(Box<Type>),
    FixedArray {
        element_type: Box<Type>,
        size: usize,
    },
    Function {
        param_types: Vec<Type>,
        return_type: Option<Box<Type>>,
    },
    Inferred, // For type inference (e.g., let x = 5;
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::I8 => write!(f, "i8"),
            Type::I16 => write!(f, "i16"),
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
            Type::U8 => write!(f, "u8"),
            Type::U16 => write!(f, "u16"),
            Type::U32 => write!(f, "u32"),
            Type::U64 => write!(f, "u64"),
            Type::ISize => write!(f, "isize"),
            Type::USize => write!(f, "usize"),
            Type::F32 => write!(f, "f32"),
            Type::F64 => write!(f, "f64"),
            Type::Bool => write!(f, "bool"),
            Type::Char => write!(f, "char"),
            Type::String => write!(f, "string"),
            Type::Named(name) => write!(f, "{}", name),
            Type::Generic { name, parameters } => {
                let params: Vec<String> = parameters.iter().map(|p| p.to_string()).collect();
                write!(f, "{}<{}>", name, params.join(", "))
            }
            Type::Reference {
                is_mutable,
                ref_type,
            } => {
                if *is_mutable {
                    write!(f, "{}&@", ref_type)
                } else {
                    write!(f, "{}&@", ref_type)
                }
            }
            Type::ArrayList(element_type) => write!(f, "arrayList<{}>", element_type),
            Type::FixedArray { element_type, size } => {
                write!(f, "fixedArray<{}, {}>", element_type, size)
            }
            Type::Function {
                param_types,
                return_type,
            } => {
                write!(f, "fn(")?;
                let params: Vec<String> = param_types.iter().map(|p| p.to_string()).collect();
                write!(f, "{}", params.join(", "))?;
                if let Some(ret_type) = return_type {
                    write!(f, ") -> {}", ret_type)
                } else {
                    write!(f, ")")
                }
            }
            Type::Inferred => write!(f, "_"),
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Modulus => "%",
            BinaryOperator::And => "&&",
            BinaryOperator::Or => "||",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::LessThan => "<",
            BinaryOperator::LessThanOrEqual => "<=",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::GreaterThanOrEqual => ">=",
            BinaryOperator::Assign => "=",
            BinaryOperator::AddAssign => "+=",
            BinaryOperator::SubtractAssign => "-=",
            BinaryOperator::MultiplyAssign => "*=",
            BinaryOperator::DivideAssign => "/=",
            BinaryOperator::ModulusAssign => "%=",
        };
        write!(f, "{}", op_str)
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            UnaryOperator::Not => "!",
            UnaryOperator::Negate => "-",
            UnaryOperator::Reference => "&",
            UnaryOperator::MutableReference => "&@",
            UnaryOperator::Dereference => "*",
        };
        write!(f, "{}", op_str)
    }
}

impl Expression {
    pub fn is_lvalue(&self) -> bool {
        matches!(
            self,
            Expression::Identifier(_)
                | Expression::RecordAccess { .. }
                | Expression::IndexAccess { .. }
                | Expression::Dereference(_)
        )
    }

    pub fn requires_semicolon(&self) -> bool {
        !matches!(
            self,
            Expression::If { .. }
                | Expression::When { .. }
                | Expression::Block { .. }
                | Expression::Loop { .. }
                | Expression::While { .. }
                | Expression::For { .. }
        )
    }
}
