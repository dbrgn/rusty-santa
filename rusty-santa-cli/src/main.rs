extern crate colored;
extern crate env_logger;
extern crate rprompt;
extern crate rusty_santa;

use std::process;

use colored::Colorize;
use rusty_santa::{Group, AssignError};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    env_logger::init().unwrap();

    println!("{}{}", "Rusty Santa v".green().bold(), VERSION.green().bold());

    let mut group = Group::new();

    println!("\nWho's in?\n(List one name per line and press enter, end the list with an empty line.)\n");

    loop {
        let name_raw = rprompt::prompt_reply_stderr("Name: ").unwrap();
        let name = name_raw.trim();
        if name.is_empty() {
            break;
        }
        group.add(name.into());
    }

    fn get_name(number: usize, group: &mut Group) -> Option<String> {
        loop {
            let name_raw = rprompt::prompt_reply_stderr(&format!("Name {}: ", number)).unwrap();
            let name = name_raw.trim();
            if name.is_empty() {
                return None;
            }
            if !group.contains_name(name) {
                println!("{} {}", "Invalid name:".red(), name.red());
                continue;
            }
            return Some(name.into());
        }
    }

    println!("\nAlright. Are there any pairs that should not give each other gifts?");
    println!("If you're done, just press enter.");

    loop {
        match get_name(1, &mut group) {
            Some(name1) => match get_name(2, &mut group) {
                Some(name2) => {
                    println!("OK, excluding the pair {} <-> {}", name1, name2);
                    println!("Someone else?");
                    group.exclude_pair(name1, name2);
                },
                None => break,
            },
            None => break,
        }
    }

    println!("\nAnd now, are there any pairs where person 1 should not give person 2 a gift?");
    println!("If you're done, just press enter.");

    loop {
        match get_name(1, &mut group) {
            Some(name1) => match get_name(2, &mut group) {
                Some(name2) => {
                    println!("OK, excluding the pair {} -> {}", name1, name2);
                    println!("Someone else?");
                    group.exclude(name1, name2);
                },
                None => break,
            },
            None => break,
        }
    }

    println!("\nGreat! Now we'll draw the names.");

    match group.assign() {
        Ok(assignments) => {
            println!("I'll show a name, first. That person should come to the computer,");
            println!("without other people seeing the screen.");
            println!("Press enter to reveal the name, press enter again to hide it.\n");

            for (from, to) in assignments {
                rprompt::prompt_reply_stderr(&format!("{}, are you ready? Press enter to see the name.", from)).unwrap();
                rprompt::prompt_reply_stderr(&format!("You'll give a gift to {}! (Press enter to hide the name)", to)).unwrap();
                println!("\x1B[1A\x1B[K******\n");
            }
        },
        Err(AssignError::GivingUp) => {
            println!("{}", "Hmm, I'm sorry. Even after 1000 attempts, I did not manage to find\nassignments where all constrainst are satisifed...".red());
            process::exit(1);
        },
        Err(e) => {
            println!("{}", format!("Error: {:?}", e).red());
            process::exit(1);
        },
    }

    println!("Happy gift-giving!");
}
