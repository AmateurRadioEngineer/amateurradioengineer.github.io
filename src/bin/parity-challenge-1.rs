use dsp_challenges::parity::{EvenParityCalculator, OddParityCalculator, ParityCalculator};
use rand::{Rng, RngCore};
use serde::Serialize;

#[derive(Serialize)]
struct Output {
    data: String,
    parity_bit: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const CORRUPTION_RATE: f64 = 0.4;
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);

    let mut csv_writer =
        csv::Writer::from_path("challenges/error-detection/parity-challenge-1.csv")?;

    for byte in bytes {
        csv_writer.serialize(Output {
            data: format!("{:08b}", byte),
            parity_bit: format!(
                "{:01b}",
                if rng.gen_bool(1.0 - CORRUPTION_RATE) {
                    EvenParityCalculator::calculate(byte)
                } else {
                    OddParityCalculator::calculate(byte)
                }
            ),
        })?;
    }

    Ok(())
}
