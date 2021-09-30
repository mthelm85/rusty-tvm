mod annuities;
mod errors;
mod input;
mod lump_sum;
mod misc;
mod traits;

use traits::{ FutureValue, InterestRate, NumberOfPeriods, Payment, PresentValue };

fn main() {
    let input = input::args();
    let result = match input.calculation {
        input::Calculation::PV => match input.pmt {
            Some(_pmt) => {
                match input.mode {
                    'e' => annuities::build_annuity(&annuities::AnnuityType::RegularAnnuity, &input).pv(),
                    'b' => annuities::build_annuity(&annuities::AnnuityType::AnnuityDue, &input).pv(),
                    _ => { 
                        println!("Mode was not 'e' or 'b', defaulting to 'e'...");
                        annuities::build_annuity(&annuities::AnnuityType::RegularAnnuity, &input).pv()
                     }
                }
            },
            None => { lump_sum::LumpSum { fv: input.fv, i: input.i, n: input.n, pv: None }.pv() }
        },
        input::Calculation::FV => match input.pmt {
            Some(_pmt) => {
                match input.mode {
                    'e' => annuities::build_annuity(&annuities::AnnuityType::RegularAnnuity, &input).fv(),
                    'b' => annuities::build_annuity(&annuities::AnnuityType::AnnuityDue, &input).fv(),
                    _ => { 
                        println!("Mode was not 'e' or 'b', defaulting to 'e'...");
                        annuities::build_annuity(&annuities::AnnuityType::RegularAnnuity, &input).fv()
                    }
                }
            },
            None => { lump_sum::LumpSum { pv: input.pv, i: input.i, n: input.n, fv: None }.fv() }
        },
        input::Calculation::N => match input.pmt {
            Some(_pmt) => {
                match input.mode {
                    'e' => annuities::build_annuity(&annuities::AnnuityType::RegularAnnuity, &input).n(),
                    'b' => annuities::build_annuity(&annuities::AnnuityType::AnnuityDue, &input).n(),
                    _ => { 
                        println!("Mode was not 'e' or 'b', defaulting to 'e'...");
                        annuities::build_annuity(&annuities::AnnuityType::RegularAnnuity, &input).n()
                    }
                }
            },
            None => { lump_sum::LumpSum { pv: input.pv, i: input.i, fv: input.fv, n: None, }.n() }
        },
        input::Calculation::I => lump_sum::LumpSum { pv: input.pv, n: input.n, fv: input.fv, i: None, }.i(),
        input::Calculation::EAIR => Ok(misc::eair(&input.i.unwrap(), &input.n.unwrap())),
        input::Calculation::PMT => match input.mode {
            'e' => annuities::build_annuity(&annuities::AnnuityType::RegularAnnuity, &input).pmt(),
            'b' => annuities::build_annuity(&annuities::AnnuityType::AnnuityDue, &input).pmt(),
            _ => { 
                println!("Mode was not 'e' or 'b', defaulting to 'e'...");
                annuities::build_annuity(&annuities::AnnuityType::RegularAnnuity, &input).pmt()
            }
        }
    };

    match result {
        Ok(val) => println!("{}", val),
        Err(e) => eprintln!("{}", e)
    }
}
