use core::option::Option;
use std::rc::{Rc, Weak};
use std::fmt::Debug;

use crate::database::models::task::Task;

#[derive(Debug)]
pub struct State {
    id: Option<u32>,
    name: Rc<String>,
    color: Option<Rc<String>>,
    position: u32,
    board: Weak<Board>
}

impl State {
    pub fn new(id: Option<u32>, name: Rc<String>, color: Option<Rc<String>>, board: Weak<Board>, position: u32) -> State {
        State { id, name, color, position, board }
    }
    
    /// Sets the id of this [`State`].
    pub fn set_id(&mut self, id: Option<u32>) -> &mut State {
        self.id = id;

        return self;
    }

    pub fn get_id(self) -> Option<u32> {
        self.id
    }

    pub fn get_name(&self) -> Rc<String> {
        self.name.clone()
    }

    
    /// Sets the name of this [`State`].
    pub fn set_name(&mut self, name: Rc<String>) -> &mut State {
        self.name = name;

        return self;
    }

    pub fn get_color(&self) -> Option<Rc<String>> {
        return match &self.color {
            Some(val) => Some(val.clone()),
            None => None,
        };
    }

    /// Sets the color of this [`State`].
    pub fn set_color(&mut self, color: Option<Rc<String>>) -> &mut State {
        self.color = color;

        return self;
    }

    pub fn get_position(&self) -> u32 {
        self.position
    }

    /// Sets new [`State`] position.
    pub fn set_position(&mut self, position: u32) -> &mut State {
        self.position = position;

        return self;
    }

    pub fn get_board(&self) -> Weak<Board> {
        self.board.clone()
    }

    /// Sets the board of this [`State`].
    pub fn set_board(&mut self, board: &Weak<Board>) -> &mut State {
        self.board = board.clone();

        return self;
    }
}


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

#[derive(Debug)]
pub struct Group {
    id: Option<u32>,
    name: Rc<String>,
    icon: Option<String>,
    position: u32,
    boards: Vec<Rc<Board>>
}

impl Group {
    pub fn new(id: Option<u32>, name: Rc<String>, icon: Option<String>, position: u32) -> Group {
        Group { id, name, icon, boards: vec![], position }
    }

    pub fn get_id(&self) -> Option<u32> {
        self.id
    }

    /// Sets the id of this [`Group`].
    pub fn set_id(&mut self, id: Option<u32>) -> &mut Group {
        self.id = id;

        return self;
    }

    pub fn get_name(&self) -> Rc<String> {
        self.name.clone()
    }

    /// Sets the name of this [`Group`].
    pub fn set_name(&mut self, name: Rc<String>) -> &mut Group {
        self.name = name;

        return self;
    }

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

    /// get [`Group`] postion on state
    pub fn get_position(self) -> u32 {
        self.position
    }

    /// Sets new [`Group`] position.
    pub fn set_position(&mut self, position: u32) -> &mut Group {
        self.position = position;

        return self;
    }

    pub fn get_boards(&self) -> &Vec<Rc<Board>> {
        &self.boards
    }

    /// Sets the boards of this [`Group`].
    pub fn set_boards(&mut self, boards: Vec<Rc<Board>>) -> &mut Group {
        self.boards = boards;

        return self;
    }

    pub fn add_board(&mut self, board: Rc<Board>) -> &mut Group {
        self.boards.push(board);
        
        return self;
    }
}

