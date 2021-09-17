//!
//! The Toql Mock Db provides a dummy database that can be used for testing or documentation examples.
//! The implementation collects all SQL statements that can be asserted.
//! For loading it return default values.
//!

use toql::backend::context::Context;
use toql::prelude::{Cache, Sql};

pub mod backend;
pub mod row;
pub mod toql_api;

use backend::MockDbBackend;

pub struct MockDb<'a> {
    backend: MockDbBackend<'a>,
}

impl<'a> MockDb<'a> {
    pub fn clear(&mut self) {
        self.backend.sqls.clear();
    }
    pub fn sqls(&mut self) -> &Vec<Sql> {
        &self.backend.sqls
    }
}

impl<'a> MockDb<'a> {
    pub fn from(cache: &'a Cache) -> MockDb<'a> {
        Self::with_context(cache, Context::default())
    }

    pub fn with_context(cache: &'a Cache, context: Context) -> MockDb<'a> {
        MockDb {
            backend: MockDbBackend {
                cache,
                context,
                sqls: Vec::new(),
            },
        }
    }
}
