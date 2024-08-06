use leptos::{mount_to_body, view};

mod app;
mod components;
mod pages;

fn main() {
    mount_to_body(|| app::App())
}
