#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate quick_xml;
extern crate time;

use std::fmt;

use quick_xml::Reader;
use quick_xml::events::Event;

use std::collections::HashSet;

use time::PreciseTime;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(i64),
    Set(HashSet<i64>),
    Plus,
    Multiply,
    LeftMustache,
    RightMustache,
    LeftParentheses,
    RightParentheses
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Domain {
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

impl Token {
    fn get_weight(self) -> i64 {
        match self {
            Token::Plus => 1,
            Token::Multiply => 2,
            _ => 0,
        }
    }
}

fn resolve(domain: Domain, tokens: Vec<Token>) -> (i64, HashSet<i64>) {
	let mut stack: Vec<i64> = Vec::new();
	let mut set_stack: Vec<HashSet<i64>> = Vec::new();

	match domain {
		Domain::Algebra => {
			for token in tokens {
				match token {
					Token::Number(val) => stack.push(val),
					Token::Plus => {
						let a = stack.pop().unwrap();
						let b = stack.pop().unwrap();
						stack.push(a + b);
					}
					Token::Multiply => {
						let a = stack.pop().unwrap();
						let b = stack.pop().unwrap();
						stack.push(a * b);
					}
					_ => {}
				}
			}
		}
		Domain::Boolean => {
			for token in tokens {
				match token {
					Token::Number(val) => stack.push(val),
					Token::Plus => {
						let a = stack.pop().unwrap();
						let b = stack.pop().unwrap();
						stack.push(a | b);
					}
					Token::Multiply => {
						let a = stack.pop().unwrap();
						let b = stack.pop().unwrap();
						stack.push(a & b);
					}
					_ => {}
				}
			}
		}
		Domain::Sets => {
			for token in tokens {
				match token {
					Token::Set(inner_set) => set_stack.push(inner_set),
					Token::Plus => {
						let a = set_stack.pop().unwrap();
						let b = set_stack.pop().unwrap();

						let mut union: HashSet<i64> = HashSet::new();
						let temp: Vec<&i64> = a.union(&b).collect::<Vec<&i64>>();
						for u in temp { union.insert(*u); }
						set_stack.push(union);
					}
					Token::Multiply => {
						let a = set_stack.pop().unwrap();
						let b = set_stack.pop().unwrap();
						let mut intersection: HashSet<i64> = HashSet::new();
						let temp: Vec<&i64> = a.intersection(&b).collect::<Vec<&i64>>();
						for u in temp { intersection.insert(*u); }
						set_stack.push(intersection);
					}
					_ => {}
				}
			}
		}
		_ => {}
	}
	if domain == Domain::Sets {
		return (0, set_stack.pop().unwrap());
	} else {
		return (stack.pop().unwrap(), HashSet::new());
	}	
}

fn recognize_string_expression(
    number_stack: &mut Vec<i64>, 
    operation_stack: &mut Vec<Token>
) -> String {
    let mut string_to_build = String::from("");
    /* Lets pop from the stacks and rebuild the expression */
    while let Some(number) = number_stack.pop() {
        if let Some(operation) = operation_stack.pop() {
            match operation {
                Token::Plus => {
                    string_to_build.push_str(&format!("{}", number));
                    string_to_build.push_str(" + ");
                } 
                Token::Multiply => {
                    if let Some(next_num) = number_stack.pop() {
                        string_to_build.push_str(&format!("{}", next_num));
                        for i in 1..number {
                             string_to_build.push_str(" + "); 
                             string_to_build.push_str(&format!("{}", next_num));

                        }
                    }
                }
                _ => {/* TODO: Handle Parentheses */}
            }
        } else {
            string_to_build.push_str(&format!("{}", number));
        }
    }
    string_to_build
}

/* The Shunting-Yard Algorithm produces Reverse Polish Notation Vec<Token> */
fn shunting_yard(domain: Domain, tokens: Vec<Token>) -> Vec<Token> {
	let mut rpn_stack: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();

    let mut set: HashSet<i64> = HashSet::new();

    for token in tokens {
        match token {
        	Token::Set(val) => {
        		rpn_stack.push(Token::Set(val.clone()));
        	}
            Token::Number(val) => {
            	if domain == Domain::Sets {
            		set.insert(val);
            	} else {
            		rpn_stack.push(token);
            	}	
            },
            Token::Plus | Token::Multiply => {
                while let Some(o) = stack.pop() {
                    if token.clone().get_weight() <= o.clone().get_weight() { /* If our token is higher precedence, do the thing */
                        rpn_stack.push(o);
                    } else {
                        stack.push(o);
                        break;
                    }
                }
                stack.push(token)
            },
            
            Token::LeftMustache => stack.push(token),
            Token::RightMustache => {
            	let mut mustache = false;
            	while let Some(op) = stack.pop() {
            		match op {
            			Token::LeftMustache => {
            				mustache = true;
			    			rpn_stack.push(Token::Set(set.clone()));
			    			set.clear(); /* Clear the previous set of integers */
            				break;
            			},
            			_ => rpn_stack.push(op),
            		}
            	}
            	assert!(mustache)
            }
            Token::LeftParentheses => stack.push(token),
            Token::RightParentheses => {
                let mut parentheses = false;
                while let Some(op) = stack.pop() {
                    match op {
                        Token::LeftParentheses => {
                            parentheses = true;
                            break;
                        },
                        _ => rpn_stack.push(op),
                    }
                }
                assert!(parentheses)
            },
        }
    }
    while let Some(op) = stack.pop() {
        rpn_stack.push(op);
    }
    rpn_stack
}

fn tokenize_string(text: String, numbers: &mut Vec<i64>, operators: &mut Vec<Token>) {
	let mut is_left: bool = true;
    
    let mut iterator = text.chars().peekable();
    while let Some(&c) = iterator.peek() {
        match c {
            '0' ... '9' => { 
                iterator.next();
                numbers.push(c.to_string().parse::<i64>().unwrap());
            }
            '+' => { 
                iterator.next();
                operators.push(Token::Plus);                
            }
            '*' => { 
                iterator.next();
                operators.push(Token::Multiply);
            }
            _ => {
                /* Sink others */
                iterator.next();
            }
        }
    }
}

fn tokenize(text: String) -> Vec<Vec<Token>> {
	let mut col_tokens: Vec<Vec<Token>> = Vec::new();
	for s_text in text.split("=") {
		let mut iterator = s_text.trim().chars().peekable();
		let mut tokens: Vec<Token> = Vec::new();

	    while let Some(&c) = iterator.peek() {
	        match c {
	            '0' ... '9' => { 
	                iterator.next();
	                tokens.push(Token::Number(c.to_string().parse::<i64>().unwrap()));                            
	            }
	            /* (1 + 1) * */
	            ')' => {
	                iterator.next();
	                tokens.push(Token::RightParentheses);
	            }

	            '(' => {
	                iterator.next();
	                tokens.push(Token::LeftParentheses);
	            }

	            '+' => { 
	                iterator.next();
	                tokens.push(Token::Plus);
	            }
	            '{' => {
	            	iterator.next();
	            	tokens.push(Token::LeftMustache);
	            }
	            '}' => {
	            	iterator.next();
	            	tokens.push(Token::RightMustache);
	            }
	            '*' => { 
	                iterator.next();
	                tokens.push(Token::Multiply);
	            }
	            _ => {
	                /* Sink others */
	                iterator.next();
	            }
	        }
	    }
	    col_tokens.push(tokens);
	}
   	col_tokens
}

fn main() {
	let start = PreciseTime::now();
	let xml = r#"
        <strings>
            2 * 3 + 1 = 2 + 2 + 2 + 1 
            <algebra>
                 2 * 3 + 1 = (1 + 1) * 2 + 2 + 1 = 7
                 <sets>
                       {1, 2} + ({1, 2, 3} * {2, 3}) = ({1, 2} + {1, 2, 3}) * {2, 3}
                 </sets>
                 1 + 2 * 2 + 1 = 2 + 2 + 2 * 1;         
            </algebra>
            <boolean>
                 (1 + 0) * 1 + 1 = 0 * 1 + 1
            </boolean>
            1 * (2 + 1) + 1 = 1 + 1 + 1 
        </strings>
    "#;

    let mut domain_stack: Vec<Domain> = Vec::new();
   
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
	let mut txt:Vec<String> = Vec::new();
	let mut buf = Vec::new();
	// The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
	loop {
	    match reader.read_event(&mut buf) {
	    // for triggering namespaced events, use this instead:
	    // match reader.read_namespaced_event(&mut buf) {
	        Ok(Event::Start(ref e)) => {
	        // for namespaced:
	        // Ok((ref namespace_value, Event::Start(ref e)))
	            match e.name() {
	                b"strings" => { domain_stack.push(Domain::Strings); }
	                b"algebra" => { domain_stack.push(Domain::Algebra); }
	                b"sets" => { domain_stack.push(Domain::Sets); }
	                b"boolean" => { domain_stack.push(Domain::Boolean); }
	                _ => (),
	            }
	        },
	        // unescape and decode the text event using the reader encoding
	        Ok(Event::Text(e)) => { 
	        	let mut equal = true;
	        	let bag_of_tokens = tokenize(e.unescape_and_decode(&reader).unwrap());

	        	let domain = *domain_stack.last().unwrap();

	        	if domain == Domain::Strings {

	        		let mut numbers: Vec<i64> = Vec::new();
                	let mut operators: Vec<Token> = Vec::new();

                	let text = e.unescape_and_decode(&reader).unwrap();
                	let mut results: Vec<String> = Vec::new();

                	for s_text in text.split("=") {
                		tokenize_string(
		        			s_text.to_string(), 
		        			&mut numbers, 
		        			&mut operators
		        		);
		        		let result = recognize_string_expression(&mut numbers, &mut operators);
		        		results.push(result);
                	}

	        		let mut resolved = String::from("");
                	for (index, result) in results.clone().iter().enumerate() { 
		        		equal = *result == results[0];
		        		resolved.push_str(&format!("{:?}", result));
		        		if index < results.len() - 1 {
		        			resolved.push_str(" == ");
		        		}
		        	}

	        		println!("
	Domain: {},
	Raw Statement: {},
	Resolved To: {},
	Valid: {}
		        	", 
			        	domain,
			        	e.unescape_and_decode(&reader).unwrap(), 
			        	resolved,
			        	equal
		        	);

	        	} else if domain == Domain::Sets {
	        		let mut results_to_compare: Vec<HashSet<i64>> = Vec::new();

	        		for tokens in bag_of_tokens {
	        			let set: HashSet<i64> = resolve(
	        				domain, 
	        				shunting_yard(domain, tokens)
	        			).1;
	        			results_to_compare.push(set);
	        		}


	        		let mut resolved = String::from("");
		        	for (index, result) in results_to_compare.clone().iter().enumerate() { 
		        		equal = *result == results_to_compare[0];
		        		resolved.push_str(&format!("{:?}", result));
		        		if index < results_to_compare.len() - 1 {
		        			resolved.push_str(" == ");
		        		}
		        	}

		        	println!("
	Domain: {},
	Raw Statement: {},
	Resolved To: {},
	Valid: {}
		        	", 
			        	domain,
			        	e.unescape_and_decode(&reader).unwrap(), 
			        	resolved,
			        	equal
		        	);

	        	} else {
		        	let mut results_to_compare: Vec<i64> = Vec::new();

		        	for tokens in bag_of_tokens {
		        		let result: i64 = resolve(domain, shunting_yard(domain, tokens)).0;
		        		results_to_compare.push(result);
		        	}

		        	let mut resolved = String::from("");
		        	for (index, result) in results_to_compare.clone().iter().enumerate() { 
		        		equal = *result == results_to_compare[0];
		        		resolved.push_str(&format!("{}", result));
		        		if index < results_to_compare.len() - 1 {
		        			resolved.push_str(" == ");
		        		}
		        	}

		        	println!("
	Domain: {},
	Raw Statement: {},
	Resolved To: {},
	Valid: {}
		        	", 
			        	domain,
			        	e.unescape_and_decode(&reader).unwrap(), 
			        	resolved,
			        	equal
		        	);
	        	}
	        },
	        Ok(Event::End(e)) => {

	        	match e.name() {
	        		b"strings" => { domain_stack.pop(); }
	                b"algebra" => { domain_stack.pop(); }
	                b"sets" => { domain_stack.pop(); }
	                b"boolean" => { domain_stack.pop(); }
	                _ => (),
	        	}
	        }
	        Ok(Event::Eof) => break, // exits the loop when reaching end of file
	        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
	        _ => (), // There are several other `Event`s we do not consider here
	    }
	    // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
	    buf.clear();
	}
	let end = PreciseTime::now();
	println!("Finished In {} Seconds.", start.to(end));
}
