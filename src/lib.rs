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
//! And has Partials to allow for matching only the OS and Architecture
//! - Windows
//! - Linux
//! - Mac
//! - X32
//! - X64
//! 
//! ## Example
//! ```rust
//! use platform_lp::{ PartialPlatform, Platform, Architecture};
//! 
//! let the_platform = Platform::get_user_platform();
//! 
//! # let the_platform = Platform::Win64;
//! 
//! assert!(the_platform == PartialPlatform::Windows);
//! assert!(the_platform == Architecture::X64);
//! 
//! ```

extern crate serde;

mod platform; 
pub use platform::Platform;

mod partialplatform;
pub use partialplatform::PartialPlatform;

mod architecture;
pub use architecture::Architecture;

mod serialization;