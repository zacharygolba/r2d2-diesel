extern crate diesel;
extern crate r2d2;

use diesel::{Connection, ConnectionError, LoadDsl};
use diesel::mysql::MysqlConnection;
use r2d2::ManageConnection;
use std::convert::Into;
use std::fmt;
use std::marker::PhantomData;

pub struct ConnectionManager<T> {
    database_url: String,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send + 'static> Sync for ConnectionManager<T> {
}

impl<T> ConnectionManager<T> {
    pub fn new<S: Into<String>>(database_url: S) -> Self {
        ConnectionManager {
            database_url: database_url.into(),
            _marker: PhantomData,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    ConnectionError(ConnectionError),
    QueryError(diesel::result::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ConnectionError(ref e) => e.fmt(f),
            Error::QueryError(ref e) => e.fmt(f),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ConnectionError(ref e) => e.description(),
            Error::QueryError(ref e) => e.description(),
        }
    }
}

impl ManageConnection for ConnectionManager<diesel::mysql::MysqlConnection> where
    MysqlConnection: Connection + Send + 'static,
{
    type Connection = MysqlConnection;
    type Error = Error;

    fn connect(&self) -> Result<MysqlConnection, Error> {
        MysqlConnection::establish(&self.database_url)
            .map_err(Error::ConnectionError)
    }

    // ::diesel::expression::sql_literal::sql
    fn is_valid(&self, conn: &mut MysqlConnection) -> Result<(), Error> {
        // conn.execute("SELECT 1").map(|_| ()).map_err(Error::QueryError)
        ::diesel::expression::sql_literal::sql::<::diesel::types::Bool>("SELECT 1")
            .get_result::<bool>(conn)
            .map(|_| ())
            .map_err(Error::QueryError)
    }

    fn has_broken(&self, _conn: &mut MysqlConnection) -> bool {
        false
    }
}
