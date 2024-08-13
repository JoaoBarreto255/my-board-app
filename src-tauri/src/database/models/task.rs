
use core::option::Option;

use crate::database::models::custom_types::{Frequence, Priority};

#[derive(Debug)]
pub struct Task {
    id: u64,
    name: String,
    description: Option<String>,
    frequence: Frequence,
    duration: u32,
    progress: u32,
    priority: Priority,
    state: String,
    profile: u32,
    started_at: Option<String>,
    ended_at: Option<String>
}

impl Task {
    fn new(
        id: u64, name: String, description: Option<String>
        , frequence: Frequence, duration: u32, progress: u32
        , priority: Priority, state: String, profile: u32
    ) -> Task {
        return Task {
            id, name, description: description, frequence
            , duration, progress, priority, state, profile
            , started_at: None, ended_at: None
        }
    }
}


