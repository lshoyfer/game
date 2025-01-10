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

    // Seperating into `is_readable` and `read`
    // plays nicer with the borrow checker
    #[inline]
    pub fn is_readable(&self) -> bool {
        self.index < self.dialogue.len()
    }

    /// Safety
    /// Check if is_readable before reading
    /// due to direct array access
    pub fn read(&self) -> &str {
        &self.dialogue[self.index]
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

    /// Returns dialogue str to be played if there is dialogue to play
    pub fn handle_dialogue(&mut self) -> Option<&str> {
        if let Some(mut_state) = self.state.as_mut() {
            if is_key_pressed(KeyCode::Enter) {
                mut_state.advance();
            }
            if mut_state.is_readable() {
                return self.state.as_ref().map(DialogueState::read)
            }
        }
        // If here, either was already None or unreadable, so consume state to end dialogue
        self.state = None;
        None
    }


    pub fn has_loaded_dialogue(&self) -> bool {
        self.state.is_some()
    }

    pub fn has_active_dialogue(&self) -> bool {
        self.state.as_ref().is_some_and(|s| s.index > 0)
    }
}
