use super::errors::*;

pub trait PresentValue {
    fn pv(&self) -> Result<f64, Error>;
}

pub trait FutureValue {
    fn fv(&self) -> Result<f64, Error>;
}

pub trait NumberOfPeriods {
    fn n(&self) -> Result<f64, Error>;
}

pub trait InterestRate {
    fn i(&self) -> Result<f64, Error>;
}

pub trait Payment {
    fn pmt(&self) -> Result<f64, Error>;
}