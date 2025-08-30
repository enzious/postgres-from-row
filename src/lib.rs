#![feature(error_generic_member_access)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

use std::backtrace::Backtrace;

pub use postgres_from_row_derive::FromRow;
use thiserror::Error;
pub use tokio_postgres;

/// A trait that allows mapping rows from either [postgres](<https://docs.rs/postgres>) or [tokio-postgres](<https://docs.rs/tokio-postgres>), to other types.
pub trait FromRow: Sized {
    /// Perform the conversion.
    ///
    /// # Panics
    ///
    /// Panics if the row does not contain the expected column names.
    fn from_row(row: &tokio_postgres::Row) -> Self;

    /// Try's to perform the conversion.
    ///
    /// Will return an error if the row does not contain the expected column names.
    fn try_from_row(row: &tokio_postgres::Row) -> Result<Self, FromRowError>;

    /// Perform the conversion on a vector of rows.
    ///
    /// # Panics
    ///
    /// Panics if the row does not contain the expected column names.
    fn from_rows(row: &[tokio_postgres::Row]) -> Vec<Self>;

    /// Try's to perform the conversion on a vector of rows.
    ///
    /// Will return an error if the row does not contain the expected column names.
    fn try_from_rows(row: &[tokio_postgres::Row]) -> Result<Vec<Self>, FromRowError>;

    /// Perform the conversion on an optional row.
    ///
    /// # Panics
    ///
    /// Panics if the row does not contain the expected column names.
    fn from_row_maybe(row: Option<&tokio_postgres::Row>) -> Option<Self>;

    /// Try's to perform the conversion on an optional row.
    ///
    /// Will return an error if the row does not contain the expected column names.
    fn try_from_row_maybe(row: Option<&tokio_postgres::Row>) -> Result<Option<Self>, FromRowError>;

    /// Perform the conversion on an optional row and expect it exists.
    ///
    /// # Panics
    ///
    /// Panics if the row does not contain the expected column names, or no result.
    fn from_row_expect(row: Option<&tokio_postgres::Row>) -> Self;

    /// Try's to perform the conversion on an optional row.
    ///
    /// Will return an error if the row does not contain the expected column names, or no result.
    fn try_from_row_expect(row: Option<&tokio_postgres::Row>) -> Result<Self, FromRowError>;
}

/// Error throw if no row is found when expected.
#[derive(Debug, Error)]
pub enum FromRowError {
    ///
    #[error("No row found when expected")]
    NoRow,
    ///
    #[error("{source}")]
    PostgresError {
        ///
        source: tokio_postgres::Error,
        ///
        backtrace: Backtrace,
    },
}
