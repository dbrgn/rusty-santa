extern crate env_logger;
extern crate rusty_santa;

use rusty_santa::Group;

fn main() {
    env_logger::init().unwrap();

    let mut group = Group::new();

    group.add("Sheldon".into());
    group.add("Amy".into());
    group.add("Leonard".into());
    group.add("Penny".into());
    group.add("Rajesh".into());

    group.exclude_pair("Sheldon".into(), "Amy".into());
    group.exclude_pair("Sheldon".into(), "Leonard".into());
    group.exclude_pair("Leonard".into(), "Penny".into());

    match group.assign() {
        Ok(assignments) => {
            for (from, to) in assignments {
                println!("{} => {}", from, to);
            }
        },
        Err(e) => println!("Error: {:?}", e),
    }
}
