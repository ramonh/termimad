use crossterm::{
    self,
    event::{
        KeyCode, KeyModifiers,
    },
};

/// a valid user event
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {

    Key(crossterm::event::KeyEvent),

    Click(u16, u16, KeyModifiers),

    RightClick(u16, u16, KeyModifiers),

    DoubleClick(u16, u16),

    /// terminal was resized. Contains the new dimensions
    Resize(u16, u16),

    /// mouse wheel turns. contains -1 if up or 1 if down
    Wheel(i32),
}

impl Event {
    /// convert a crossterm event into a termimad one.
    ///
    /// normalize \r and \n into Enter (useful for key combinations)
    ///
    /// To get a double-click you'll either need to use a termimad event-source
    /// or to do the computation yourself.
    pub fn from_crossterm_event(
        crossterm_event: crossterm::Result<crossterm::event::Event>
    ) -> Option<Event> {
        match crossterm_event {
            Ok(crossterm::event::Event::Key(mut key)) => {
                if key.code==KeyCode::Char('\r') || key.code==KeyCode::Char('\n') {
                    key.code = KeyCode::Enter;
                }
                Some(Event::Key(key))
            }
            Ok(crossterm::event::Event::Mouse(crossterm::event::MouseEvent::Up(button, x, y, modifiers))) => {
                use crossterm::event::MouseButton::*;
                match button {
                    Left => Some(Event::Click(x, y, modifiers)),
                    Right => Some(Event::RightClick(x, y, modifiers)),
                    _ => None
                }
            }
            Ok(crossterm::event::Event::Resize(w, h)) => {
                Some(Event::Resize(w, h))
            }
            Ok(crossterm::event::Event::Mouse(crossterm::event::MouseEvent::ScrollUp(..))) => {
                Some(Event::Wheel(-1))
            }
            Ok(crossterm::event::Event::Mouse(crossterm::event::MouseEvent::ScrollDown(..))) => {
                Some(Event::Wheel(1))
            }
            _ => None,
        }
    }
    pub const fn crtl_key(code: KeyCode) -> Self {
        Event::Key(
            crossterm::event::KeyEvent {
                code,
                modifiers: KeyModifiers::CONTROL,
            }
        )
    }
    pub const fn simple_key(code: KeyCode) -> Self {
        Event::Key(
            crossterm::event::KeyEvent {
                code,
                modifiers: KeyModifiers::empty(),
            }
        )
    }
    /// In case the event is mouse related, give the position
    pub fn mouse_pos(self) -> Option<(u16, u16)> {
        match self {
            Event::Click(x, y, _) => Some((x, y)),
            Event::RightClick(x, y, _) => Some((x, y)),
            Event::DoubleClick(x, y) => Some((x, y)),
            _ => None,
        }
    }
}


