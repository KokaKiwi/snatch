use hyper::header::{ContentLength, Headers};

use Bytes;

/// Trait to extend functionalities of the Headers type, from `hyper`
pub trait GetContentLength {
    /// Function to get the content length of a remote document.
    /// The returned type is `Option<Bytes>`.
    fn get_content_length(&self) -> Option<Bytes>;
}

impl GetContentLength for Headers {
    /// Function to get the `content-length` container, from a given header.
    /// This function returns an Option that contains a `Bytes` type.
    fn get_content_length(&self) -> Option<Bytes> {
        self.get::<ContentLength>()
            // We need to deref it twice as it's a *double* reference!
            // (&ContentLength -> ContentLength -> u32)
            .map(|length| **length)
    }
}
