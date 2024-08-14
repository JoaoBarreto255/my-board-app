use core::option::Option;
use std::rc::{Rc, Weak};
use std::fmt::Debug;

use crate::database::models::Board;

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
