use serde::{Deserialize, Serialize};

pub type NodeID = i64;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum NodeType {
    SourceUnit,
    PragmaDirective,
    ImportDirective,
    UsingForDirective,
    ContractDefinition,
    InheritanceSpecifier,
    OverrideSpecifier,
    IdentifierPath,
    StructuredDocumentation,
    VariableDeclaration,
    Mapping,
    ElementaryTypeName,
    ElementaryTypeNameExpression,
    ArrayTypeName,
    TupleExpression,
    FunctionDefinition,
    ParameterList,
    Block,
    UncheckedBlock,
    Continue,
    Break,
    Return,
    Throw,
    Literal,
    Conditional,
    Identifier,
    IndexAccess,
    IndexRangeAccess,
    MemberAccess,
    Assignment,
    FunctionCall,
    FunctionCallOptions,
    FunctionTypeName,
    NewExpression,
    ExpressionStatement,
    VariableDeclarationStatement,
    IfStatement,
    TryCatchClause,
    UnaryOperation,
    BinaryOperation,
    EventDefinition,
    ErrorDefinition,
    EmitStatement,
    PlaceholderStatement,
    TryStatement,
    RevertStatement,
    ForStatement,
    WhileStatement,
    ModifierDefinition,
    ModifierInvocation,
    EnumDefinition,
    EnumValue,
    StructDefinition,
    UserDefinedTypeName,
    InlineAssembly,
    YulLiteral,
    YulTypedName,
    YulSwitch,
    YulCase,
    YulFunctionCall,
    YulExpressionStatement,
    YulAssignment,
    YulIdentifier,
    YulVariableDeclaration,
    YulBlock,
    UserDefinedValueTypeDefinition,
}
