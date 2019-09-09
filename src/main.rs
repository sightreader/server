#[macro_use]
extern crate log;
extern crate chrono;
extern crate config;
extern crate fern;

mod app;

fn main() {
    app::logger::setup().expect("Could not set up logger.");
    app::config::load_settings();
}
