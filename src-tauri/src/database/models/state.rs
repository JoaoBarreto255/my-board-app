use core::option::Option;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

use crate::database::models::Board;
use crate::database::models::ModelQueryBuilder;

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    id: Option<i64>,
    name: String,
    color: Option<String>,
    position: u32,
    #[serde(skip)]
    board: Option<Weak<Board>>,
}

impl State {
    pub fn new(
        id: Option<i64>,
        name: String,
        color: Option<String>,
        board: Option<Weak<Board>>,
        position: u32,
    ) -> State {
        State {
            id,
            name,
            color,
            position,
            board,
        }
    }

    /// Sets the id of this [`State`].
    pub fn set_id(&mut self, id: Option<i64>) -> &mut State {
        self.id = id;

        return self;
    }

    /// Get [`State`] identifier if exists.
    pub fn get_id(&self) -> Option<i64> {
        self.id
    }

    /// Get [`State`] name.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Update [`State`] name.
    pub fn set_name(&mut self, name: String) -> &mut State {
        self.name = name;

        return self;
    }

    /// Get [`State`] label color if exists.
    pub fn get_color(&self) -> &Option<String> {
        &self.color
    }

    /// Sets the color of this [`State`].
    pub fn set_color(&mut self, color: Option<String>) -> &mut State {
        self.color = color;

        return self;
    }

    /// Get [`State`] postion in board.
    pub fn get_position(&self) -> u32 {
        self.position
    }

    /// Change [`State`] position in board.
    pub fn set_position(&mut self, position: u32) -> &mut State {
        self.position = position;

        return self;
    }

    /// Get [`Board`] from [`State`]
    pub fn get_board(&self) -> Option<Rc<Board>> {
        match &self.board {
            None => None,
            Some(board) => Some(board.upgrade().expect("Opss! Missing board data."))
        }
    }

    /// Change [`State`] [`Board`].
    pub fn set_board(&mut self, board: &Weak<Board>) -> &mut State {
        self.board = Some(board.clone());

        return self;
    }
}

impl ModelQueryBuilder for State {
    fn insert_query(&self) -> &str {
        r#"INSERT states(name, color, position, board_id) VALUES (?1, ?2, ?3, ?4);"#
    }

    fn insert(&mut self, conn: &Connection) -> Result<bool> {
        conn.execute(
            self.insert_query(),
            params![
                self.get_name(),
                self.get_color(),
                self.get_position(),
                self.get_board().expect("Cannot create state without board").get_id(),
            ],
        )?;
        self.set_id(Some(conn.last_insert_rowid()));

        Ok(true)
    }

    fn update_query(&self) -> &str {
        r#"UPDATE states SET name = ?1, color = ?2, position = ?3 WHERE id = ?4;"#
    }

    fn update(&self, conn: &Connection) -> Result<bool> {
        let count = conn.execute(
            self.update_query(),
            params![
                self.get_name(),
                self.get_color(),
                self.get_position(),
                self.get_id()
            ],
        )?;

        Ok(count > 0)
    }

    fn delete_query(&self) -> &str {
        r#"DELETE * FROM states WHERE id = ?1;"#
    }

    fn delete(&self, conn: &Connection) -> Result<bool> {
        let count = conn.execute(
            self.delete_query(),
            params![self.get_id().expect("Entity not persisted yet!")],
        )?;

        Ok(count > 0)
    }
}
