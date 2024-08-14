use core::option::Option;
use std::rc::{Rc, Weak};
use std::fmt::Debug;

use crate::database::models::Task;
use crate::database::models::State;
use crate::database::models::Group;


#[derive(Debug)]
pub struct Board {
    id: Option<u32>,
    name: Rc<String>,
    states: Vec<Rc<State>>,
    tasks: Vec<Rc<Task>>,
    position: u32,
    group: Weak<Group>,
}

impl Board {
    pub fn new(id: Option<u32>, name: Rc<String>, group: Weak<Group>, position: u32) -> Board {
        Board{ id, name, states: vec![], tasks: vec![], position, group }
    }

    pub fn get_id(&self) -> Option<u32> {
        self.id
    }

    /// Sets the id of this [`Board`].
    pub fn set_id(&mut self, id: Option<u32>) -> &mut Board {
        self.id = id;

        return self;
    }

    pub fn get_name(&self) -> Rc<String> {
        self.name.clone()
    }

    /// Sets the name of this [`Board`].
    pub fn set_name(&mut self, name: Rc<String>) -> &mut Board {
        self.name = name;

        return self
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

    pub fn get_group(&self) -> Weak<Group> {
        self.group.clone()
    }

    /// Sets the group of this [`Board`].
    pub fn set_group(&mut self, group: Weak<Group>) -> &mut Board {
        self.group = group;

        return self;
    }
}
