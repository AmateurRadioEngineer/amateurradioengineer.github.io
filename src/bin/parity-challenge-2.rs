use rand::RngCore;
use serde::Serialize;

#[derive(Serialize)]
struct Output {
    data: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);

    let mut csv_writer =
        csv::Writer::from_path("challenges/error-detection/parity-challenge-2.csv")?;

    for byte in bytes {
        csv_writer.serialize(Output {
            data: format!("{:08b}", byte),
        })?;
    }

    Ok(())
}
