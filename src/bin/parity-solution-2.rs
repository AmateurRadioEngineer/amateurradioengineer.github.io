use dsp_challenges::parity::{OddParityCalculator, ParityCalculator};
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;

#[derive(Deserialize)]
struct Input {
    data: String,
}

#[derive(Serialize)]
struct Output {
    data: String,
    parity_bit: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut csv_reader =
        csv::Reader::from_path("challenges/error-detection/parity-challenge-2.csv")?;

    let mut csv_writer =
        csv::Writer::from_path("challenges/error-detection/parity-solution-2.csv")?;

    for record in csv_reader.deserialize::<Input>() {
        let input = record?;
        let data = u8::from_str_radix(&input.data, 2)?;

        csv_writer.serialize(Output {
            data: format!("{:08b}", data),
            parity_bit: format!("{:01b}", OddParityCalculator::calculate(data)),
        })?;
    }

    Ok(())
}
