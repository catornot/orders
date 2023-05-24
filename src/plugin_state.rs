use std::{collections::HashMap, num::NonZeroU32};

#[derive(Debug)]
pub struct PluginState {
    pub delay: NonZeroU32,
    pub chance: NonZeroU32,
    pub endtime: NonZeroU32,
    pub nexttime: NonZeroU32,
    pub active: bool,
    pub player_message_handles: HashMap<String, MessageHandle>,
    pub player_score: HashMap<String, u64>,
}

impl Default for PluginState {
    fn default() -> Self {
        Self {
            delay: NonZeroU32::new(300).unwrap(),
            chance: NonZeroU32::new(100).unwrap(),
            endtime: NonZeroU32::new(1).unwrap(),
            nexttime: NonZeroU32::new(300).unwrap(),
            active: false,
            player_message_handles: HashMap::new(),
            player_score: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct MessageHandle {
    pub first: String,
    pub second: String,
    pub third: String,
}
