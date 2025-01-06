use crate::prelude::*;

pub type Dialogue = Arc<[Box<str>]>;

struct DialogueState {
    dialogue: Dialogue,
    index: usize,
    eid: usize,
}

impl DialogueState {
    fn new(dialogue: Dialogue, eid: usize) -> Self {
        DialogueState { dialogue, index: 0, eid }
    }

    pub fn read(&self) -> Option<&str> {
        if self.index >= self.dialogue.len() {
            None
        } else {
            Some(&self.dialogue[self.index])
        }
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }
}

pub struct DialogueManager {
    state: Option<DialogueState>,
}

impl DialogueManager {
    pub fn new() -> Self {
        DialogueManager { state: None }
    }

    /// Loads an npc's dialogue if it isn't already loaded, but does not draw it.
    /// Use [`DialogueManager::handle_dialogue`] to draw.
    pub fn load_dialogue(&mut self, npc: &NPC) {
        match self.state.as_mut() {
            Some(state) if state.eid == npc.id() => (),
            _ => self.state = Some(DialogueState::new(
                Arc::clone(&npc.dialogue),
                npc.id()
            ))
        }
    }

    pub fn handle_dialogue(&mut self) {
        if let Some(state) = self.state.as_mut() {
            if is_key_pressed(KeyCode::Enter) {
                state.advance()
            } 
            if let Some(dpart) = state.read() {
                draw_text(dpart, 0.0, 600.0, 16.0, BLACK);
            } else {
                self.state = None;
            }
        }
    }

    pub fn has_loaded_dialogue(&self) -> bool {
        self.state.is_some()
    }

    pub fn has_active_dialogue(&self) -> bool {
        self.state.as_ref().is_some_and(|s| s.index > 0)
    }
}