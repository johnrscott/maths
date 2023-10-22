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
use formula::{pred_form, quant_form, bin_form, eq_form, not, Formula};
use binary_connective::{and, or, implies, equivalent, BinaryConnective};
use term::{var_term, Term};

fn main() {
    let x = var!("x");
    let y = var!("y");
    let z = var!("z");
    let a = var!("a");

    // Axiom of extensionality
    let x_equals_y = eq_form!(var_term!(x), var_term!(y));
    let z_memberof_x = pred!("MemberOf", z, x);
    let z_memberof_y = pred!("MemberOf", z, y);
    let equiv = equivalent!(pred_form!(z_memberof_x), pred_form!(z_memberof_y));
    let for_all_z = for_all!(z, bin_form!(equiv));
    let p = implies!(quant_form!(for_all_z), x_equals_y);
    let for_all_y = for_all!(y, bin_form!(p));
    let axiom_of_extensionality = for_all!(x, quant_form!(for_all_y));
    println!("{axiom_of_extensionality}");

    // Axiom of regularity
    let a_memberof_x = pred!("MemberOf", a, x);
    let x_nonempty = there_exists!(a, pred_form!(a_memberof_x));
    let z_in_y_and_z_in_x = and!(pred_form!(z_memberof_y), pred_form!(z_memberof_x));
    let no_z = not!(quant_form!(there_exists!(z, bin_form!(z_in_y_and_z_in_x))));
    let y_memberof_x = pred!("MemberOf", y, x);
    let y_in_x_and_no_z = and!(pred_form!(y_memberof_x), no_z);
    let there_exists_y = there_exists!(y, bin_form!(y_in_x_and_no_z));
    let nonempty_implies_is_y = implies!(quant_form!(x_nonempty), quant_form!(there_exists_y));
    let axiom_of_regularity = for_all!(x, bin_form!(nonempty_implies_is_y));
    println!("{axiom_of_regularity}")

	
}
