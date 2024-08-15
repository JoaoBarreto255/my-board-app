use rusqlite::{Connection, Result};

use super::models::ModelQueryBuilder;

pub struct Manager {
    connection: Connection,
}

impl Manager {
    /// Manage insert model in DB
    pub fn insert<M: ModelQueryBuilder>(&self, model: &mut M) -> Result<bool> {
        return model.insert(&self.connection);
    }

    /// Manage update model in DB
    pub fn update<M: ModelQueryBuilder>(&self, model: &M) -> Result<bool> {
        return model.update(&self.connection);
    }

    /// Manage deletion of models in DB
    pub fn delete<M: ModelQueryBuilder>(&self, model: &M) -> Result<bool> {
        return model.delete(&self.connection);
    }
}
