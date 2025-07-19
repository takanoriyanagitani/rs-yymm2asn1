use std::error::Error;
use std::io::{self, Write};
use std::process::ExitCode;

use rs_yymm2asn1::DateYyMmCompactInfer;

fn ym2infer(yy: u8, mm: u8) -> Result<DateYyMmCompactInfer, io::Error> {
    match mm {
        1 => Ok(DateYyMmCompactInfer::Jan(yy)),
        2 => Ok(DateYyMmCompactInfer::Feb(yy)),
        3 => Ok(DateYyMmCompactInfer::Mar(yy)),
        4 => Ok(DateYyMmCompactInfer::Apr(yy)),
        5 => Ok(DateYyMmCompactInfer::May(yy)),
        6 => Ok(DateYyMmCompactInfer::Jun(yy)),
        7 => Ok(DateYyMmCompactInfer::Jul(yy)),
        8 => Ok(DateYyMmCompactInfer::Aug(yy)),
        9 => Ok(DateYyMmCompactInfer::Sep(yy)),
        10 => Ok(DateYyMmCompactInfer::Oct(yy)),
        11 => Ok(DateYyMmCompactInfer::Nov(yy)),
        12 => Ok(DateYyMmCompactInfer::Dec(yy)),
        _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid month")),
    }
}

fn der2writer<W: Write>(der_bytes: &[u8], mut wtr: W) -> Result<(), io::Error> {
    wtr.write_all(der_bytes)
}

fn der2stdout(der_bytes: &[u8]) -> Result<(), io::Error> {
    der2writer(der_bytes, io::stdout())
}

fn arg2infer() -> Result<DateYyMmCompactInfer, Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err(format!("Usage: {} <yy> <mm>", args[0]).into());
    }

    let yy: u8 = args[1].parse()?;
    let mm: u8 = args[2].parse()?;

    Ok(ym2infer(yy, mm)?)
}

fn sub() -> Result<(), Box<dyn Error>> {
    let infer: DateYyMmCompactInfer = arg2infer()?;
    let der_bytes: Vec<u8> = infer.to_der_bytes()?;

    der2stdout(&der_bytes)?;

    Ok(())
}

fn main() -> ExitCode {
    if let Err(e) = sub() {
        eprintln!("Error: {}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
