#[macro_use]
extern crate log;
extern crate chrono;
extern crate config;
extern crate fern;
extern crate zip;

mod app;
mod musicxml;

fn main() {
    app::logger::setup().expect("Could not set up logger.");
    app::config::load_settings();
}
