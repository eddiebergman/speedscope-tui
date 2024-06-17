use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

pub enum EventResponse {
    Continue,
    Consumed,
    Ignored,
    Halt,
}

pub trait EventHandler {
    fn handle_event(e: Event) -> EventResponse;
}

pub fn is_ctrlc(event: &Event) -> bool {
    if let Event::Key(key) = event {
        if key.kind == KeyEventKind::Press
            && key.code == KeyCode::Char('c')
            && key.modifiers == KeyModifiers::CONTROL
        {
            return true;
        }
    }
    false
}
