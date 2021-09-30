use structopt::StructOpt;

use super::errors::*;

#[derive(Debug)]
pub enum Calculation {
    PV,
    FV,
    N,
    I,
    EAIR,
    PMT
}

fn parse_calculation(src: &str) -> Result<Calculation, crate::input::Error> {
    match src {
        "pv" => Ok(Calculation::PV),
        "fv" => Ok(Calculation::FV),
        "n" => Ok(Calculation::N),
        "i" => Ok(Calculation::I),
        "eair" => Ok(Calculation::EAIR),
        "pmt" => Ok(Calculation::PMT),
        _ => Err(Error::ParseCalculation { c: src.to_string() })
    }
}

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(help="Calculation to perform", parse(try_from_str = parse_calculation))]
    pub calculation: Calculation,

    #[structopt(short, long, help="Present value")]
    pub pv: Option<f64>,

    #[structopt(
        short, long, help="Future value"
    )]
    pub fv: Option<f64>,

    #[structopt(short, long, help="Discount/interest rate, percent per period")]
    pub i: Option<f64>,

    #[structopt(short, long, help="Number of periods")]
    pub n: Option<f64>,

    #[structopt(
        short="r", long, help="Payment amount",
    )]
    pub pmt: Option<f64>,

    #[structopt(short, long, help="Annuity flag. Pass 'true' to indicate the calculation is for an annuity")]
    pub annuity: bool,

    #[structopt(
        short, long, help="Payment mode 'e' for end of period (regular annuity), 'b' for beginning of period (annuity due)",
        default_value = "e"
    )]
    pub mode: char
}

pub fn args() -> Opt {
    Opt::from_args()
}