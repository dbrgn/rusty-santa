# Rusty Santa

A small library for resolving [secret santa](https://en.wikipedia.org/wiki/Secret_Santa)
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
