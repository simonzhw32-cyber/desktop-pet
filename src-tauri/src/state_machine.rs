pub struct PetStateMachine {
    current_state: String,
    frame_index: usize,
}

impl PetStateMachine {
    pub fn new() -> Self {
        Self {
            current_state: "idle".to_string(),
            frame_index: 0,
        }
    }

    pub fn switch_state(&mut self, new_state: String) {
        self.current_state = new_state;
        self.frame_index = 0;
    }

    pub fn current_state(&self) -> &str {
        &self.current_state
    }

    #[allow(dead_code)]
    pub fn frame_index(&self) -> usize {
        self.frame_index
    }

    pub fn advance_frame(&mut self, total_frames: usize) {
        self.frame_index += 1;
        if self.frame_index >= total_frames {
            self.frame_index = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_default_state() {
        let sm = PetStateMachine::new();
        assert_eq!(sm.current_state(), "idle");
        assert_eq!(sm.frame_index(), 0);
    }

    #[test]
    fn test_switch_state() {
        let mut sm = PetStateMachine::new();
        sm.switch_state("happy".into());
        assert_eq!(sm.current_state(), "happy");
        assert_eq!(sm.frame_index(), 0); // frame resets on switch
    }

    #[test]
    fn test_advance_frame() {
        let mut sm = PetStateMachine::new();
        sm.advance_frame(5);
        assert_eq!(sm.frame_index(), 1);
        sm.advance_frame(5);
        assert_eq!(sm.frame_index(), 2);
    }

    #[test]
    fn test_advance_frame_loops() {
        let mut sm = PetStateMachine::new();
        for _ in 0..4 {
            sm.advance_frame(4);
        }
        assert_eq!(sm.frame_index(), 0); // should loop back
    }
}