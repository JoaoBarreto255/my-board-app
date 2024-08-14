use core::option::Option;
use std::rc::Rc;
use std::fmt::Debug;

use crate::database::models::Board;

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

