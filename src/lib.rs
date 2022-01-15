#![warn(missing_docs)]
// This is an HTML parser. HTML can be untrusted input from the internet.
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::option_option)]
#![allow(clippy::too_many_lines)]

mod arrayvec;
mod char_validator;
mod emitter;
mod entities;
mod error;
mod machine;
mod machine_helper;
mod never;
mod read_helper;
mod reader;
mod tokenizer;
mod utils;

#[cfg(feature = "integration-tests")]
pub mod testutils;

pub(crate) fn trace_log(msg: &str) {
    #[cfg(feature = "integration-tests")]
    testutils::trace_log(msg);
    #[cfg(not(feature = "integration-tests"))]
    let _ = msg;
}

pub use emitter::{DefaultEmitter, Doctype, Emitter, EndTag, StartTag, Token};
pub use error::Error;
pub use never::Never;
pub use reader::{IoReader, Readable, Reader, StringReader};
pub use tokenizer::{InfallibleTokenizer, Tokenizer};
