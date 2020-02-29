// type of π generators we support - TODO: do we need this?
enum GeneratorType {
    Chudnovsky,
    Spigot,
}

// TODO: a type that represents an arbitrary precision irrational number...
type Irrational = f64;
// TODO: this needs to be an arbitrary precision rational type (e.g. a 2-tuple?)
type Rational = (i32, i32);
type Integer = i32;
// TODO: this isn't efficient
type Digit = i32;

// This type basically gives us iterator-like semantics - only works for pi generators where it's
// relatively easy to get the kth digit of pi without first having to compute the k-1th digit
trait IteratorGenerator {
    fn next(&mut self) -> Digit;
    fn prev(&mut self) -> Digit;
}

// if you implement this trait you are one-shot generator
trait OneShotGenerator {
    fn calc(&self) -> Irrational;
}

// a linear fractional transform, represented as a 2x2 matrix
type LFT = (i32, i32, i32, i32);

// 2x2 matrix multiplication where a and b are LFTs
fn comp(a: LFT, b: LFT) -> LFT {
    (
        a.0 * b.0 + a.1 * b.2,
        a.0 * b.1 + a.1 * b.3,
        a.2 * b.0 + a.3 * b.2,
        a.2 * b.1 + a.3 * b.3,
    )
}

fn extr(a: LFT, x: Integer) -> Rational {
    (a.0 * x + a.1, a.2 * x + a.3)
}

fn floor(r: Rational) -> Integer {
    (r.0 / r.1) as Integer
}

fn next(z: LFT) -> Integer {
    floor(extr(z, 3))
}

fn safe(z: LFT, n: Integer) -> bool {
    n == floor(extr(z, 4))
}

fn prod(z: LFT, n: Integer) -> LFT {
    comp((10, -10 * n, 0, 1), z)
}

// Spigot algorithm - generate digits of π, independently
// https://en.wikipedia.org/wiki/Pi#Spigot_algorithms
// The algorithm is described at, http://www.cs.ox.ac.uk/jeremy.gibbons/publications/spigot.pdf)
struct Spigot {
    current_digit: i32,
}
impl Spigot {
    fn new() -> Spigot {
        Spigot { current_digit: 0 }
    }

    fn get_digit(&mut self, k: i32) -> Digit {
        self.current_digit = k;

        println!("get digit: {}", k);

        // start with the identity matrix
        let mut state: LFT = (1, 0, 0, 1);
        let mut digit = Default::default();

        // rely on the state converging to break out of the loop
        for i in 1.. {
            let lft: LFT = (i, 4 * i + 2, 0, 2 * i + 1);
            println!("lft: {:?}", lft);

            println!("state: {:?}", state);
            let prod = prod(comp(state, lft), k);
            println!("prod: {:?}", prod);
            if safe(prod, k) {
                digit = next(state);
                break;
            } else {
                state = prod;
            }
        }

        digit
    }
}
impl IteratorGenerator for Spigot {
    fn next(&mut self) -> Digit {
        self.current_digit += 1;
        self.get_digit(self.current_digit)
    }
    fn prev(&mut self) -> Digit {
        self.current_digit -= 1;
        self.get_digit(self.current_digit)
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
    fn calc(&self) -> Irrational {
        0f64
    }
}

fn main() {
    println!("Welcome to the Rust π generator!");

    /*
    let pi_oneshot = Chudnovsky::new();
    println!("Value of π (using Chudnovsky): {}", pi_oneshot.calc());
    */

    let mut pi_iter = Spigot::new();

    /*
    for _ in 0..10 {
        print!("{}", pi_iter.next());
    }
    println!();
    */

    let k = 2;
    println!("Digit number {} of π is {}", k, pi_iter.get_digit(k));
}
