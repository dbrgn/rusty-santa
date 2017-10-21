extern crate rand;

use std::collections::{HashMap, HashSet};

use rand::{thread_rng, Rng};


#[derive(Debug, Clone, PartialEq, Eq)]
enum Constraint {
    ExcludePair {
        a: String,
        b: String,
    },
    Exclude {
        from: String,
        to: String,
    },
}

#[derive(Debug, Clone)]
struct Group {
    people_set: HashSet<String>,
    constraints: Vec<Constraint>,
}

impl Group {
    pub fn new() -> Self {
        Group {
            people_set: HashSet::new(),
            constraints: vec![],
        }
    }

    pub fn add(&mut self, name: String) {
        self.people_set.insert(name);
    }

    fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    pub fn exclude(&mut self, from: String, to: String) {
        let constraint = Constraint::Exclude { from: from, to: to };
        self.add_constraint(constraint);
    }

    pub fn exclude_pair(&mut self, a: String, b: String) {
        let constraint = Constraint::ExcludePair { a: a, b: b };
        self.add_constraint(constraint);
    }

    pub fn assign(&self) -> Result<Vec<(String, String)>, AssignError> {
        // Shuffle the people
        let mut people: Vec<String> = self.people_set.iter().cloned().collect();
        let mut rng = thread_rng();
        rng.shuffle(&mut people);

        // Initialize the gift possibility matrix
        let count = people.len();
        let mut matrix = HashMap::with_capacity(count);
        for person in people.iter() {
            let mut receivers = self.people_set.clone();
            receivers.remove(person);
            matrix.insert(person.clone(), receivers);
        }

        // Iterate over constraints, apply them to the matrix
        for constraint in self.constraints.iter() {
            match constraint {
                &Constraint::ExcludePair{ ref a, ref b } => {
                    if !matrix.contains_key(a) {
                        return Err(AssignError::BadConstraint(format!("Unknown person \"{}\"", a)));
                    }
                    if !matrix.contains_key(b) {
                        return Err(AssignError::BadConstraint(format!("Unknown person \"{}\"", b)));
                    }
                    matrix.get_mut(a).unwrap().remove(b);
                    matrix.get_mut(b).unwrap().remove(a);
                },
                &Constraint::Exclude { ref from, ref to } => {
                    if !matrix.contains_key(from) {
                        return Err(AssignError::BadConstraint(format!("Unknown person \"{}\"", from)));
                    }
                    if !matrix.contains_key(to) {
                        return Err(AssignError::BadConstraint(format!("Unknown person \"{}\"", to)));
                    }
                    matrix.get_mut(from).unwrap().remove(to);
                }
            }
        };
        
        for person in people {
            println!("{} -> {:?}", person, matrix.get(&person));
        }

        Ok(vec![])
    }
}

#[derive(Debug)]
pub enum AssignError {
    BadConstraint(String),
}

fn main() {
    println!("Hello, world!");

    let mut group = Group::new();

    group.add("Sheldon".into());
    group.add("Amy".into());
    group.add("Leonard".into());
    group.add("Penny".into());
    group.add("Rajesh".into());

    group.exclude_pair("Sheldon".into(), "Amy".into());
    group.exclude_pair("Leonard".into(), "Penny".into());

    match group.assign() {
        Ok(assignments) => println!("{:?}", assignments),
        Err(e) => println!("Error: {:?}", e),
    }
}
