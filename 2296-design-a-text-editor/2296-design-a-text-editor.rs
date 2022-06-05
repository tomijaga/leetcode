use std::cmp::{min, max};

struct TextEditor {
    left: Vec<char>,
    right: Vec<char>
}

impl TextEditor {

    fn new() -> Self {
        Self{
            left:vec![],
            right:vec![]
        }
    }
    
    fn add_text(&mut self, text: String) {
        for c in text.chars(){
            self.left.push(c);
        }
    }
    
    fn delete_text(&mut self, k: i32) -> i32 {
        let n = min(self.left.len() as i32, k);
        for i in 0..n{
            self.left.pop();
        }
        
        n
    }
    
    fn cursor_left(&mut self, k: i32) -> String {
        let n = min(self.left.len() as i32, k);
        
        for i in 0..n{
            let c = self.left.pop().unwrap();
            self.right.push(c);
        }
        
        let mut slice = &self.left[max(self.left.len() as i32 - 10, 0) as usize..];
        slice.iter().collect::<String>()
    }
    
    fn cursor_right(&mut self, k: i32) -> String {
        let n = min(self.right.len() as i32, k);
        
        for i in 0..n{
            let c = self.right.pop().unwrap();
            self.left.push(c);
        }
        
        self.cursor_left(0)
    }
}

/**
 * Your TextEditor object will be instantiated and called as such:
 * let obj = TextEditor::new();
 * obj.add_text(text);
 * let ret_2: i32 = obj.delete_text(k);
 * let ret_3: String = obj.cursor_left(k);
 * let ret_4: String = obj.cursor_right(k);
 */