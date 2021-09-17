use std::collections::{HashMap, HashSet};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};
use toql::backend::{context::Context, Backend};
use toql::prelude::{AliasFormat, Cache, Result, Sql, SqlArg, TableMapperRegistry, ToqlError};
use toql::{page::Page, sql_builder::build_result::BuildResult};

use crate::row::Row;
use async_trait::async_trait;

pub(crate) struct MockDbBackend<'a> {
    pub(crate) sqls: Vec<Sql>,
    pub(crate) context: Context,
    pub(crate) cache: &'a Cache,
}

// Implement template functions for updating entities
#[async_trait]
impl<'a> Backend<Row, ToqlError> for MockDbBackend<'a> {
    async fn execute_sql(&mut self, sql: Sql) -> Result<()> {
        self.sqls.push(sql);
        Ok(())
    }
    async fn insert_sql(&mut self, sql: Sql) -> Result<Vec<SqlArg>> {
        let number_of_rows: u64 = *(&(sql.0.as_str()).matches(')').count()) as u64;

        self.sqls.push(sql);
        let ids = (0..number_of_rows)
            .map(|n| SqlArg::U64((n + 100).into()))
            .collect::<Vec<_>>();
        Ok(ids)
    }

    async fn select_sql(&mut self, sql: Sql) -> Result<Vec<Row>> {
        self.sqls.push(sql);
        Ok(vec![])
    }
    fn prepare_page(&self, _result: &mut BuildResult, _page: &Page) {}

    async fn select_max_page_size_sql(&mut self, sql: Sql) -> Result<u64> {
        self.sqls.push(sql);
        Ok(0)
    }
    async fn select_count_sql(&mut self, sql: Sql) -> Result<u64> {
        self.sqls.push(sql);
        Ok(0)
    }

    fn registry(&self) -> std::result::Result<RwLockReadGuard<'_, TableMapperRegistry>, ToqlError> {
        self.cache.registry.read().map_err(ToqlError::from)
    }

    fn registry_mut(
        &mut self,
    ) -> std::result::Result<RwLockWriteGuard<'_, TableMapperRegistry>, ToqlError> {
        self.cache.registry.write().map_err(ToqlError::from)
    }

    fn roles(&self) -> &HashSet<String> {
        &self.context.roles
    }
    fn alias_format(&self) -> AliasFormat {
        self.context.alias_format.clone()
    }
    fn aux_params(&self) -> &HashMap<String, SqlArg> {
        &self.context.aux_params
    }
}
