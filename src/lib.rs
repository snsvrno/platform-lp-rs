//! Platform structure for use with `lovepack` 
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate serde_test;

extern crate serde;

use std::slice::Iter;
use std::fmt;
use std::marker::PhantomData;

// the str constant values
pub const WIN64 : &str = "Windows x86_64";
pub const WIN32 : &str = "Windows x32";
pub const NIX64 : &str = "Linux x86_64";
pub const NIX32 : &str = "Linux x32";
pub const MAC64 : &str = "Mac OS x86_64";
pub const MAC32 : &str = "Mac OS x32";
pub const NONE : &str = "None";

// the str constant values
pub const S_WIN64 : &str = "win64";
pub const S_WIN32 : &str = "win32";
pub const S_NIX64 : &str = "nix64";
pub const S_NIX32 : &str = "nix32";
pub const S_MAC64 : &str = "mac64";
pub const S_MAC32 : &str = "mac32";

struct PlatformVisitor {
  marker: PhantomData<fn() -> Platform>
}

impl PlatformVisitor {
  fn new() -> Self {
    PlatformVisitor {
      marker : PhantomData
    }
  }
}

impl <'de>serde::de::Visitor<'de> for PlatformVisitor {
  type Value = Platform;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("platform")
  }

  fn visit_enum<A>(self, data:A) -> Result<Self::Value, A::Error> where A: serde::de::EnumAccess<'de> {
    if let Ok((variant,_)) = data.variant() {
      Ok(Platform::new(variant))
    } else { Ok(Platform::None) }
  }
}

#[derive(Debug,PartialEq,Hash,Eq)]
pub enum Platform {
  Win64,
  Win32,
  Nix64,
  Nix32,
  Mac64,
  Mac32,
  None
}

impl Platform {

  // OBJECT FUNCTIONS

  pub fn clone(&self) -> Platform {
    //! copies the platform into a new platform

    match *self {
      Platform::Win64 => { Platform::Win64 },
      Platform::Win32 => { Platform::Win32 },
      Platform::Nix64 => { Platform::Nix64 },
      Platform::Nix32 => { Platform::Nix32 },
      Platform::Mac64 => { Platform::Mac64 },
      Platform::Mac32 => { Platform::Mac32 },
      Platform::None => { Platform::None }
    }
  }

  pub fn display(&self) -> &'static str { 
    //! displays the platform object

