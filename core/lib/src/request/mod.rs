//! Types and traits for request parsing and handling.

mod request;
mod from_param;
mod from_request;

#[cfg(test)]
mod tests;

pub use self::request::Request;
pub use self::from_request::{FromRequest, Outcome};
pub use self::from_param::{FromParam, FromSegments};

#[doc(inline)]
pub use crate::response::flash::FlashMessage;

pub(crate) use self::request::ConnectionMeta;

crate::export! {
    /// Store and immediately retrieve a vector-like value `$v` (`String` or
    /// `Vec<T>`) in `$request`'s local cache using a locally generated
    /// anonymous type to avoid type conflicts.
    ///
    /// Unlike `local_cache_once`, this macro's generated code _always_ returns
    /// a unique reference to request-local cache.
    ///
    /// # Note
    ///
    /// The value `$v` must be of type `String` or `Vec<T>`, that is, a type
    /// that implements the sealed trait [`Shareable`](crate::form::Shareable)bb).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::request::{local_cache, local_cache_once};
    /// # let c = rocket::local::blocking::Client::debug_with(vec![]).unwrap();
    /// # let request = c.get("/");
    ///
    /// // The first store into local cache for a given type wins.
    /// for i in 0..4 {
    ///     assert_eq!(request.local_cache(|| i.to_string()), "0");
    /// }
    ///
    /// // This shows that we cannot cache different values of the same type; we
    /// // _must_ use a proxy type. To avoid the need to write these manually, use
    /// // `local_cache!`, which generates one of the fly.
    /// for i in 0..4 {
    ///     assert_eq!(local_cache!(request, i.to_string()), i.to_string());
    /// }
    ///
    /// // Note that while `local_cache_once!` generates a new type for the
    /// // _macro_ invocation, that type is the same per run-time invocation, so
    /// // all "calls" to `local_cache_once!` on the same line return the same
    /// // reference for a given request.
    /// for i in 1..4 {
    ///     // Note that this is `1`, so _not_ the `String` from line 4.
    ///     assert_eq!(local_cache_once!(request, i.to_string()), "1");
    /// }
    /// ```
    macro_rules! local_cache {
        ($request:expr, $v:expr $(,)?) => ({
            struct Local<T: $crate::form::Shareable>($crate::form::SharedStack<T>);
            let stack = $request.local_cache(|| Local($crate::form::SharedStack::new()));
            stack.0.push_owned($v)
        })
    }
}

crate::export! {
    /// Store and immediately retrieve a value `$v` in `$request`'s local cache
    /// using a locally generated anonymous type to avoid type conflicts.
    ///
    /// The code generated by this macro is expected to be invoked at-most once
    /// per-request. This is because while `local_cache_once!` generates a new
    /// type for the _macro_ invocation, that type is the same per run-time
    /// invocation. Thus, for a given request, a `local_cache_once!` invocation
    /// always returns the same reference.
    ///
    /// To get a unique request-local reference to string-like values, use
    /// [`local_cache!`] instead.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::request::local_cache_once;
    /// # let c = rocket::local::blocking::Client::debug_with(vec![]).unwrap();
    /// # let request = c.get("/");
    ///
    /// // The first store into local cache for a given type wins.
    /// assert_eq!(request.local_cache(|| String::from("hello")), "hello");
    ///
    /// // The following returns the cached, previously stored value for the type.
    /// assert_eq!(request.local_cache(|| String::from("goodbye")), "hello");
    ///
    /// // This shows that we cannot cache different values of the same type;
    /// // we _must_ use a proxy type. To avoid the need to write these manually,
    /// // use `local_cache_once!`, which generates one of the fly.
    /// assert_eq!(local_cache_once!(request, String::from("hello")), "hello");
    /// assert_eq!(local_cache_once!(request, String::from("goodbye")), "goodbye");
    ///
    /// // But a macro invocation for the same request always resolves to the
    /// // first reference as the unique type is generated at compile-time.
    /// for i in 1..4 {
    ///     assert_eq!(local_cache_once!(request, i.to_string()), "1");
    /// }
    /// ```
    macro_rules! local_cache_once {
        ($request:expr, $v:expr $(,)?) => ({
            struct Local<T>(T);
            &$request.local_cache(move || Local($v)).0
        })
    }
}
