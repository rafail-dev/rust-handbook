//! # My Crate
//!
//! "//!" - commennt for "lib.rs"
//! "doc to the item that contains the comments rather
//! than to the items following the comments"
//!
//! ```bash
//! cargo doc --open
//! python3 -m http.server --directory ./target/doc
//! ```

/// "///" - md-based comment
/// for generated docs
///
/// # Examples
///
/// ```
/// let result = ch14_cargo_and_crates::my(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn my(a: i32, b: i32) -> i32 {
    a + b
}

// re-export
pub use self::sub::subsub::My;

pub mod sub {
    pub mod subsub {
        pub enum My {}
    }
}