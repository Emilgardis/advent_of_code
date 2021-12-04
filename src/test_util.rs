// Inspired by https://github.com/ZcashFoundation/zebra/blob/2f46d698dd86e40e7928bddca948f5be14667934/zebra-test/src/lib.rs
use color_eyre::section::PanicMessage;
use owo_colors::OwoColorize;
use std::sync::Once;
use tracing_subscriber::{prelude::*, util::SubscriberInitExt};
static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        let fmt_layer = tracing_subscriber::fmt::layer().with_target(false);
        // Use the RUST_LOG env var, or by default:
        //  - warn for most tests, and
        //  - for some modules, hide expected warn logs
        let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::try_new("debug").unwrap());
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .with(tracing_error::ErrorLayer::default())
            .init();
        color_eyre::config::HookBuilder::default()
            .add_frame_filter(Box::new(|frames| {
                let mut displayed = std::collections::HashSet::new();
                let filters = &[
                    "test::run_test_in_process",
                    "core::ops::function::FnOnce::call_once",
                    "std::thread::local",
                    "<alloc::boxed::Box",
                    "<std::panic::AssertUnwindSafe",
                    "core::result::Result",
                    "<tracing_futures::Instrumented",
                    "test::assert_test_result",
                    "spandoc::",
                ];

                frames.retain(|frame| {
                    let loc = (frame.lineno, &frame.filename);
                    let inserted = displayed.insert(loc);

                    if !inserted {
                        return false;
                    }

                    !filters.iter().any(|f| {
                        let name = if let Some(name) = frame.name.as_ref() {
                            name.as_str()
                        } else {
                            return true;
                        };

                        name.starts_with(f)
                    })
                });
            }))
            .panic_message(SkipTestReturnedErrPanicMessages)
            .install()
            .unwrap();
    })
}

struct SkipTestReturnedErrPanicMessages;

impl PanicMessage for SkipTestReturnedErrPanicMessages {
    fn display(
        &self,
        pi: &std::panic::PanicInfo<'_>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let payload = pi
            .payload()
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| pi.payload().downcast_ref::<&str>().cloned())
            .unwrap_or("<non string panic payload>");

        // skip panic output that is clearly from tests that returned an `Err`
        // and assume that the test handler has already printed the value inside
        // of the `Err`.
        if payload.contains("the test returned a termination value with a non-zero status code") {
            return write!(f, "---- end of test output ----");
        }

        writeln!(f, "{}", "\nThe application panicked (crashed).".red())?;

        write!(f, "Message:  ")?;
        writeln!(f, "{}", payload.cyan())?;

        // If known, print panic location.
        write!(f, "Location: ")?;
        if let Some(loc) = pi.location() {
            write!(f, "{}", loc.file().purple())?;
            write!(f, ":")?;
            write!(f, "{}", loc.line().purple())?;
        } else {
            write!(f, "<unknown>")?;
        }

        Ok(())
    }
}
