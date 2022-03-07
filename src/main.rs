mod utils;
mod tests;
mod eval;
mod move_gen;
mod evaluation_function;
mod hashing;
mod deprecated_move_gen;



// TODO write a check to see if a seed does not generate two equal numbers
// TODO write a more sophisticated eval function
// TODO store possible moves in advance to use in a more sophisticated eval function
fn main() {
    tests::main();
}

