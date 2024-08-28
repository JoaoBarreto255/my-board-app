use core::option::Option;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

use crate::database::models::Group;
use crate::database::models::ModelQueryBuilder;
use crate::database::models::State;
use crate::database::models::Task;

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    id: Option<i64>,
    name: String,
    #[serde(skip_deserializing)]
    states: Vec<Rc<State>>,
    #[serde(skip_deserializing)]
    tasks: Vec<Weak<Task>>,
    position: u32,
    #[serde(skip)]
    group: Weak<Group>,
}

impl Board {
    pub fn new(id: Option<i64>, name: String, group: Weak<Group>, position: u32) -> Board {
        Board {
            id,
            name,
            states: vec![],
            tasks: vec![],
            position,
            group,
        }
    }

    pub fn get_id(&self) -> Option<i64> {
        self.id
    }

    /// Sets the id of this [`Board`].
    pub fn set_id(&mut self, id: Option<i64>) -> &mut Board {
        self.id = id;

        return self;
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Sets the name of this [`Board`].
    pub fn set_name(&mut self, name: String) -> &mut Board {
        self.name = name;

        return self;
    }

    pub fn get_states(&self) -> &Vec<Rc<State>> {
        &self.states
    }

    /// Sets the states of this [`Board`].
    pub fn set_states(&mut self, states: Vec<Rc<State>>) -> &mut Board {
        self.states = states;

        return self;
    }

    /// Append [`State`] to stack of [`Board`] states.
    pub fn add_new_state(&mut self, state: Rc<State>) -> &mut Board {
        self.states.push(state);

        return self;
    }

    pub fn get_tasks(&self) -> &Vec<Rc<Task>> {
        &self.tasks
    }

    /// Sets the tasks of this [`Board`].
    pub fn set_tasks(&mut self, tasks: Vec<Rc<Task>>) -> &mut Board {
        self.tasks = tasks;

        return self;
    }

    /// Append [`Task`] to stack of [`Board`] tasks.
    pub fn add_task(&mut self, task: Rc<Task>) -> &mut Board {
        self.tasks.push(task);

        return self;
    }

    pub fn get_position(&self) -> u32 {
        self.position
    }

    /// Sets new [`Board`] position.
    pub fn set_position(&mut self, position: u32) -> &mut Board {
        self.position = position;

        return self;
    }

    pub fn get_group(&self) -> Rc<Group> {
        self.group.upgrade().expect("theres no group!")
    }

    /// Sets the group of this [`Board`].
    pub fn set_group(&mut self, group: Weak<Group>) -> &mut Board {
        self.group = group;

        return self;
    }
}

impl ModelQueryBuilder for Board {
    fn insert_query(&self) -> &str {
        r#"INSERT INTO boards(name, position, group_id) VALUES (?1, ?2, ?3);"#
    }

    fn update_query(&self) -> &str {
        r#"UPDATE boards SET name = ?1,position = ?2, group_id = ?3 WHERE id = ?4;"#
    }

    fn delete_query(&self) -> &str {
        r#"DELETE FROM boards WHERE id = ?1;"#
    }

    fn insert(&mut self, conn: &Connection) -> Result<bool> {
        let group = self.get_group().get_id();
        conn.execute(
            self.insert_query(),
            params![self.get_name(), self.get_position(), group,],
        )?;

        self.set_id(Some(conn.last_insert_rowid()));

        return Ok(true);
    }

    fn update(&self, conn: &Connection) -> Result<bool> {
        let count = conn.execute(
            self.update_query(),
            params![
                self.get_name(),
                self.get_position(),
                self.get_group().get_id(),
                self.get_id().expect("Entity not persisted!"),
            ],
        )?;

        Ok(count > 0)
    }

    fn delete(&self, conn: &Connection) -> Result<bool> {
        let count = conn.execute(
            self.delete_query(),
            params![self.get_id().expect("Entity not persisted")],
        )?;

        Ok(count > 0)
    }
}
