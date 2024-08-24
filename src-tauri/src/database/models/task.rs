use core::option::Option;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

use crate::database::models::Board;
use crate::database::models::ModelQueryBuilder;
use crate::database::models::Priority;
use crate::database::models::State;
use rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DurationInput {
    Minutes(u32),
    HoursAndMinutes(u32, u32),
}

impl DurationInput {
    pub fn value(&self) -> u32 {
        match self {
            Self::Minutes(val) => *val,
            Self::HoursAndMinutes(hours, minutes) => *hours * 60 + *minutes,
        }
    }

    pub fn from(value: u32) -> Self {
        if value < 60 {
            return Self::Minutes(value);
        }

        return Self::HoursAndMinutes(value / 60, value % 60);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: Option<i64>,
    name: String,
    description: Option<String>,
    duration: u32,
    progress: Option<f32>,
    priority: Priority,
    state: Rc<State>,
    #[serde(skip)]
    board: Option<Weak<Board>>,
    position: u32,
    started_at: Option<String>,
    ended_at: Option<String>,
}

impl Task {
    pub fn new(
        name: String,
        description: Option<String>,
        duration: u32,
        priority: Priority,
        state: Rc<State>,
        board: Option<Weak<Board>>,
        position: u32,
    ) -> Task {
        return Task {
            id: None,
            name,
            description,
            duration,
            progress: None,
            priority,
            state,
            board,
            position,
            started_at: None,
            ended_at: None,
        };
    }

    /// get id from [`Task`]
    pub fn get_id(&self) -> Option<i64> {
        self.id
    }

    /// Sets the id of this [`Task`].
    pub fn set_id(&mut self, id: Option<i64>) -> &mut Self {
        self.id = id;

        return self;
    }

    /// get name from [`Task`]
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Sets the name of this [`Task`].
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;

        return self;
    }

    /// get description from [`Task`]
    pub fn get_description(&self) -> &Option<String> {
        return &self.description;
    }

    /// Sets the descriptio of this [`Task`].
    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);

        return self;
    }

    /// get duration from [`Task`] and returns
    /// tuple with duration splited in (hours, minutes).
    pub fn get_duration(&self) -> DurationInput {
        return DurationInput::from(self.duration);
    }

    /// Sets the duration of this [`Task`].
    pub fn set_duration(&mut self, duration: DurationInput) -> &mut Self {
        self.duration = duration.value();

        return self;
    }

    /// obtains current [`Task`] progress.
    pub fn get_progress(&self) -> Option<f32> {
        self.progress
    }

    /// update current [`Task`] progress.
    pub fn set_progress(&mut self, progress: Option<f32>) -> &mut Self {
        self.progress = progress;

        return self;
    }

    /// obtains current [`Task`] priority.
    pub fn get_priority(&self) -> Priority {
        match self.priority {
            Priority::High => Priority::High,
            Priority::Normal => Priority::Normal,
            Priority::Low => Priority::Low,
            Priority::Now => Priority::Now,
        }
    }

    /// update current [`Task`] priority.
    pub fn set_priority(&mut self, priority: Priority) -> &mut Self {
        self.priority = priority;

        return self;
    }

    /// obtains current [`Task`] state.
    pub fn get_state(&self) -> Rc<State> {
        Rc::clone(&self.state)
    }

    /// update current [`Task`] state.
    pub fn set_state(&mut self, state: Rc<State>) -> &mut Self {
        self.state = state;

        return self;
    }

    /// obtains current [`Task`] task_group.
    pub fn get_board(&self) -> Option<Rc<Board>> {
        if let Some(board) = &self.board {
            let result = board.upgrade()?;
            return Some(result);
        }

        return None;
    }

    /// update current [`Task`] task_group.
    pub fn set_board(&mut self, board: Weak<Board>) -> &mut Self {
        self.board = Some(board);

        return self;
    }

    /// get [`Task`] postion on state
    pub fn get_position(&self) -> u32 {
        self.position
    }

    /// Sets new [`Task`] position.
    pub fn set_position(&mut self, position: u32) -> &mut Self {
        self.position = position;

        return self;
    }

    /// obtains current [`Task`] started_at.
    pub fn get_started_at(&self) -> &Option<String> {
        return &self.started_at;
    }

    /// update current [`Task`] started_at.
    /// if current task already updated it, just ignore
    pub fn set_started_ed(&mut self, started_et: String) -> &mut Self {
        if self.started_at.is_some() {
            return self;
        }

        self.started_at = Some(started_et);

        return self;
    }

    /// obtains current [`Task`] started_at.
    pub fn get_ended_at(&self) -> &Option<String> {
        &self.ended_at
    }

    /// update current [`Task`] ended_at.
    /// if current task already updated it, just ignore.
    pub fn set_ended_ed(&mut self, ended_at: String) -> &mut Self {
        if self.ended_at.is_some() {
            return self;
        }

        self.ended_at = Some(ended_at);

        return self;
    }
}

impl ModelQueryBuilder for Task {
    fn insert_query(&self) -> &str {
        r#"INSERT states(
            name, description, duration, priority
            , state_id, board_id, position
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);"#
    }

    fn update_query(&self) -> &str {
        r#"UPDATE states SET
            name = ?1, description = ?2, duration = ?3, progress = ?4
            , priority = ?5, state_id = ?6, board_id = ?7, position = ?8
            , started_at = ?9, ended_at = ?10
        WHERE id = ?11;"#
    }

    fn delete_query(&self) -> &str {
        r#"DELETE * FROM states WHERE id = ?1;"#
    }

    fn insert(&mut self, conn: &Connection) -> Result<bool> {
        let state = self
            .get_state()
            .get_id()
            .expect("state must already be persited!");
        let board = self
            .get_board()
            .unwrap()
            .get_id()
            .expect("Board must be already persisted!");

        conn.execute(
            self.insert_query(),
            params![
                self.get_name(),
                self.get_description(),
                self.get_duration().value(),
                self.get_priority().code(),
                state,
                board,
                self.get_position()
            ],
        )?;

        let last_row_id = conn.last_insert_rowid();

        self.set_id(Option::Some(last_row_id));

        return Ok(true);
    }

    fn update(&self, conn: &Connection) -> Result<bool> {
        let id = self.get_id().expect("Cannot update an unexistent task!");
        let state = self
            .get_state()
            .get_id()
            .expect("state must already be persited!");
        let board = self
            .get_board()
            .unwrap()
            .get_id()
            .expect("Board must be already persisted!");
        let count = conn.execute(
            &self.update_query(),
            params![
                self.get_name(),
                self.get_description(),
                self.get_duration().value(),
                self.get_progress(),
                self.get_priority().code(),
                state,
                board,
                self.get_position(),
                self.get_started_at(),
                self.get_ended_at(),
                id
            ],
        )?;
        return Ok(count > 0);
    }

    fn delete(&self, conn: &Connection) -> Result<bool> {
        let id = self.get_id().expect("Cannot delete a task that not exists");
        let count = conn.execute(self.delete_query(), params![id])?;
        return Ok(count > 0);
    }
}
