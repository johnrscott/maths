use std::rc::Rc;
use std::fmt;
use crate::predicate::Predicate;
use crate::term::Term;
use crate::binary_connective::BinaryConnective;
use crate::quantifier::Quantifier;

/// Well-formed formula
pub enum Formula {
    Predicate(Rc<Predicate>),
    Equal(Rc<Term>, Rc<Term>), // Only if equality is part of the logic
    Not(Rc<Formula>),
    BinaryConnective(Rc<BinaryConnective>),
    Quantifier(Rc<Quantifier>),
}

impl Formula {
    pub fn from_predicate(pred: Rc<Predicate>) -> Self {
        Self::Predicate(pred)
    }
    pub fn equality(term1: Rc<Term>, term2: Rc<Term>) -> Self {
        Self::Equal(term1, term2)
    }
    pub fn negate(formula: Rc<Formula>) -> Self {
        Self::Not(formula)
    }
    pub fn from_binary_connective(binary_connective: Rc<BinaryConnective>) -> Self {
        Self::BinaryConnective(binary_connective)
    }
    pub fn from_quantifier(quantifier: Rc<Quantifier>) -> Self {
	Self::Quantifier(quantifier)	
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	const not: &str = "\u{00ac}";
	match self {
            Self::Predicate(pred) => write!(f, "{pred}"),
            Self::Equal(term1, term2) => write!(f, "[{term1} = {term2}]"),
            Self::Not(func) => write!(f, "{not}{func}"),
            Self::BinaryConnective(bin) => write!(f, "[{bin}]"),
	    Self::Quantifier(quant) => write!(f, "{quant}"),
	}
    }
}

/// Predicate formula
macro_rules! pred_form {
    ($pred: expr) => {Rc::new(Formula::from_predicate($pred.clone()))}
}

pub(crate) use pred_form;

/// Quantifier formula
macro_rules! quant_form {
    ($quant: expr) => {Rc::new(Formula::from_quantifier($quant.clone()))}
}

pub(crate) use quant_form;

/// Binary-connective formula
macro_rules! bin_form {
    ($quant: expr) => {Rc::new(Formula::from_binary_connective($quant.clone()))}
}

pub(crate) use bin_form;

/// Equality formula
macro_rules! eq_form {
    ($term1: expr, $term2: expr) => {Rc::new(Formula::equality($term1.clone(), $term2.clone()))}
}

pub(crate) use eq_form;

/// Negate formula
macro_rules! not {
    ($term: expr) => {Rc::new(Formula::negate($term.clone()))}
}

pub(crate) use not;
