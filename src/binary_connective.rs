use std::rc::Rc;
use std::fmt;
use crate::formula::Formula;

pub enum BinaryConnective {
    Conjunction(Rc<Formula>, Rc<Formula>),
    Disjunction(Rc<Formula>, Rc<Formula>),
    Implication(Rc<Formula>, Rc<Formula>),
    Biconditional(Rc<Formula>, Rc<Formula>),
}

impl BinaryConnective {
    pub fn and(form1: Rc<Formula>, form2: Rc<Formula>) -> Self {
        Self::Conjunction(form1, form2)
    }
    pub fn or(form1: Rc<Formula>, form2: Rc<Formula>) -> Self {
        Self::Disjunction(form1, form2)
    }
    pub fn implies(form1: Rc<Formula>, form2: Rc<Formula>) -> Self {
        Self::Implication(form1, form2)
    }
    pub fn iff(form1: Rc<Formula>, form2: Rc<Formula>) -> Self {
        Self::Biconditional(form1, form2)
    }
}

impl fmt::Display for BinaryConnective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	const and: &str = "\u{22c0}";
	const or: &str = "\u{22c1}";
	const implies: &str = "\u{2192}";
	const iff: &str = "\u{2194}";
	match self {
            Self::Conjunction(form1, form2) => write!(f, "{form1} {and} {form2}"),
            Self::Disjunction(form1, form2) => write!(f, "{form1} {or} {form2}"),
            Self::Implication(form1, form2) => write!(f, "{form1} {implies} {form2}"),
            Self::Biconditional(form1, form2) => write!(f, "{form1} {iff} {form2}"),
        }
    }
}

macro_rules! and {
    ($form1:expr, $form2:expr) => {Rc::new(BinaryConnective::and($form1.clone(), $form2.clone()))} 
}

pub(crate) use and;

macro_rules! or {
    ($form1:expr, $form2:expr) => {Rc::new(BinaryConnective::or($form1.clone(), $form2.clone()))} 
}

pub(crate) use or;

macro_rules! implies {
    ($form1:expr, $form2:expr) => {Rc::new(BinaryConnective::implies($form1.clone(), $form2.clone()))} 
}

pub(crate) use implies;

macro_rules! equivalent {
    ($form1:expr, $form2:expr) => {Rc::new(BinaryConnective::iff($form1.clone(), $form2.clone()))} 
}

pub(crate) use equivalent;
