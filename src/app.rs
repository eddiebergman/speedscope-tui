use std::collections::HashSet;

use ratatui::widgets::ScrollbarState;

use crate::speedscope::{Data, Frame, Profile, Stack};

const DEFAULT_FILE_FILTERS: [&str; 3] = [
    "<frozen importlib._bootstrap_external>",
    "<frozen importlib._bootstrap>",
    "<string>",
];

#[derive(Debug)]
pub struct DataView {
    pub profile: Profile,
    pub active_stack_index: usize,
    pub active_frame_index: usize,
    pub scrollbar_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub expand_code: bool,
}

impl DataView {
    pub fn active_stack(&self) -> &Stack {
        self.profile
            .stacks
            .get(self.active_stack_index)
            .expect("Active stack not found")
    }

    pub fn active_frame(&self) -> &Frame {
        self.active_stack()
            .frames
            .get(self.active_frame_index)
            .expect("Active frame not found")
    }

    pub fn prev_stack(&mut self) {
        if self.active_stack_index == 0 {
            return;
        }
        self.active_stack_index -= 1;
        self.vertical_scroll -= 1;
        self.scrollbar_state = self.scrollbar_state.position(self.vertical_scroll);

        // Ensure the active frame index is within bounds
        let active_stack = self.active_stack();
        if self.active_frame_index >= active_stack.len() {
            self.active_frame_index = active_stack.len() - 1;
        }
    }

    pub fn next_stack(&mut self) {
        let profile_len = self.profile.len();
        if self.active_stack_index >= profile_len - 1 {
            return;
        }

        self.active_stack_index += 1;
        self.vertical_scroll += 1;
        self.scrollbar_state = self.scrollbar_state.position(self.vertical_scroll);

        // Ensure the active frame index is within bounds
        let active_stack = self.active_stack();
        if self.active_frame_index >= active_stack.len() {
            self.active_frame_index = active_stack.len() - 1;
        }
    }

    pub fn prev_frame(&mut self) {
        if self.active_frame_index > 0 {
            self.active_frame_index -= 1;
        }
    }

    pub fn next_frame(&mut self) {
        let active_stack_len = self.active_stack().len();
        if self.active_frame_index < active_stack_len - 1 {
            self.active_frame_index += 1;
        }
    }

    pub fn toggle_expand(&mut self) {
        self.expand_code = !self.expand_code;
    }
}

#[derive(Debug)]
pub struct App {
    pub data: Data,
    pub view: DataView,
    pub file_filters: HashSet<String>,
}

impl App {
    pub fn new(data: Data) -> Self {
        let first_profile = data
            .profiles
            .values()
            .next()
            .expect("No profiles found")
            .to_owned();
        let file_filters: HashSet<String> =
            DEFAULT_FILE_FILTERS.iter().map(|s| s.to_string()).collect();

        let view = DataView {
            profile: first_profile.with_filter(&file_filters),
            scrollbar_state: ScrollbarState::default(),
            vertical_scroll: 0,
            active_stack_index: 0,
            active_frame_index: 0,
            expand_code: false,
        };
        App {
            data,
            view,
            file_filters,
        }
    }
}
