# Rusty Santa

![Logo](logo.png)

A small Rust library (and command-line tool) for resolving [Secret
Santa](https://en.wikipedia.org/wiki/Secret_Santa) assignments with additional
constraints.

It is possible to add the following constraints to the random assignments:

- Person A and B should not draw each other (e.g. for couples)
- Person A should not draw person B (e.g. if person B already received a gift
  from person A the previous year, or if person A dislikes person B)

While this is an interesting mathematical problem and can be solved with
bipartite graphs and the hungarian algorithm, this library sticks to a simpler
approach and tries to emulate the real name drawing from a basket. Up to 1000
attempts are made at resolving the name assignments without conflict until the
algorithm fails.


## Library

### Usage

```rust
let mut group = Group::new();

group.add("Sheldon".into());
group.add("Amy".into());
group.add("Leonard".into());
group.add("Penny".into());
group.add("Rajesh".into());
group.add("Howard".into());
group.add("Bernadette".into());

// Exclude couples
group.exclude_pair("Sheldon".into(), "Amy".into());
group.exclude_pair("Leonard".into(), "Penny".into());
group.exclude_pair("Howard".into(), "Bernadette".into());

// Sheldon can't keep secrets from his roommates
group.exclude("Sheldon".into(), "Leonard".into());

match group.assign() {
    Ok(assignments) => {
        for (from, to) in assignments {
            println!("{} => {}", from, to);
        }
    },
    Err(e) => println!("Error: {:?}", e),
}
```


### Logging

Rusty Santa logs the algorithm steps on the TRACE level:

```
$ RUST_LOG=rusty_santa=trace cargo run -p rusty-santa --example cli
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/examples/cli`
TRACE:rusty_santa: Drawing recipient for Howard
TRACE:rusty_santa: Options: ["Sheldon", "Penny", "Amy", "Rajesh", "Leonard"]
TRACE:rusty_santa: Picked Sheldon!
TRACE:rusty_santa: Drawing recipient for Sheldon
TRACE:rusty_santa: Options: ["Howard", "Bernadette", "Penny", "Rajesh"]
TRACE:rusty_santa: Picked Howard!
TRACE:rusty_santa: Drawing recipient for Bernadette
TRACE:rusty_santa: Options: ["Penny", "Amy", "Rajesh", "Leonard"]
TRACE:rusty_santa: Picked Penny!
TRACE:rusty_santa: Drawing recipient for Penny
TRACE:rusty_santa: Options: ["Bernadette", "Amy", "Rajesh"]
TRACE:rusty_santa: Picked Amy!
TRACE:rusty_santa: Drawing recipient for Amy
TRACE:rusty_santa: Options: ["Bernadette", "Rajesh", "Leonard"]
TRACE:rusty_santa: Picked Rajesh!
TRACE:rusty_santa: Drawing recipient for Rajesh
TRACE:rusty_santa: Options: ["Bernadette", "Leonard"]
TRACE:rusty_santa: Picked Leonard!
TRACE:rusty_santa: Drawing recipient for Leonard
TRACE:rusty_santa: Options: ["Bernadette"]
TRACE:rusty_santa: Picked Bernadette!
Howard => Sheldon
Sheldon => Howard
Bernadette => Penny
Penny => Amy
Amy => Rajesh
Rajesh => Leonard
Leonard => Bernadette
```

## Command Line Tool

There is a proof-of-concept command-line interface:

    $ cargo run -p rusty-santa-cli
    Rusty Santa v0.1.0

    Who's in?
    (List one name per line and press enter, end the list with an empty line.)

    Name: A
    Name: B
    Name: C
    Name: D
    Name: 

    Alright. Are there any pairs that should not give each other gifts?
    If you're done, just press enter.
    Name 1: A
    Name 2: B
    OK, excluding the pair A <-> B
    Someone else?
    Name 1: 

    And now, are there any pairs where person 1 should not give person 2 a gift?
    If you're done, just press enter.
    Name 1: A
    Name 2: C
    OK, excluding the pair A -> C
    Someone else?
    Name 1: 

    Great! Now we'll draw the names.
    I'll show a name, first. That person should come to the computer,
    without other people seeing the screen.
    Press enter to reveal the name, press enter again to hide it.

    B, are you ready? Press enter to see the name.
    ******

    A, are you ready? Press enter to see the name.
    You'll give a gift to D! (Press enter to hide the name)

...and so on.


## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.


### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
