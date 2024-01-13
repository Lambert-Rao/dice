use lazy_static::lazy_static;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::components::Component;
use crate::components::home::Home;
use crate::components::fps::FpsCounter;
use crate::components::game::Dialog;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Home,
    Game,
}

pub struct ModeInfo {
    pub components: Box<dyn Component>,
}

impl Mode {
    fn new() -> Self {
        Self::Home
    }
}

lazy_static! {
    pub static ref MODE_COMPONENTS: HashMap<Mode, Vec<Box<dyn Component>>> = {
        let mut map = HashMap::new();
        let homevec:Vec<Box<dyn Component>> = vec![Box::new(Home::new())];
        map.insert(Mode::Home, homevec);
        let gamevec:Vec<Box<dyn Component>> = vec![Box::new(Dialog::new(),FpsCounter::new())];
        map.insert(Mode::Game,gamevec);
        map
    };
}

