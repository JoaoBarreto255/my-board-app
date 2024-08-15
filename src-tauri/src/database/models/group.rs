use core::option::Option;
use std::fmt::Debug;
use std::rc::Rc;

use crate::database::models::Board;
use crate::database::models::ModelQueryBuilder;

use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Group {
    id: Option<i64>,
    name: Rc<String>,
    icon: Option<String>,
    position: u32,
    boards: Vec<Rc<Board>>,
}

impl Group {
    pub fn new(id: Option<i64>, name: Rc<String>, icon: Option<String>, position: u32) -> Group {
        Group {
            id,
            name,
            icon,
            boards: vec![],
            position,
        }
    }

    /// Get [`Group`] identifier if exists
    pub fn get_id(&self) -> Option<i64> {
        self.id
    }

    /// Sets the id of this [`Group`].
    pub fn set_id(&mut self, id: Option<i64>) -> &mut Group {
        self.id = id;

        return self;
    }

    /// Get [`Group`] name.
    pub fn get_name(&self) -> Rc<String> {
        self.name.clone()
    }

    /// Sets the name of this [`Group`].
    pub fn set_name(&mut self, name: Rc<String>) -> &mut Group {
        self.name = name;

        return self;
    }

    /// Get [`Group`] icon path if exists.
    pub fn get_icon(&self) -> Option<String> {
        return match &self.icon {
            Some(v) => Some(v.clone()),
            None => None,
        };
    }

    /// Sets the icon of this [`Group`].
    pub fn set_icon(&mut self, icon: Option<String>) -> &mut Group {
        self.icon = icon;

        return self;
    }

    /// Get [`Group`] postion on list.
    pub fn get_position(&self) -> u32 {
        self.position
    }

    /// Sets new [`Group`] position.
    pub fn set_position(&mut self, position: u32) -> &mut Group {
        self.position = position;

        return self;
    }

    /// Get [`Group`] boards
    pub fn get_boards(&self) -> &Vec<Rc<Board>> {
        &self.boards
    }

    /// Sets the boards of this [`Group`].
    pub fn set_boards(&mut self, boards: Vec<Rc<Board>>) -> &mut Group {
        self.boards = boards;

        return self;
    }

    /// Add board to stack of boards
    pub fn add_board(&mut self, board: Rc<Board>) -> &mut Group {
        self.boards.push(board);

        return self;
    }
}

impl ModelQueryBuilder for Group {
    fn insert_query(&self) -> &str {
        r#"INSERT INTO groups(name, icon, position) VALUES (?1, ?2, ?3);"#
    }

    fn update_query(&self) -> &str {
        r#"UPDATE groups SET name = ?1, icon = ?2, position = ?3 WHERE id = ?4;"#
    }

    fn delete_query(&self) -> &str {
        r#"DELETE FROM groups WHERE id = ?1;"#
    }

    fn insert(&mut self, conn: &Connection) -> Result<bool> {
        conn.execute(
            self.insert_query(),
            params![self.get_name(), self.get_icon(), self.get_position(),],
        )?;
        let last_id = conn.last_insert_rowid();
        self.set_id(Some(last_id));

        Ok(true)
    }

    fn update(&self, conn: &Connection) -> Result<bool> {
        let id = self
            .get_id()
            .expect("Cannot update a group not persisted before!");
        let count = conn.execute(
            self.update_query(),
            params![self.get_name(), self.get_icon(), self.get_position(), id],
        )?;

        Ok(count > 0)
    }

    fn delete(&self, conn: &Connection) -> Result<bool> {
        let id = self
            .get_id()
            .expect("Could not delete group that is not persisted");
        let count = conn.execute(self.delete_query(), params![id])?;

        Ok(count > 0)
    }
}
