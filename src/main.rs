use lazy_static::lazy_static;
use pest::iterators::Pairs;
use pest::pratt_parser::{Assoc, Op, PrattParser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "demo.pest"]
struct MyParser;

lazy_static! {
  static ref PRATT_PARSER: PrattParser<Rule> = {
    use Assoc::*;
    use Rule::*;

    PrattParser::new()
      .op(Op::infix(add, Left) | Op::infix(subtract, Left))
      .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
      .op(Op::infix(power, Right))
  };
}

fn eval(expression: Pairs<Rule>) -> f64 {
  PRATT_PARSER
    .map_primary(|primary| match primary.as_rule() {
      Rule::num => primary.as_str().parse::<f64>().unwrap(),
      Rule::expr => eval(primary.into_inner()),
      _ => unreachable!(),
    })
    .map_infix(|lhs, op, rhs| match op.as_rule() {
      Rule::add => lhs + rhs,
      Rule::subtract => lhs - rhs,
      Rule::multiply => lhs * rhs,
      Rule::divide => lhs / rhs,
      Rule::power => lhs.powf(rhs),
      _ => unreachable!(),
    })
    .parse(expression)
}

fn main() {}
