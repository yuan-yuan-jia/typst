//! Scopes containing function parsers.

use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

use crate::func::ParseFunc;
use super::parsing::CallParser;
use super::Model;

/// A map from identifiers to function parsers.
pub struct Scope {
    parsers: HashMap<String, Box<CallParser>>,
    fallback: Box<CallParser>,
}

impl Scope {
    /// Create a new empty scope with a fallback parser that is invoked when no
    /// match is found.
    pub fn new<F>() -> Scope
    where F: ParseFunc<Meta=()> + Model + 'static {
        Scope {
            parsers: HashMap::new(),
            fallback: make_parser::<F>(()),
        }
    }

    /// Create a new scope with the standard functions contained.
    pub fn with_std() -> Scope {
        crate::library::std()
    }

    /// Associate the given name with a type that is parseable into a function.
    pub fn add<F>(&mut self, name: &str)
    where F: ParseFunc<Meta=()> + Model + 'static {
        self.add_with_meta::<F>(name, ());
    }

    /// Add a parseable type with additional metadata  that is given to the
    /// parser (other than the default of `()`).
    pub fn add_with_meta<F>(&mut self, name: &str, metadata: <F as ParseFunc>::Meta)
    where F: ParseFunc + Model + 'static {
        self.parsers.insert(
            name.to_string(),
            make_parser::<F>(metadata),
        );
    }

    /// Return the parser with the given name if there is one.
    pub fn get_parser(&self, name: &str) -> Option<&CallParser> {
        self.parsers.get(name).map(AsRef::as_ref)
    }

    /// Return the fallback parser.
    pub fn get_fallback_parser(&self) -> &CallParser {
        &*self.fallback
    }
}

impl Debug for Scope {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_set()
            .entries(self.parsers.keys())
            .finish()
    }
}

fn make_parser<F>(metadata: <F as ParseFunc>::Meta) -> Box<CallParser>
where F: ParseFunc + Model + 'static {
    Box::new(move |f, s| {
        F::parse(f, s, metadata.clone())
            .map(|model| Box::new(model) as Box<dyn Model>)
    })
}