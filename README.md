# Rusty Santa

![Logo](logo.png)

A small Rust library for resolving [Secret Santa](https://en.wikipedia.org/wiki/Secret_Santa)
assignments with additional constraints.

It is possible to add the following constraints to the random assignments:

- Person A and B should not draw each other (e.g. for couples)
- Person A should not draw person B (e.g. if person B already received a gift
  from person A the previous year, or if person A dislikes person B)

While this is an interesting mathematical problem and can be solved with
bipartite graphs and the hungarian algorithm, this library sticks to a simpler
approach and tries to emulate the real name drawing from a basket. Up to 1000
attempts are made at resolving the name assignments without conflict until the
algorithm fails.


## Usage

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


## Logging

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
