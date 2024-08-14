
use core::option::Option;
use std::rc::{Rc, Weak};
use std::fmt::Debug;

use crate::database::models::custom_types::Priority;
use crate::database::models::group::{State, Board};

pub enum DurationInput {
    Minutes(u32),
    HoursAndMinutes(u32, u32),
}

#[derive(Debug)]
pub struct Task {
    id: Option<u64>,
    name: Rc<String>,
    description: Option<Rc<String>>,
    duration: u32,
    progress: Option<f32>,
    priority: Priority,
    state: Rc<State>,
    board: Weak<Board>,
    position: u32,
    started_at: Option<Rc<String>>,
    ended_at: Option<Rc<String>>
}

impl Task {
    pub fn new(
        id: Option<u64>, name: Rc<String>, description: Option<Rc<String>>
        , duration: u32, priority: Priority, state: Rc<State>
        , board: Weak<Board>, position: u32
    ) -> Task {
        return Task {
            id, name, description: description, duration
            , progress: None, priority, state, board
            , position, started_at: None, ended_at: None
        }
    }

    /// get id from [`Task`]
    pub fn get_id(self) -> Option<u64> {
        self.id
    }

    /// Sets the id of this [`Task`].
    pub fn set_id(&mut self, id: Option<u64>) -> &mut Task {
        self.id = id;

        return self;
    }

    /// get name from [`Task`]
    pub fn get_name(self) -> Rc<String> {
        self.name.clone()
    }

    /// Sets the name of this [`Task`].
    pub fn set_name(&mut self, name: String) -> &mut Task {
        self.name = Rc::new(name);

        return self;
    }

    /// get description from [`Task`]
    pub fn get_description(self) -> Option<Rc<String>> {
        return match &self.description {
            Some(description) => Some(description.clone()),
            _ => None,
        };
    }

    /// Sets the descriptio of this [`Task`].
    pub fn set_description(&mut self, description: String) -> &mut Task {
        self.description = Some(Rc::new(description));

        return self;
    }

    /// get duration from [`Task`] and returns 
    /// tuple with duration splited in (hours, minutes).
    pub fn get_duration(self) -> DurationInput {
        let time_in_hours = self.duration / 60;
        let time_in_minutes = self.duration % 60;

        if time_in_hours == 0 {
            return DurationInput::Minutes(time_in_minutes);
        }

        return DurationInput::HoursAndMinutes(time_in_hours, time_in_minutes);
    }

    /// Sets the duration of this [`Task`].
    pub fn set_duration(&mut self, duration: DurationInput) -> &mut Task {
        self.duration = match duration {
            DurationInput::Minutes(m) => m,
            DurationInput::HoursAndMinutes(h, m) => h * 60 + m
        };

        return self;
    }

    /// obtains current [`Task`] progress.
    pub fn get_progress(self) -> Option<f32> {
        self.progress
    }

    /// update current [`Task`] progress.
    pub fn set_progress(&mut self, progress: Option<f32>) -> &mut Task {
        self.progress = progress;

        return self;
    }

    /// obtains current [`Task`] priority.
    pub fn get_priority(self) -> Priority {
        return self.priority;
    }

    /// update current [`Task`] priority.
    pub fn set_priority(&mut self, priority: Priority) -> &mut Task {
        self.priority = priority;

        return self;
    }

    /// obtains current [`Task`] state.
    pub fn get_state(self) -> Rc<State> {
        return self.state.clone();
    }

    /// update current [`Task`] state.
    pub fn set_state(&mut self, state: Rc<State>) -> &mut Task {
        self.state = state;

        return self;
    }

    /// obtains current [`Task`] task_group.
    pub fn get_board(self) -> Weak<Board> {
        self.board.clone()
    }

    /// update current [`Task`] task_group.
    pub fn set_board(&mut self, board: Weak<Board>) -> &mut Task {
        self.board = board;

        return self;
    }

    /// get [`Task`] postion on state
    pub fn get_position(self) -> u32 {
        self.position
    }

    /// Sets new [`Task`] position.
    pub fn set_position(&mut self, position: u32) -> &mut Task {
        self.position = position;

        return self;
    }

    /// obtains current [`Task`] started_at.
    pub fn get_started_at(self) -> Option<Rc<String>> {
        return match &self.started_at {
            Some(date) => Some(date.clone()),
            None => None,
        };
    }

    /// update current [`Task`] started_at.
    /// if current task already updated it, just ignore 
    pub fn set_started_ed(&mut self, started_et: String) -> &mut Task {
        if let Some(_) = &self.started_at {
            return self;
        }
        self.started_at = Some(Rc::new(started_et));

        return self;
    }

    /// obtains current [`Task`] started_at.
    pub fn get_ended_at(self) -> Option<Rc<String>> {
        return match &self.ended_at {
            Some(date) => Some(date.clone()),
            None => None,
        };
    }

    /// update current [`Task`] ended_at.
    /// if current task already updated it, just ignore. 
    pub fn set_ended_ed(&mut self, ended_at: String) -> &mut Task {
        if let Some(_) = &self.ended_at {
            return self;
        }
        self.ended_at = Some(Rc::new(ended_at));

        return self;
    }
}