    self.as_str() 
  }

  pub fn to_string(&self) -> String { 
    //! returns a String object with the &str value of the platform

    self.as_str().to_string() 
  }

  pub fn as_str(&self) -> &'static str {
    //! returns the &str constant value

    match *self {
      Platform::Win64 => { WIN64 },
      Platform::Win32 => { WIN32 },
      Platform::Nix64 => { NIX64 },
      Platform::Nix32 => { NIX32 },
      Platform::Mac64 => { MAC64 },
      Platform::Mac32 => { MAC32 },
      Platform::None => { NONE }
    }
  }

  pub fn to_short_string(&self) -> String { 
    //! returns a String object with the short &str value of the platform

    self.as_short_str().to_string() 
  }

  pub fn as_short_str(&self) -> &'static str {
    //! returns the short &str constant value

    match *self {
      Platform::Win64 => { S_WIN64 },
      Platform::Win32 => { S_WIN32 },
      Platform::Nix64 => { S_NIX64 },
      Platform::Nix32 => { S_NIX32 },
      Platform::Mac64 => { S_MAC64 },
      Platform::Mac32 => { S_MAC32 },
      Platform::None => { NONE }
    }
  }

  pub fn new(platform : &str) -> Platform {
    //! creates a new platform from a string
    //!
    //! uses a set of possible strings similar this set

    let mut plat = Platform::None;

    match platform {
      NIX64 => { plat = Platform::Nix64; },
      NIX32 => { plat = Platform::Nix32; },
      WIN64 => { plat = Platform::Win64; },
      WIN32 => { plat = Platform::Win32; },
      MAC64 => { plat = Platform::Mac64; },
      MAC32 => { plat = Platform::Mac32; },
      _ => { }
    }

    match platform.to_lowercase().as_str() {
      "linux64" | "lin64" | "nix64" | "l64" => { plat = Platform::Nix64; },
      "linux32" | "lin32" | "nix32" | "l32" => { plat = Platform::Nix32; },
      "windows64" | "win64" | "w64" => { plat = Platform::Win64; },
      "windows32" | "win32" | "w32" => { plat = Platform::Win32; },
      "macos64" | "mac64" | "m64" => { plat = Platform::Mac64; },
      "macos32" | "mac32" | "m32" => { plat = Platform::Mac32; }
      _ => { }
    }

    let string_lower = platform.to_lowercase();

    // architecture
    let mut arch : u8 = 32;
    if string_lower.contains("64") || string_lower.contains("x86") { arch = 64; }
    // platform
    if string_lower.contains("win") { plat = if arch == 32 { Platform::Win32 } else { Platform::Win64 }; }
    if string_lower.contains("nix") || string_lower.contains("lin") { plat = if arch == 32 { Platform::Nix32 } else { Platform::Nix64 }; }
    if string_lower.contains("mac") || string_lower.contains("apple") { plat = if arch == 32 { Platform::Mac32 } else { Platform::Mac64 }; }

    plat
  }

  pub fn is_valid_execution_platform(&self) -> bool {
    //! checks if a binary built on the *self* platform would run on the current platform

    #[cfg(all(unix,target_pointer_width = "64"))]
    match *self { 
      Platform::Nix64 => { true }
      Platform::Nix32 => { true }
      _ => { false }
    }
    #[cfg(all(unix,target_pointer_width = "32"))]
    match *self { 
      Platform::Nix32 => { true }
      _ => { false }
    }
    #[cfg(all(windows,target_pointer_width = "64"))]
    match *self { 
      Platform::Win64 => { true }
      Platform::Win32 => { true }
      _ => { false }
    }
    #[cfg(all(windows,target_pointer_width = "32"))]
    match *self { 
      Platform::Win32 => { true }
      _ => { false }
    }
    #[cfg(all(macos,target_pointer_width = "64"))]
    match *self { 
      Platform::Mac64 => { true }
      Platform::Mac32 => { true }
      _ => { false }
    }
    #[cfg(all(macos,target_pointer_width = "32"))]
    match *self { 
      Platform::Mac32 => { true }
      _ => { false }
    }
  }

  // STATIC FUNCTIONS

  pub fn get_user_platform() -> Platform {
    //! returns the current user platform

    #[cfg(all(unix,target_pointer_width = "64"))]
      return Platform::Nix64;
    #[cfg(all(unix,target_pointer_width = "32"))]
      return Platform::Nix32;
    #[cfg(all(windows,target_pointer_width = "64"))]
      return Platform::Win64;
    #[cfg(all(windows,target_pointer_width = "32"))]
      return Platform::Win32;
    #[cfg(all(macos,target_pointer_width = "64"))]
      return Platform::Mac64;
    #[cfg(all(macos,target_pointer_width = "32"))]
      return Platform::Mac32;
  }

  pub fn iterator() -> Iter<'static, Platform> {
    //! an iterator that iterates through all the defined platforms

    static PLATFORMS: [Platform; 6] = [
      Platform::Win64,
      Platform::Win32,
      Platform::Nix64,
      Platform::Nix32,
      Platform::Mac64,
      Platform::Mac32
    ];
    PLATFORMS.into_iter()
  }

  pub fn get_valid_execution_platform() -> Vec<Platform> {
    //! returns a vector of all the available execution platforms
    //!
    //! typically its a vector of 1, but also returns the 32bit version on 64bit systems.

    let mut valid_platforms = Vec::new();

    #[cfg(all(unix,target_pointer_width = "64"))]
    { 
      valid_platforms.push(Platform::Nix64);
      valid_platforms.push(Platform::Nix32);
    }
    #[cfg(all(unix,target_pointer_width = "32"))]
    valid_platforms.push(Platform::Nix32);
    #[cfg(all(windows,target_pointer_width = "64"))]
    { 
      valid_platforms.push(Platform::Win64);
      valid_platforms.push(Platform::Win32);
    }
    #[cfg(all(windows,target_pointer_width = "32"))]
    valid_platforms.push(Platform::Win32);
    #[cfg(all(macos,target_pointer_width = "64"))]
    { 
      valid_platforms.push(Platform::Mac64);
      valid_platforms.push(Platform::Mac32);
    }
    #[cfg(all(macos,target_pointer_width = "32"))]
    valid_platforms.push(Platform::Mac32);

    valid_platforms
  }
}

impl serde::Serialize for Platform {
  fn serialize<S>(&self,serializer : S) -> Result<S::Ok, S::Error> where S : serde::Serializer {
    serializer.serialize_str(&self.to_short_string())
  }
}

impl <'de> serde::Deserialize<'de> for Platform {
  fn deserialize<D>(deserializer : D) -> Result<Platform, D::Error> where D : serde::Deserializer<'de> {
    deserializer.deserialize_enum("Platform",
      &["Windows x32","Linux x86_64","Linux x32","Mac OS x86_64","Mac OS x32","None"],
      PlatformVisitor::new()
    )
  }
}

#[cfg(test)]
mod tests {
  
    #[test]
    fn new() {
      //! checks string parser works to get the correct platform

      assert_eq!(super::Platform::new("linux64"),super::Platform::Nix64);
      assert_eq!(super::Platform::new("w32"),super::Platform::Win32);
      assert_eq!(super::Platform::new("linux i686"),super::Platform::Nix32);
      assert_eq!(super::Platform::new("aPpLe 64 bit"),super::Platform::Mac64);
    }

    #[test]
    fn clone() {
      //! makes sure clone works, check the address of the objects to make sure they are different.

      let plat1 = super::Platform::new("linux64");
      let plat2 = plat1.clone();
      let plat3 = &plat1;

      let p1 = &plat1 as *const _;
      let p2 = &plat2 as *const _;
      let p3 = plat3 as *const _;

      assert_eq!(plat1,plat2);
      assert_eq!(p1,p3);
      assert_eq!(p1 == p2,false);
    }

    #[test]
    fn serde() {
      use serde_test::{Token, assert_tokens};
  
      let platform = super::Platform::new("linux 64)");
      assert_tokens(&platform,&[Token::Str("Linux x86_64")]);

    }
}