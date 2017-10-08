extern crate regex;

use regex::Regex;
use std::process::Command;

// Initializes the vector of rules.  These can be dynamically assigned by, say, a genetic alg
fn main() {
    let mut rule_set: Vec<(&str, &str)> = Vec::new();
    let axiom = String::from("X");
    rule_set.push(("X", "F-[[X]+X]+F[+FX]-X"));
    rule_set.push(("F", "FF"));
    let instructions = replace(rule_set, axiom, 0, 6);
    Command::new("python3")
            .arg("src/pturtle.py")
            .arg(instructions)
            .spawn()
            .expect("yikes"); // Hey, I don't judge your error messages
}

// Performs the substitutions laid out in the given ruleset
fn replace(rule_set: Vec<(&str, &str)>,
    axiom: String,
    generation: usize,
    gen_count: usize) -> String {
        let mut new_string = axiom;
        match generation {
            _ if generation == gen_count => return new_string,
            _ =>  {
                for &(a, b) in rule_set.iter() {
                    let re = Regex::new(a).unwrap();
                    let old_string = new_string;
                    new_string = re.replace_all(&old_string, b).to_string();
                }
                return replace(rule_set, new_string, generation+1, gen_count)
            }
        }
    }
