use tracing_oslog::OsLogger;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;

pub fn ensure_subscriber() {
    // kind of inefficient to do unconditionally, oh well
    let collector = tracing_subscriber::registry()
        .with(OsLogger::new("com.praxwallet.mobile", "default").with_filter(LevelFilter::DEBUG));
    // if set this will error
    let _ = tracing::subscriber::set_global_default(collector);
}
