mod tokenize;
mod error;
mod parse;

pub use parser::error::Error;
pub use parser::parse::parse_program;
use field::FieldPrime;

lalrpop_mod!(parser);

#[test]
fn test_parsergen() {
    let expr = parser::Expr::new()
        .parse::<FieldPrime>("22 * 44 + 66")
        .unwrap();
    //assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}
