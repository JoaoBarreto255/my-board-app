mod board;
mod group;
mod priority;
mod state;
mod task;

pub use board::Board;
pub use group::Group;
pub use priority::Priority;
pub use state::State;
pub use task::*;

use rusqlite::{Connection, Result};

/// trait for help to insert, update and delete data for each model.
pub trait ModelQueryBuilder {
    /// returns model insert sql query.
    fn insert_query(&self) -> &str;

    /// returns model update sql query.
    fn update_query(&self) -> &str;

    /// returns model delete sql query.
    fn delete_query(&self) -> &str;

    /// runs model insert action.
    fn insert(&mut self, conn: &Connection) -> Result<bool>;

    /// runs model update action.
    fn update(&self, conn: &Connection) -> Result<bool>;

    /// runs model delete action.
    fn delete(&self, conn: &Connection) -> Result<bool>;
}
