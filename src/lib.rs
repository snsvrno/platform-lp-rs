//! Platform structure for use with `lovepack` 

use std::slice::Iter;

// the str constant values
pub const WIN64 : &str = "Windows x86_64";
pub const WIN32 : &str = "Windows x32";
pub const NIX64 : &str = "Linux x86_64";
pub const NIX32 : &str = "Linux x32";
pub const MAC64 : &str = "Mac OS x86_64";
pub const MAC32 : &str = "Mac OS 32";

#[derive(Debug,PartialEq)]
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
      Platform::None => { "You are not running an OS." }
    }
  }

  pub fn new(platform : &str) -> Platform {
    //! creates a new platform from a string
    //!
    //! uses a set of possible strings similar this set
    //!
    //! ```rust
    //! "linux64" | "lin64" | "nix64" | "l64"
    //! ```

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

