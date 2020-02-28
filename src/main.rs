// type of π generators we support - TODO: do we need this?
enum GeneratorType {
    Chudnovsky,
    Spigot,
}

// TODO: a type that represents an arbitrary precision irrational number...
type Number = u128;
// TODO: this isn't efficient
type Digit = u8;

// This type basically gives us iterator-like semantics - only works for pi generators where it's
// relatively easy to get the kth digit of pi without first having to compute the k-1th digit
trait IteratorGenerator {
    fn next(&self) -> Digit;
    fn prev(&self) -> Digit;
}

// if you implement this trait you are one-shot generator
trait OneShotGenerator {
    fn calc(&self) -> Number;
}

// Spigot algorithm - generate digits of π, independently
// https://en.wikipedia.org/wiki/Pi#Spigot_algorithms
struct Spigot {}
impl Spigot {
    fn new() -> Spigot {
        Spigot {}
    }

    fn get_digit(&self, k: u128) -> Digit {
        1
    }
}
impl IteratorGenerator for Spigot {
    fn next(&self) -> Digit {
        1
    }
    fn prev(&self) -> Digit {
        1
    }
}

// Chudovsky algorithm - iterative
// https://en.wikipedia.org/wiki/Chudnovsky_algorithm

// TODO: could it just represent the current computation state/step in Chudnovsky's algorithm?
// https://wikimedia.org/api/rest_v1/media/math/render/svg/ac4896b5f640ed03446ae8e49d5424fc20faa3b9
#[derive(Default)]
struct State {}
impl State {
    fn new() -> State {
        State {}
    }
}
struct Chudnovsky {
    state: State,
}
impl Chudnovsky {
    fn new() -> Chudnovsky {
        Chudnovsky {
            state: Default::default(),
        }
    }
}
impl OneShotGenerator for Chudnovsky {
    fn calc(&self) -> Number {
        0
    }
}

fn main() {
    println!("Welcome to the Rust π generator!");

    let pi_oneshot = Chudnovsky::new();
    println!("Value of π (using Chudnovsky): {}", pi_oneshot.calc());

    let pi_iter = Spigot::new();
    for _ in 0..10 {
        print!("{}", pi_iter.next());
    }
    println!();

    println!("1000-th digit of π: {}", pi_iter.get_digit(1000));
}
