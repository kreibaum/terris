//! Version of the "Shoe Game" that is often played at a wedding.
//! The Bride and Groom each get one shoe of their partner. They then get asked
//! questions about their relationship and must answer with the correct shoe.
//!
//! Example: "Who empties the trash more often?"
//!
//! This extension of the Shoe Game adds a way for all the participants to also
//! vote on the answer. The votes and the answer are then displayed at the end.
//!
//! This means there are several different user roles for this application:
//! * Guest: See the question and pick a virtual shoe.
//! * Presentation Screen: See questions and stats.
//! * Moderator: Controll question flow and presentation.
//!
//! Rules are added in advance and can not be changed through the app.

#[macro_use]
extern crate diesel;
extern crate simplelog;

mod models;
mod schema;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use log::debug;
use terris::websocket_actor::websocket_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    let results = load_all_questions();

    // Instead of routing, I want this server to have a single state that is
    // shared by all connections.

    HttpServer::new(|| App::new().route("/ws", web::get().to(websocket_handler)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn load_all_questions() -> Vec<models::Question> {
    use crate::schema::questions::dsl::*;
    debug!("Opening database.");
    let connection = SqliteConnection::establish("questions.db").unwrap();
    let results = questions
        .order(sort_order.asc())
        .load::<models::Question>(&connection)
        .expect("Error loading questions");
    debug!("Loaded {} questions:", results.len());
    for q in &results {
        debug!("{}: {}", q.sort_order, q.question);
    }
    results
}

// Set up logging //
////////////////////

fn init_logger() {
    use simplelog::*;

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        // WriteLogger::new(
        //     LevelFilter::Info,
        //     Config::default(),
        //     std::fs::File::create("server.log").unwrap(),
        // ),
    ])
    .unwrap();

    debug!("Logger successfully initialized");
}
