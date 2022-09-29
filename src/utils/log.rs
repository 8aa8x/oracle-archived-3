use slog::{o, Drain, Logger};
use slog_async::Async;
use slog_term::{CompactFormat, TermDecorator};

pub fn create_logger<T: ToString>(id: &T) -> Logger {
    // Create a new drain hierarchy that writes to the terminal
    let decorator = TermDecorator::new().build();
    let drain = CompactFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();

    // Return the base logger, which can be cloned to add context
    slog::Logger::root(drain, o!("id" => id.to_string()))
}
