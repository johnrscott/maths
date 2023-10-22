use crate::variable::Variable;
use crate::function::Function;
use std::fmt;
use std::rc::Rc;

pub enum Term {
    Variable(Rc<Variable>),
    Function(Rc<Function>),
}

impl Term {
    pub fn from_variable(var: Rc<Variable>) -> Self {
	Self::Variable(var)
    }
    pub fn from_function(func: Rc<Function>) -> Self {
	Self::Function(func)
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Variable(var) => write!(f, "{var}"),
            Self::Function(func) => write!(f, "{func}"),
        }
    }
}

macro_rules! var_term {
    ($var:expr) => {Rc::new(Term::from_variable($var.clone()))}
}

pub(crate) use var_term;
