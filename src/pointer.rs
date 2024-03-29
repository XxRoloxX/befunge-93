use crate::funge_space::FungeSpace;
use crate::instructions::Executable;
use crate::instructions::{PutCharInstruction, SwitchStringModeInstruction};
use crate::interpreter::Interpreter;
use crate::interpreter::ReadMode;
use crate::symbol_mapper::map_symbol_to_instruction as mapper;
use std::sync::Arc;

#[derive(Debug)]
pub struct Pointer {
    x: i32,
    y: i32,
    direction: Direction,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Pointer {
    pub fn new() -> Pointer {
        Pointer {
            x: 0,
            y: 0,
            direction: Direction::Right,
        }
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
    pub fn get_current_symbol(&self, interpreter: &Interpreter) -> char {
        interpreter
            .get_immutable_space()
            .get_symbol_at(self.x as usize, self.y as usize)
    }

    pub fn move_vertically(&mut self, steps: i32) {
        self.y = self.y + steps;
    }
    pub fn move_horizontally(&mut self, steps: i32) {
        self.x = self.x + steps;
    }
    pub fn wrap_pointer(&mut self, space: &FungeSpace) {
        self.y = self.y.rem_euclid(space.height as i32);
        self.x = self.x.rem_euclid(space.width as i32); 
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn current_move(&mut self) {
        match self.direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    pub fn move_up(&mut self) {
        self.move_vertically(-1);
    }

    pub fn move_down(&mut self) {
        self.move_vertically(1);
    }

    pub fn move_left(&mut self) {
        self.move_horizontally(-1);
    }

    pub fn move_right(&mut self) {
        self.move_horizontally(1);
    }
    pub fn get_current_instruction(
        &self,
        interpreter: &Interpreter,
    ) -> Option<Arc<dyn Executable>> {
        // self.wrap_pointer(&interpreter.get_space());

        let current_symbol = interpreter
            .get_immutable_space()
            .get_symbol_at(self.x as usize, self.y as usize);

        match interpreter.get_mode() {
            ReadMode::String if (current_symbol == '"') => {
                Some(Arc::from(SwitchStringModeInstruction {}))
            }
            ReadMode::String => Some(Arc::from(PutCharInstruction(current_symbol))),
            ReadMode::Normal => mapper(current_symbol),
        }
    }
}
