#![allow(clippy::redundant_closure_call)]
#![allow(clippy::await_holding_refcell_ref)]
#![allow(clippy::drop_non_drop)]
#![allow(non_snake_case)]

pub mod util;

pub mod app;
pub mod page;
mod elements;

use cfg_if::cfg_if;

pub const ROOT_API_URL: &str = "http://127.0.0.1:8070/";

cfg_if! {
    if #[cfg(feature = "console_log")] {
        pub fn init_log() {
            use log::Level;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        pub fn init_log() {}
    }
}

fn main() {
    init_log();
    dioxus_web::launch(app::App)
}

mod prelude {
    pub use crate::page;
    pub use crate::util::{async_handler, sync_handler, maybe_class};
}
