use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error: Unable to parse {} as a valid calculation", c)]
    ParseCalculation { c: String },

    #[error("Error: Present value is required for this calculation. Use '--pv' or '-p'")]
    PresentValue,

    #[error("Error: Future value is required for this calculation. Use '--fv' or '-f'")]
    FutureValue,

    #[error("Error: Discount/interest rate is required for this calculation. Use '--i' or '-i'")]
    InterestRate,

    #[error("Error: Number of periods is required for this calculation. Use '--n' or '-n'")]
    NumberOfPeriods,

    #[error("Error: Payment amount is required for this calculation. Use '--pmt' or '-r'")]
    Payment,

    #[error("Error: This calculation requires either present or future value")]
    ValueError,

    #[error("Error: You just found a new way to break this program!")]
    Unknown,
}