mod controllers;
mod models;
mod views;

use controllers::anime::Controller;
use views::console::ConsoleView;

fn main() {
    let controller = Controller::new(ConsoleView);
    controller.run();
    
}
