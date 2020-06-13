use crate::{parse_inputs, parse_program};
use leo_ast::ParserError;
use leo_compiler::errors::CompilerError;
use leo_inputs::InputParserError;

#[test]
fn test_semicolon() {
    let bytes = include_bytes!("semicolon.leo");
    let error = parse_program(bytes).err().unwrap();

    match error {
        CompilerError::ParserError(ParserError::SyntaxError(_)) => {}
        _ => panic!("test_semicolon failed the wrong expected error, should be a ParserError"),
    }
}

#[test]
fn inputs_syntax_error() {
    let bytes = include_bytes!("inputs_semicolon.leo");
    let error = parse_inputs(bytes).err().unwrap();

    match error {
        CompilerError::InputParserError(InputParserError::SyntaxError(_)) => {}
        _ => panic!("inputs syntax error should be a ParserError"),
    }
}