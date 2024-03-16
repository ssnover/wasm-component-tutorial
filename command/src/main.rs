#[allow(warnings)]
mod bindings;

use bindings::docs::calculator::calculate::{eval_expression, Op};

fn main() {
    let mut args = std::env::args();
    if args.len() >= 4 {
        let operand1 = args.nth(2).unwrap().parse::<u32>().unwrap();
        let operand2 = args.nth(3).unwrap().parse::<u32>().unwrap();
        let op = args.nth(4).unwrap().as_str().to_string();
        if op.as_str() == "add" {
            let out = eval_expression(Op::Add, operand1, operand2);
            println!("{operand1} + {operand2} = {out}");
        }
    }
}
