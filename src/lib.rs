mod config;
mod page;
pub use config::InertiaConfig;
pub(crate) use page::PageObject;
mod renderer;
pub use renderer::*;