#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Tokens {
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    ADDASSIGNMENT,
    SUBASSIGNMENT,
    MULASSIGNMENT,
    DIVASSIGNMENT,
    MODASSIGNMENT,
    LPARENTHESIS,
    RPARENTHESIS,
    EQUALS,
    LESS,
    LESSEQ,
    GREAT,
    GREATEQ,
    AND,
    OR,
    IDENTIFIER,
    INTEGERS,
    FLOATS,
    SEMICOLON,
    COMMA,
    LBRACKET,
    RBRACKET,
    EQUALITY,
    INEQUALITY,
    UNKNOWN
}