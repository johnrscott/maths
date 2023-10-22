use std::fmt;

#[derive(PartialEq, Eq)]
pub struct Variable {
    name: String,
}

impl Variable {
    pub fn new(name: String) -> Self {
	Self {name}
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	write!(f, "{}", self.name)
    }
}

macro_rules! var {
    ($name:expr) => {Rc::new(Variable::new(format!($name)))}
}

pub(crate) use var;
