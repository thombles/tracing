//! [`Layer`]s that control which spans and events are enabled by the wrapped
//! subscriber.
//!
//! [`Layer`]: ../trait.Layer.html
#[cfg(feature = "env-filter")]
mod env;
mod level;

pub use self::level::{LevelFilter, ParseError as LevelParseError};

#[cfg(feature = "env-filter")]
pub use self::env::*;

use std::sync::Arc;

/// A `Layer` which filters spans and events based on a set of filter
/// directives.
///
/// **Note**: renamed to `EnvFilter` in 0.1.2; use that instead.
#[cfg(feature = "env-filter")]
#[deprecated(since = "0.1.2", note = "renamed to `EnvFilter`")]
pub type Filter = EnvFilter;

#[derive(Debug, Clone)]
pub(crate) struct TargetFilter(Option<Arc<str>>);

impl TargetFilter {
    pub(crate) fn matches(&self, target: &str) -> bool {
        if let Some(filter) = &self.0 {
            target.starts_with(&filter[..])
        } else {
            true
        }
    }
}

impl From<Option<Arc<str>>> for TargetFilter {
    fn from(t: Option<Arc<str>>) -> TargetFilter {
        TargetFilter(t)
    }
}
