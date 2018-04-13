use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Domain {
	Strings,
	Algebra,
	Sets,
	Boolean
}

impl fmt::Display for Domain {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    	match self {
    		Domain::Strings => { write!(formatter, "Strings") }
    	    Domain::Algebra => {write!(formatter, "Algebra")}
    		Domain::Sets => {write!(formatter, "Sets")}
    		Domain::Boolean => {write!(formatter, "Boolean")}
    	} 
    }
}
