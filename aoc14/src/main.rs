/*
Note about the entropy:

It seems peculiar that we find the christmas tree by considering
the *maximum* entropy. Surely, a picture of a christmas tree should
have a *lower* entropy than some random configuration of robots?

In theory, this is true. However, in our string encoding of the
grid, we discard the information about the number of robots at
each grid position. That is, we only specify whether a point is
occupied (#) or empty (.). As it turns out, when the robots go
into the christmas tree formation, multiple robots tend to be
less likely to simultaneously occupy the same position than
in the random case.

This means that in the christmas tree configuration, we have more
occupied positions (i.e. more occurences of "#" in the string). As
a consequence, there are simply more ways to arrange the string when
there are more "#" (higher multiplicity), which means a higher entropy.
*/

use std::fs;

fn main() {
    let input = fs::read_to_string("data.txt").unwrap();
    let safety_score1 = aoc14::problem1(&input);
    let (iteration, entropy) = aoc14::problem2(&input);
    println!("Safety score (problem 1): {}", safety_score1);
    println!(
        "Iteration {} gave maximum entropy {} (problem 2)",
        iteration, entropy
    );
}
