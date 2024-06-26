//! Events that occur in readline

use crossterm::event::KeyEvent;

use crate::prelude::{HookEvent, HookEventMarker, LineMode};

/// Runs whenever the current mode of the line changes
#[derive(HookEvent)]
pub struct LineModeSwitchEvent {
    pub line_mode: LineMode,
}

// #[derive(HookEvent)]
// pub struct ReadEventStartEvent;

// #[derive(HookEvent)]
// pub struct PreRenderEvent {}

// #[derive(HookEvent)]
// pub struct PostRenderEvent {}

// #[derive(HookEvent)]
// pub struct ReadEventEndEvent;

/// Alternative for keybinding
///
/// It is recommended that keybinding is used instead
/// if the hook is responding to a specific keypress
#[derive(HookEvent)]
pub struct OnKeyEvent {
    key: KeyEvent,
}
