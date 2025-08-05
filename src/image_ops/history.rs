use crate::image_ops::ImageData;
use std::collections::VecDeque;

pub struct ImageHistory {
    history: VecDeque<ImageData>,
    current_index: usize,
    max_history: usize,
}

impl ImageHistory {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            current_index: 0,
            max_history: 50, // Keep last 50 states
        }
    }
    
    pub fn push_state(&mut self, image_data: ImageData) {
        // Remove any states after current index (when we're not at the end)
        while self.history.len() > self.current_index {
            self.history.pop_back();
        }
        
        // Add new state
        self.history.push_back(image_data);
        self.current_index = self.history.len();
        
        // Limit history size
        while self.history.len() > self.max_history {
            self.history.pop_front();
            if self.current_index > 0 {
                self.current_index -= 1;
            }
        }
    }
    
    pub fn can_undo(&self) -> bool {
        self.current_index > 1
    }
    
    pub fn can_redo(&self) -> bool {
        self.current_index < self.history.len()
    }
    
    pub fn undo(&mut self) -> Option<&ImageData> {
        if self.can_undo() {
            self.current_index -= 1;
            self.history.get(self.current_index - 1)
        } else {
            None
        }
    }
    
    pub fn redo(&mut self) -> Option<&ImageData> {
        if self.can_redo() {
            self.current_index += 1;
            self.history.get(self.current_index - 1)
        } else {
            None
        }
    }
    
    pub fn clear(&mut self) {
        self.history.clear();
        self.current_index = 0;
    }
}
