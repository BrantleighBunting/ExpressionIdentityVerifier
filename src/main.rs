extern crate quick_xml;
extern crate time;

use quick_xml::Reader;
use quick_xml::events::Event;

use std::collections::HashSet;

use time::PreciseTime;

use std::fs::File;
use std::io::prelude::*;

mod token;
mod domain;

use token::Token;
use domain::Domain;

fn resolve(domain: Domain, tokens: Vec<Token>) -> (i64, HashSet<i64>) {
	let mut stack: Vec<i64> = Vec::new();
	let mut set_stack: Vec<HashSet<i64>> = Vec::new();

	match domain {
		Domain::Algebra | Domain::Strings | Domain::Boolean => {
			for token in tokens {
				match token {
					Token::Number(val) => stack.push(val),
					Token::Plus => {
						let a = stack.pop().unwrap();
						let b = stack.pop().unwrap();
						match domain {
							Domain::Boolean => {
								stack.push(a | b);
							}
							_ => { stack.push(a + b); }
						}
					}
					Token::Minus => {
						let a = stack.pop().unwrap();
						let b = stack.pop().unwrap();
						stack.push(b - a);
					}
					Token::Power => {
						let a = stack.pop().unwrap();
						let mut b = stack.pop().unwrap();
						let repeat = b;
						for _ in 1..a { b *= repeat; }
						stack.push(b);
					}
					Token::Multiply => {
						let a = stack.pop().unwrap();
						let b = stack.pop().unwrap();
						match domain {
							Domain::Boolean => {
								stack.push(a & b);
							}
							_ => { stack.push(a * b); }
						}
						
					}
					_ => {}
				}
			}
			return (stack.pop().unwrap(), HashSet::new());
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
			return (0, set_stack.pop().unwrap());
		}
	}	
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
            Token::Plus | Token::Multiply | Token::Minus | Token::Power => {
                while let Some(o) = stack.pop() {
                    if token.clone().operator_precedence() <= o.clone().operator_precedence() {
                        rpn_stack.push(o);
                    } else {
                        stack.push(o);
                        break;
                    }
                }
                stack.push(token)
            },
            Token::LeftParentheses | Token::LeftMustache => stack.push(token),
            Token::RightParentheses | Token::RightMustache => {
                let mut closure = false;
                while let Some(op) = stack.pop() {
                    match op {
                    	Token::LeftMustache => {
            				closure = true;
			    			rpn_stack.push(Token::Set(set.clone()));
			    			set.clear(); /* Clear the previous set of integers */
            				break;
            			},
                        Token::LeftParentheses => {
                            closure = true;
                            break;
                        },
                        _ => rpn_stack.push(op),
                    }
                }
                assert!(closure)
            },
        }
    }
    while let Some(op) = stack.pop() {
        rpn_stack.push(op);
    }
    rpn_stack
}

fn tokenize(text: String) -> Vec<Vec<Token>> {
	let mut col_tokens: Vec<Vec<Token>> = Vec::new();
	for s_text in text.split("=") {
		let mut iterator = s_text.trim().chars().peekable();
		let mut tokens: Vec<Token> = Vec::new();

	    while let Some(&c) = iterator.peek() {
	    	iterator.next();
	        match c {
	            '0' ... '9' => { tokens.push(Token::Number(c.to_string().parse::<i64>().unwrap())); }
	            ')' => { tokens.push(Token::RightParentheses); }
	            '(' => { tokens.push(Token::LeftParentheses); }
	            '+' => { tokens.push(Token::Plus); }
	            '-' => { tokens.push(Token::Minus); }
	            '{' => { tokens.push(Token::LeftMustache); }
	            '}' => { tokens.push(Token::RightMustache); }
	            '*' => { tokens.push(Token::Multiply); }
	            '^' => { tokens.push(Token::Power); }
	            _ => {}
	        }
	    }
	    col_tokens.push(tokens);
	}
   	col_tokens
}

