mod term;
mod variable;
mod function;
mod predicate;
mod formula;
mod binary_connective;
mod quantifier;

use std::rc::Rc;
use variable::{var, Variable};
use predicate::{pred, Predicate};
use quantifier::{for_all, there_exists, Quantifier};
use formula::{pred_form, quant_form, Formula};
use binary_connective::{and, BinaryConnective};

fn main() {
    let x = var!("x");
    let y = var!("y");

    let p = pred!("P", x, y);

    let q = for_all!(x, pred_form!(p));
    let r = there_exists!(y, pred_form!(p));

    let s = and!(quant_form!(q), quant_form!(r));
    
    println!("{s}")
}
