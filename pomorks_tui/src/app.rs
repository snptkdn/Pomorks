use crate::statefull_list::StatefulList;
use std::process::Command;
use std::fs;
use std::path;
use std::error::Error;
use std::env;
use std::io::BufReader;

pub struct Signal<S: Iterator> {
    source: S,
    pub points: Vec<S::Item>,
    tick_rate: usize,
}

impl<S> Signal<S>
where
    S: Iterator,
{
    fn on_tick(&mut self) {
        for _ in 0..self.tick_rate {
            self.points.remove(0);
        }
        self.points
            .extend(self.source.by_ref().take(self.tick_rate));
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub show_chart: bool,
    pub progress: f64,
    pub enhanced_graphics: bool,
    pub folders_index: usize,
    pub path_copied: String,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {

        App {
            title,
            should_quit: false,
            show_chart: false,
            progress: 0.0,
            enhanced_graphics,
            folders_index: 0,
            path_copied: "".to_string(),
        }
    }

    pub fn on_up(&mut self) {
        //self.folders[self.folders_index].previous();
    }

    pub fn on_down(&mut self) {
        //self.folders[self.folders_index].next();
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

    pub fn on_focus_left_pain(&mut self) {
        self.folders_index = 0;
    }

    pub fn on_focus_right_pain(&mut self) {
        self.folders_index = 1;
    }

    pub fn on_key(&mut self, c: char, _: (u16, u16)) {
        match c {
            'e' => {
                self.should_quit = true;
            }
            't' => {
                self.show_chart = !self.show_chart;
            }
            'j' => { self.on_down(); }
            'k' => { self.on_up(); }
            'c' => { self.on_enter_dir(); }
            'l' => { self.on_focus_right_pain(); }
            'h' => { self.on_focus_left_pain(); }
            _ => {}
        }
    }

    pub fn add_task(&mut self) {

    }

    pub fn on_tick(&mut self) {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }
    }
}
