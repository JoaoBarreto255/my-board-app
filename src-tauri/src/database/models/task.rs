use core::option::Option;
use std::fmt::Debug;
use std::result;
use std::rc::{Rc, Weak};

use crate::database::models::Board;
use crate::database::models::ModelQueryBuilder;
use crate::database::models::Priority;
use crate::database::models::State;
use rusqlite::{params, Connection, Result};
use serde::de::{Error, Visitor};
use serde::ser::SerializeStruct;
use serde::{Serialize, Deserialize};

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

impl Serialize for DurationInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    {
        serializer.serialize_u32(self.value())
    }   
}

impl<'de> Deserialize<'de> for DurationInput {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Ok(Self::from(value))
    }
}

#[derive(Debug)]
pub struct Task {
    id: Option<i64>,
    name: String,
    description: Option<String>,
    duration: u32,
    progress: Option<f32>,
    priority: Priority,
    state: Rc<State>,
    board: Option<Weak<Board>>,
    position: u32,
    started_at: Option<String>,
    ended_at: Option<String>,
}

impl Serialize for Task {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut task = serializer.serialize_struct("Task", 11)?;
        task.serialize_field("id", &self.id)?;
        task.serialize_field("name", &self.name)?;
        task.serialize_field("description", &self.description)?;
        task.serialize_field("duration", &self.duration)?;
        task.serialize_field("progress", &self.progress)?;
        task.serialize_field("priority", &self.priority)?;
        task.serialize_field("state_id", &self.state.as_ref().get_id())?;
        task.serialize_field("position", &self.position)?;
        task.serialize_field("started_at", &self.started_at)?;
        task.serialize_field("ended_at", &self.ended_at)?;
        task.end()
    }
}

impl<'a> Deserialize<'a> for Task {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        /// Holds task field names
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field { Id, Name, Description, Duration, Progress, Priority, StateId, Position, StartedAt, EndedAt }

        struct TaskVisitor;

        impl<'b> Visitor<'b> for TaskVisitor {
            type Value = Task;
            
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct crate::database::models::Task")
            }
            
            fn visit_seq<A>(self, mut seq: A) -> result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'b>,
            {
                let id = seq.next_element()?.ok_or(Error::invalid_length(0, &self))?;
                let name = seq.next_element()?.ok_or(Error::invalid_length(0, &self))?;
                let description = seq.next_element()?.ok_or(Error::invalid_length(0, &self))?;
                let duration = seq.next_element()?.ok_or(Error::invalid_length(0, &self))?;
                let progress = seq.next_element()?.ok_or(Error::invalid_length(0, &self))?;
                let priority = seq.next_element()?.ok_or(Error::invalid_length(0, &self))?;
                let state_id = seq.next_element()?.ok_or(Error::invalid_length(0, &self))?;
                let position = seq.next_element()?.ok_or(Error::invalid_length(0, &self))?;
                let started_at = seq.next_element()?.ok_or(Error::invalid_length(0, &self))?;
                let ended_at = seq.next_element()?.ok_or(Error::invalid_length(0, &self))?;

                let state = State::new(Some(state_id), "".to_string(), None, None, 0);
                
                Ok(Task {
                    id, name, description, duration, progress, priority,
                    state: Rc::from(state), board: None, position, started_at, ended_at
                })
            }
            
            fn visit_map<A>(self, mut map: A) -> result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'b>,
            {
                let mut id = None;
                let mut name = None;
                let mut description = None;
                let mut duration = None;
                let mut progress = None;
                let mut priority = None;
                let mut state_id = None;
                let mut position = None;
                let mut started_at = None;
                let mut ended_at = None;

                while let Some(key) = map.next_key()? { 
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(Error::duplicate_field("id"));
                            }

                            id = Some(map.next_value()?);
                        }
                        Field::Name => {
                            if name.is_some() {
                                return Err(Error::duplicate_field("name"));
                            }

                            name = Some(map.next_value()?);
                        },
                        Field::Description => {
                            if description.is_some() {
                                return Err(Error::duplicate_field("description"));
                            }

                            description = Some(map.next_value()?);
                        },
                        Field::Duration => {
                            if duration.is_some() {
                                return Err(Error::duplicate_field("duration"));
                            }

                            duration = Some(map.next_value()?);
                        },
                        Field::Progress => {
                            if progress.is_some() {
                                return Err(Error::duplicate_field("progress"));
                            }

                            progress = Some(map.next_value()?);
                        },
                        Field::Priority => {
                            if priority.is_some() {
                                return Err(Error::duplicate_field("priority"));
                            }

                            priority = Some(map.next_value()?);
                        },
                        Field::StateId => {
                            if state_id.is_some() {
                                return Err(Error::duplicate_field("state_id"));
                            }

                            state_id = Some(map.next_value()?);
                        }
                        Field::Position => {
                            if position.is_some() {
                                return Err(Error::duplicate_field("position"));
                            }

                            position = Some(map.next_value()?);
                        },
                        Field::StartedAt => {
                            if started_at.is_some() {
                                return Err(Error::duplicate_field("started_at"));
                            }

                            started_at = Some(map.next_value()?);
                        },
                        Field::EndedAt => {
                            if ended_at.is_some() {
                                return Err(Error::duplicate_field("ended_at"));
                            }

                            ended_at = Some(map.next_value()?);
                        },
                    }
                }


                let id = id.ok_or(Error::missing_field("id"))?;
                let name = name.ok_or(Error::missing_field("name"))?;
                let description = description.ok_or(Error::missing_field("description"))?;
                let duration = duration.ok_or(Error::missing_field("duration"))?;
                let progress = progress.ok_or(Error::missing_field("progress"))?;
                let priority = priority.ok_or(Error::missing_field("priority"))?;
                let state_id = state_id.ok_or(Error::missing_field("state"))?;
                let position = position.ok_or(Error::missing_field("position"))?;
                let started_at = started_at.ok_or(Error::missing_field("started_at"))?;
                let ended_at = ended_at.ok_or(Error::missing_field("ended_at"))?;

                let state = State::new(Some(state_id), "".to_string(), None,  None, 0);

                Ok(Task {
                    id, name, description, duration, progress, priority,
                    state: Rc::from(state), board: None, position, started_at, ended_at
                }) 
            }
        }
        deserializer.deserialize_struct("Task", &[
            "id",
            "name",
            "description",
            "duration",
            "progress",
            "priority",
            "state_id",
            "position",
            "started_at",
            "ended_at",
        ], TaskVisitor)
    }
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
