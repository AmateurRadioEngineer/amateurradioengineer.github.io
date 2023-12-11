use dsp_challenges::parity::{EvenParityChecker, ParityChecker};
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;

#[derive(Deserialize)]
struct Input {
    data: String,
    parity_bit: String,
}

#[derive(Serialize)]
enum Validity {
    Valid,
    Corrupt,
}

#[derive(Serialize)]
struct Output {
    data: String,
    parity_bit: String,
    validity: Validity,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut csv_reader = csv::Reader::from_path("docs/error-detection/parity-challenge-1.csv")?;

    let mut csv_writer = csv::Writer::from_path("docs/error-detection/parity-solution-1.csv")?;

    for record in csv_reader.deserialize::<Input>() {
        let input = record?;
        let data = u8::from_str_radix(&input.data, 2)?;
        let p = u8::from_str_radix(&input.parity_bit, 2)?;

        csv_writer.serialize(Output {
            data: format!("{:08b}", data),
            parity_bit: format!("{:01b}", p),
            validity: match EvenParityChecker::check(data, p) {
                true => Validity::Valid,
                false => Validity::Corrupt,
            },
        })?;
    }

    Ok(())
}
