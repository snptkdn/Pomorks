use crate::statefull_list::StatefulList;
use pomorks_data_manage::todo::{TodoItem, TodoList};

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub show_chart: bool,
    pub progress: f64,
    pub enhanced_graphics: bool,
    pub todos: StatefulList<TodoItem>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool, todo_list: &TodoList) -> App<'a> {
        App {
            title,
            should_quit: false,
            show_chart: false,
            progress: 0.0,
            todos: StatefulList::with_items(todo_list.get_vec_of_todo()),
            enhanced_graphics,
        }
    }

    pub fn on_up(&mut self) {
        self.todos.previous();
    }

    pub fn on_down(&mut self) {
        self.todos.next();
    }

    pub fn on_right(&mut self) {
        //self.tabs.next();
    }

    pub fn on_left(&mut self) {
        //self.tabs.previous();
    }

    pub fn on_enter_dir(&mut self) {
        //match self.folders[self.folders_index].state.selected() {
        //_ => {}
        //}
    }

    pub fn on_focus_left_pain(&mut self) {}

    pub fn on_focus_right_pain(&mut self) {}

    pub fn on_key(&mut self, c: char, _: (u16, u16)) {
        match c {
            'e' => {
                self.should_quit = true;
            }
            't' => {
                self.show_chart = !self.show_chart;
            }
            'j' => {
                self.on_down();
            }
            'k' => {
                self.on_up();
            }
            'c' => {
                self.on_enter_dir();
            }
            'l' => {
                self.on_focus_right_pain();
            }
            'h' => {
                self.on_focus_left_pain();
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }
    }
}
