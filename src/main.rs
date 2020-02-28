struct Generator {
    digit_limit: u64,
}

impl Generator {
    fn new(limit: u64) -> Generator {
        Generator { digit_limit: limit }
    }

    // generate the next digit in pi
    fn next(&mut self) -> Option<u8> {
        if self.digit_limit > 0 {
            self.digit_limit -= 1;
            Some(1)
        } else {
            None
        }
    }
}

fn main() {
    println!("Welcome to the Rust Pi generator!");

    // arbitrary limit to the number of digits to generate for
    let mut pi: Generator = Generator::new(10);

    while let Some(digit) = pi.next() {
        println!("{}", digit)
    }
}
