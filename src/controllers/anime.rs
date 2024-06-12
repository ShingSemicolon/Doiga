use crate::{models::anime::AnimeModel, views::trait_view::View};

pub struct Controller<T: View> {
    view: T,
}

impl<T: View> Controller<T> {
    pub fn new(view: T) -> Self {
        Self { view }
    }

    pub fn run(&self) {
        self.view.clear_screen();

    }


}
