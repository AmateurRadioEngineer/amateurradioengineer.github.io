pub trait ParityChecker {
    fn check(byte: u8, p: u8) -> bool;
}

pub trait ParityCalculator {
    fn calculate(byte: u8) -> u8;
}

pub enum Parity {
    Even,
    Odd,
}

pub struct EvenParityChecker;
pub struct OddParityChecker;

pub struct EvenParityCalculator;
pub struct OddParityCalculator;

impl ParityChecker for EvenParityChecker {
    fn check(byte: u8, p: u8) -> bool {
        check_parity(Parity::Even, byte, p)
    }
}

impl ParityCalculator for EvenParityCalculator {
    fn calculate(byte: u8) -> u8 {
        calculate_parity(Parity::Even, byte)
    }
}

impl ParityChecker for OddParityChecker {
    fn check(byte: u8, p: u8) -> bool {
        check_parity(Parity::Odd, byte, p)
    }
}

impl ParityCalculator for OddParityCalculator {
    fn calculate(byte: u8) -> u8 {
        calculate_parity(Parity::Odd, byte)
    }
}

fn check_parity(parity: Parity, byte: u8, p: u8) -> bool {
    let count = byte.count_ones() as u8 + p & 1;
    match parity {
        Parity::Even => count % 2 == 0,
        Parity::Odd => count % 2 == 1,
    }
}

fn calculate_parity(parity: Parity, byte: u8) -> u8 {
    let count = byte.count_ones() as u8;
    match parity {
        Parity::Even => count % 2,
        Parity::Odd => (count + 1) % 2,
    }
}
