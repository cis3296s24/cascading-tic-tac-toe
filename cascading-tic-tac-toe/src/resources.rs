// @see https://bevy-cheatbook.github.io/programming/res.html

use bevy::prelude::*;

#[derive(Resource)]
pub struct RoundInit {
    pub round_count: u32,
    pub target: u32,
    // pub time: u32,
}

impl RoundInit {
    pub fn new(init_round_count: u32, init_target: u32) -> Self {
        RoundInit { round_count: init_round_count, target: init_target }
    }
}

// #[derive(Resource)]
// pub struct RoundCount(u32);

// impl RoundCount {
//     pub fn new(initial_value: u32) -> Self {
//         RoundCount(initial_value)
//     }
//     pub fn get_current(&self) -> u32 {
//         self.0
//     }
// }