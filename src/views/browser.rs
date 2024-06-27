extern crate web_view;
use web_view::*;

use crate::models::PlayersModel;

pub fn display(name: &str, players: &Vec<PlayersModel>) {
    println!("{}: {}", name, players[1].data);
    web_view::builder()
        .title(format!("Doiga: {}", name).as_str())
        .content(Content::Url(players[2].data.as_str()))
        .size(800, 600)
        .resizable(true)
        .debug(false)
        .user_data(())
        .invoke_handler(|_webview, _arg| {
            // Inyectar JavaScript para eliminar anuncios y contenido innecesario
            Ok(())
        })        .run()
        .unwrap();
}
