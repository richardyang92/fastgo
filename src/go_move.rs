#[derive(Debug, Clone)]
pub struct GoMove {
    move_id: usize,
    move_pos: (usize, usize, i8),
    eat_stones: Vec<(usize, usize, i8)>,
}

impl GoMove {
    pub fn new(move_id: usize, pos_x: usize, pos_y: usize, cur_state: i8) -> Self {
        GoMove { move_id, move_pos: (pos_x, pos_y, cur_state), eat_stones: vec![] }
    }

    pub fn new_with_eaten_stones(move_id: usize, pos_x: usize, pos_y: usize,
        cur_state: i8, eat_stones: Vec<(usize, usize, i8)>) -> Self {
        GoMove { move_id, move_pos: (pos_x, pos_y, cur_state), eat_stones }
    }

    pub fn set_eaten_stones(&mut self, eaten_stones: Vec<(usize, usize, i8)>) {
        self.eat_stones = eaten_stones;
    }

    pub fn eaten_stones(&self) -> Vec<(usize, usize, i8)> {
        self.eat_stones.clone()
    }

    pub fn move_pos(&self) -> (usize, usize, i8) {
        self.move_pos
    }

    pub fn move_id(&self) -> usize {
        self.move_id
    }
}