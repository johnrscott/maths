use std::rc::Rc;
use std::fmt;
use crate::variable::Variable;
use crate::formula::Formula;

pub enum Quantifier {
    Universal(Rc<Variable>, Rc<Formula>),
    Existential(Rc<Variable>, Rc<Formula>),
}

impl Quantifier {
    pub fn universal(variable: Rc<Variable>, formula: Rc<Formula>) -> Self {
	Self::Universal(variable, formula)
    }
    pub fn existent(variable: Rc<Variable>, formula: Rc<Formula>) -> Self {
	Self::Existential(variable, formula)
    }
}

impl fmt::Display for Quantifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	const for_all: &str = "\u{2200}";
	const exists: &str = "\u{2203}";
	match self {
	    Self::Universal(variable, formula) => write!(f, "{for_all}{variable}[{formula}]"),
	    Self::Existential(variable, formula) => write!(f, "{exists}{variable}[{formula}]"),
	}
    }
}

macro_rules! for_all {
    ($var:expr, $pred:expr) => {
	Rc::new(Quantifier::universal($var.clone(), $pred.clone()))
    }
}

pub(crate) use for_all;

macro_rules! there_exists {
    ($var:expr, $pred:expr) => {
	Rc::new(Quantifier::existent($var.clone(), $pred.clone()))
    }
}

pub(crate) use there_exists;