fn handle_sets_text(text: String, domain: Domain) {
	let mut equal: bool = true;

	let mut results_to_compare: Vec<HashSet<i64>> = Vec::new();

	if text.contains(";") {
		for s_text in text.split(";") {
			let bag_of_tokens = tokenize(s_text.trim().clone().to_string());

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

			println!("\tStatement: \t{},
	Domain: \t{},
	Valid: \t\t{}
================================================", 
		    	s_text.trim(), 
		    	domain,
		    	equal
			);
		}
	} else {
		let bag_of_tokens = tokenize(text.clone());

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

		println!("\tStatement: \t{},
	Domain: \t{},
	Valid: \t\t{}
================================================", 
	    	text, 
	    	domain,
	    	equal
		);
	}
}

fn handle_other_domains(text: String, domain: Domain) {
	if text.contains(";") {
		let coll = text.split(";");
		for s_text in coll {
			if s_text == "" { break; }

			let mut equal: bool = true;

			let mut results_to_compare: Vec<i64> = Vec::new();

			let bag_of_tokens = tokenize(s_text.trim().clone().to_string());

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

			println!("\tStatement: \t{},
	Domain: \t{},
	Valid: \t\t{}
================================================",
		    	s_text.trim(),
		    	domain, 
		    	equal
			);
		}
	} else {
		let mut equal: bool = true;

		let mut results_to_compare: Vec<i64> = Vec::new();

		let bag_of_tokens = tokenize(text.clone());

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

		println!("\tStatement: \t{},
	Domain: \t{},
	Valid: \t\t{}
================================================",
	    	text,
	    	domain, 
	    	equal
		);
	}
}

fn main() {
	let start = PreciseTime::now();

	let mut file = File::open("input.xml");
    let mut xml = String::new();
    file.unwrap().read_to_string(&mut xml);


	// let xml = r#"
 //        <strings>
 //            2 * 3 + 1 = 2 + 2 + 2 + 1 
 //            <algebra>
 //                 2 ^ 3 - 1 = (1 + 1) * 2 + 2 + 1 = 7
 //                 <sets>
 //                       {1, 2} + ({1, 2, 3} * {2, 3}) = ({1, 2} + {1, 2, 3}) * {2, 3};
 //                       {1, 2} + ({1, 2} * {2, 3}) = ({1, 2} + {1, 2, 3}) * {2, 3}
 //                 </sets>
 //                 1 + 2 * 2 + 1 = 2 + 2 + 2 * 1;
 //                 1 + 2 * 2 + 1 = 2 + 2 + 2 + 5 * 1;
 //            </algebra>
 //            <boolean>
 //                 (1 + 0) * 1 + 1 = 0 * 1 + 1
 //            </boolean>
 //            1 * (2 + 1) + 1 = 1 + 1 + 1 
 //        </strings>
 //    "#;

    let mut domain_stack: Vec<Domain> = Vec::new();
   
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);

	let mut buf = Vec::new();
	println!("================================================");
	loop {
	    match reader.read_event(&mut buf) {
	        Ok(Event::Start(ref e)) => {
	            match e.name() {
	                b"strings" => { domain_stack.push(Domain::Strings); }
	                b"algebra" => { domain_stack.push(Domain::Algebra); }
	                b"sets" => { domain_stack.push(Domain::Sets); }
	                b"boolean" => { domain_stack.push(Domain::Boolean); }
	                _ => (),
	            }
	        },
	        Ok(Event::Text(e)) => { 
	        	let domain = *domain_stack.last().unwrap();

	        	if domain == Domain::Sets {
	        		handle_sets_text(e.unescape_and_decode(&reader).unwrap(), domain);

	        	} else {
		        	handle_other_domains(e.unescape_and_decode(&reader).unwrap(), domain);
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
	        Ok(Event::Eof) => break,
	        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
	        _ => (),
	    }
	    buf.clear();
	}

	let end = PreciseTime::now();
	println!("Finished In {} Seconds.", start.to(end));
}
