use std::f64;

use super::input::Opt;
use super::errors::*;
use super::traits::*;

#[derive(Debug)]
pub enum AnnuityType { AnnuityDue, RegularAnnuity }

#[derive(Debug)]
pub struct Annuity {
    pub kind: AnnuityType,
    pub i: Option<f64>,
    pub fv: Option<f64>,
    pub pv: Option<f64>,
    pub n: Option<f64>,
    pub pmt: Option<f64>
}

impl PresentValue for Annuity {
    fn pv(&self) -> Result<f64, Error> {
        match &self.kind {
            AnnuityType::AnnuityDue => {
                if let (Some(pmt), Some(i), Some(n)) = (self.pmt, self.i, self.n) { 
                    let i = i / 100.0;
                    Ok(pmt * ((1.0 - (1.0 / f64::powf(1.0 + i, n - 1.0))) / i) + pmt)
                 } else if self.pmt == None {
                    Err(Error::Payment)
                } else if self.i == None {
                    Err(Error::InterestRate)
                } else if self.n == None {
                    Err(Error::NumberOfPeriods)
                } else {
                    Err(Error::Unknown)
                }             
            },
            AnnuityType::RegularAnnuity => {
                if let (Some(pmt), Some(i), Some(n)) = (self.pmt, self.i, self.n) {
                    let i = i / 100.0;
                    Ok(pmt * ((1.0 - (1.0 / f64::powf(1.0 + i, n))) / i))
                } else if self.pmt == None {
                    Err(Error::Payment)
                } else if self.i == None {
                    Err(Error::InterestRate)
                } else if self.n == None {
                    Err(Error::NumberOfPeriods)
                } else {
                    Err(Error::Unknown)
                }
            }
        }
    }
}

impl FutureValue for Annuity {
    fn fv(&self) -> Result<f64, Error> {
        match &self.kind {
            AnnuityType::AnnuityDue => {
                if let (Some(pmt), Some(i), Some(n)) = (self.pmt, self.i, self.n) { 
                    let i = i / 100.0;
                    Ok(pmt * ((f64::powf(1.0 + i, n) - 1.0) / i) * (1.0 + i))
                 } else if self.pmt == None {
                    Err(Error::Payment)
                } else if self.i == None {
                    Err(Error::InterestRate)
                } else if self.n == None {
                    Err(Error::NumberOfPeriods)
                } else {
                    Err(Error::Unknown)
                }   
            },
            AnnuityType::RegularAnnuity => {
                if let (Some(pmt), Some(i), Some(n)) = (self.pmt, self.i, self.n) { 
                    let i = i / 100.0;
                    Ok(pmt * ((f64::powf(1.0 + i, n) - 1.0) / i))
                 } else if self.pmt == None {
                    Err(Error::Payment)
                } else if self.i == None {
                    Err(Error::InterestRate)
                } else if self.n == None {
                    Err(Error::NumberOfPeriods)
                } else {
                    Err(Error::Unknown)
                }   
            }
        }
    }
}

impl Payment for Annuity {
    fn pmt(&self) -> Result<f64, Error> {
        match &self.kind {
            AnnuityType::AnnuityDue => {
                if let (Some(n), Some(i), Some(pv)) = (self.n, self.i, self.pv) { 
                    let i = i / 100.0;
                    Ok(pv / (((1.0 - (1.0 / f64::powf(1.0 + i, n - 1.0))) / i) + 1.0))
                 } else if let (Some(n), Some(i), Some(fv)) = (self.n, self.i, self.fv) { 
                    let i = i / 100.0;
                    Ok(fv / (((f64::powf(1.0 + i, n) - 1.0) / i) * (1.0 + i)))
                 } else if self.n == None {
                    Err(Error::NumberOfPeriods)
                } else if self.i == None {
                    Err(Error::InterestRate)
                } else if self.pv == None && self.fv == None {
                    Err(Error::ValueError)
                } else {
                    Err(Error::Unknown)
                }
            },
            AnnuityType::RegularAnnuity => {
                if let (Some(n), Some(i), Some(pv)) = (self.n, self.i, self.pv) { 
                    let i = i / 100.0;
                    Ok(pv / ((1.0 - (1.0 / f64::powf(1.0 + i, n))) / i))
                 } else if let (Some(n), Some(i), Some(fv)) = (self.n, self.i, self.fv) { 
                    let i = i / 100.0;
                    Ok(fv / ((f64::powf(1.0 + i, n) - 1.0) / i))
                 } else if self.n == None {
                    Err(Error::NumberOfPeriods)
                } else if self.i == None {
                    Err(Error::InterestRate)
                } else if self.pv == None && self.fv == None {
                    Err(Error::ValueError)
                } else {
                    Err(Error::Unknown)
                }
            }
        }
    }
}

impl NumberOfPeriods for Annuity {
    fn n(&self) -> Result<f64, Error> {
        match &self.kind {
            AnnuityType::AnnuityDue => {
                if let (Some(pmt), Some(i), Some(pv)) = (self.pmt, self.i, self.pv) { 
                    let i = i / 100.0;
                    Ok(-f64::ln(1.0 + i * (1.0 - (pv / pmt))) / f64::ln(1.0 + i) + 1.0)
                 } else if let (Some(pmt), Some(i), Some(fv)) = (self.pmt, self.i, self.fv) { 
                    let i = i / 100.0;
                    Ok(f64::ln(1.0 + (fv / (pmt * (1.0 + i)) * i)) / f64::ln(1.0 + i))
                 } else if self.pmt == None {
                    Err(Error::Payment)
                } else if self.i == None {
                    Err(Error::InterestRate)
                } else if self.pv == None && self.fv == None {
                    Err(Error::ValueError)
                } else {
                    Err(Error::Unknown)
                }   
            },
            AnnuityType::RegularAnnuity => {
                if let (Some(pmt), Some(i), Some(pv)) = (self.pmt, self.i, self.pv) { 
                    let i = i / 100.0;
                    Ok(-f64::ln(1.0 - ((pv / pmt) * i)) / f64::ln(1.0 + i))
                 } else if let (Some(pmt), Some(i), Some(fv)) = (self.pmt, self.i, self.fv) { 
                    let i = i / 100.0;
                    Ok(f64::ln(1.0 + ((fv / pmt) * i)) / f64::ln(1.0 + i))
                 } else if self.pmt == None {
                    Err(Error::Payment)
                } else if self.i == None {
                    Err(Error::InterestRate)
                } else if self.pv == None && self.fv == None {
                    Err(Error::ValueError)
                } else {
                    Err(Error::Unknown)
                }  
            }
        }
    }
}

impl Annuity {
    pub fn new(kind: &AnnuityType, input: &Opt) -> Annuity {
        match kind {
            AnnuityType::RegularAnnuity => {
                Annuity {
                    kind: AnnuityType::RegularAnnuity,
                    fv: input.fv,
                    pv: input.pv,
                    i: input.i,
                    n: input.n,
                    pmt: input.pmt
                }
            },
            AnnuityType::AnnuityDue => {
                Annuity {
                    kind: AnnuityType::AnnuityDue,
                    fv: input.fv,
                    pv: input.pv,
                    i: input.i,
                    n: input.n,
                    pmt: input.pmt
                }
            }
        }
    }
}
