#[macro_use] extern crate serde_derive;
extern crate serde;

mod platform; 
pub use platform::Platform;

mod serialization;