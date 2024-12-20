use crate::models::AnimeModel;
pub trait View {
    fn display_message(&self, message: &str);
    fn display_error(&self, message: &str);
    fn get_user_input(&self, message: &str) -> String;
    fn clear_screen(&self);
    fn display_animes(&self, animes: &Vec<AnimeModel>);
}