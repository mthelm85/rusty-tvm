use std::f64;

use super::errors::*;
use super::traits::*;

#[derive(Debug)]
pub struct LumpSum {
    pub fv: Option<f64>,
    pub pv: Option<f64>,
    pub n: Option<f64>,
    pub i: Option<f64>
}

impl PresentValue for LumpSum {
    fn pv(&self) -> Result<f64, Error> {
        if let (Some(fv), Some(i), Some(n)) = (self.fv, self.i, self.n) {
            Ok(fv / f64::powf(1.0 + i / 100.0, n))
        } else if self.fv == None {
            Err(Error::FutureValue)
        } else if self.i == None {
            Err(Error::InterestRate)
        } else if self.n == None {
            Err(Error::NumberOfPeriods)
        } else {
            Err(Error::Unknown)
        }
    }
}

impl FutureValue for LumpSum {
    fn fv(&self) -> Result<f64, Error> {
        if let (Some(pv), Some(i), Some(n)) = (self.pv, self.i, self.n) {
            Ok(pv * f64::powf(1.0 + i / 100.0, n))
        } else if self.pv == None {
            Err(Error::PresentValue)
        } else if self.i == None {
            Err(Error::InterestRate)
        } else if self.n == None {
            Err(Error::NumberOfPeriods)
        } else {
            Err(Error::Unknown)
        }
    }
}

impl NumberOfPeriods for LumpSum {
    fn n(&self) -> Result<f64, Error> {
        if let (Some(pv), Some(fv), Some(i)) = (self.pv, self.fv, self.i) {
            Ok(f64::ln(fv / pv) / f64::ln(1.0 + i / 100.0))
        } else if self.pv == None {
            Err(Error::PresentValue)
        } else if self.fv == None {
            Err(Error::FutureValue)
        } else if self.i == None {
            Err(Error::InterestRate)
        } else {
            Err(Error::Unknown)
        }
    }
}

impl InterestRate for LumpSum {
    fn i(&self) -> Result<f64, Error> {
        if let (Some(pv), Some(fv), Some(n)) = (self.pv, self.fv, self.n) {
            Ok(((fv / pv).powf(1.0 / n) - 1.0) * 100.0)
        } else if self.pv == None {
            Err(Error::PresentValue)
        } else if self.fv == None {
            Err(Error::FutureValue)
        } else if self.n == None {
            Err(Error::NumberOfPeriods)
        } else {
            Err(Error::Unknown)
        }
        
    }
}