//! # Platform-lp
//! 
//! A crate for easily determining the user's platform at runtime and comparision of platforms.
//! 
//! _Platform-lp_ is made up of an enum, [Platform](enum.Platform.html) which can be used
//! to determine what platform a user is running, what platform other tools
//! or binaries might be and then compare the two to find the right object
//! for your user based on their current running platform.
//! 
//! Currently supports the following platforms
//! - Win64
//! - Win32
//! - Linux64
//! - Linux32
//! - MacOS64
//! - MacOS32
//! 
//! ## Example
//! ```rust
//! let the_platform = platform_lp::Platform::get_user_platform();
//! ```

extern crate serde;

mod platform; 
pub use platform::Platform;

mod serialization;