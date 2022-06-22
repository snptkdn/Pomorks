mod app;
mod statefull_list;
mod tui;
mod ui;
fn main() {
    match tui::launch_tui() {
        Ok(()) => (),
        Err(e) => println!("{}", e),
    }
}
