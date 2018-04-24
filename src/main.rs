extern crate quick_xml;
extern crate time;

use quick_xml::Reader;
use quick_xml::events::Event;

/* Command Line Args Imports */
use std::env;
use std::process;

use std::collections::HashSet;

use std::fs::File;
use std::io::prelude::*;

mod token;
mod domain;

use token::Token;
use domain::Domain;

fn resolve(domain: Domain, tokens: Vec<Token>) -> (i64, HashSet<i64>) {
	let mut stack: Vec<i64> = Vec::new();
	let mut set_stack: Vec<HashSet<i64>> = Vec::new();

	let mut build_string: String = String::new();

	match domain {
		Domain::Algebra | Domain::Strings | Domain::Boolean => {
			for (index, token) in tokens.iter().enumerate() {
				match token {
					Token::Number(val) => stack.push(*val),
					Token::Plus => {
						
						match domain {
							Domain::Boolean => {
								let a = stack.pop().unwrap();
								let b = stack.pop().unwrap();
								stack.push(a | b);
							}
							Domain::Strings => {
								let a = stack.pop().unwrap();
								let b = stack.pop().unwrap();

								build_string.push_str(&a.to_string());
								build_string.push_str(&b.to_string());

								if (index + 1) < tokens.len() {
									if tokens[index + 1] == Token::Multiply { /* Handle strings that should resolve to algebra addition */
										build_string = String::from("");
										build_string.push_str(&(a + b).to_string())
									}
								}			

								stack.push(build_string.parse::<i64>().unwrap());
								build_string = String::from("");
							}
							_ => { 
								let a = stack.pop().unwrap();
								let b = stack.pop().unwrap();
								stack.push(a + b); 
							}
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
						match domain {
							Domain::Boolean => {
								let a = stack.pop().unwrap();
								let b = stack.pop().unwrap();
								stack.push(a & b);
							}
							Domain::Strings => {
								let a = stack.pop().unwrap();
								let b = stack.pop().unwrap();

								for _ in 0..a {
									build_string.push_str(&b.to_string());
								}

								stack.push(build_string.parse::<i64>().unwrap());
								build_string = String::from("");
							}
							_ => { 
								let a = stack.pop().unwrap();
								let b = stack.pop().unwrap();
								stack.push(a * b); 
							}
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
                while let Some(op) = stack.pop() {
                    match op {
                    	Token::LeftMustache => {
			    			rpn_stack.push(Token::Set(set.clone()));
			    			set.clear(); /* Clear the previous set of integers */
            				break;
            			},
                        Token::LeftParentheses => { break; },
                        _ => rpn_stack.push(op),
                    }
                }
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
	Resolved: \t{},
	Domain: \t{},
	Valid: \t\t{}
================================================", 
		    	s_text.trim(), 
		    	resolved,
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
	Resolved: \t{},
	Domain: \t{},
	Valid: \t\t{}
================================================", 
	    	text, 
	    	resolved,
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
	Resolved: \t{},
	Domain: \t{},
	Valid: \t\t{}
================================================",
		    	s_text.trim(),
		    	resolved,
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
	Resolved: \t{},
	Domain: \t{},
	Valid: \t\t{}
================================================",
	    	text,
	    	resolved,
	    	domain, 
	    	equal
		);
	}
}

fn main() {
	let argv: Vec<String> = env::args().collect();

	if argv.len() != 2 {
        println!("\n\nError, exiting...\nUsage: {:?} <input>.xml", argv[0]);
        process::exit(1);
    }

	let file = File::open(&argv[1]);
    let mut xml = String::new();
    file.unwrap().read_to_string(&mut xml).unwrap();

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
}
