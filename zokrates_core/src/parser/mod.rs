mod tokenize;
mod error;
mod parse;

pub use parser::error::Error;
pub use parser::parse::parse_program;
use field::FieldPrime;

lalrpop_mod!(parser, "/src/parser/parser.rs");

#[test]
fn test_parsergen() {
    let prog =
r#"def foo(field a) -> (field, field, field, field):
	field b = 12*a
	return a, 2*a, 5*b, a*b

def main(field i) -> (field):
	field x, field y, field z, field t = foo(i)
  return 1"#;

    let expr = parser::ProgramParser::new()
        .parse::<FieldPrime>(prog)
        .unwrap();
    //assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}

