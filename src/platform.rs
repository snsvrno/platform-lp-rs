use std::slice::Iter;

// the str constant values
const WIN64 : &str = "Windows x86_64";
const WIN32 : &str = "Windows x32";
const NIX64 : &str = "Linux x86_64";
const NIX32 : &str = "Linux x32";
const MAC64 : &str = "Mac OS x86_64";
const MAC32 : &str = "Mac OS x32";
const NONE : &str = "None";

// the short str constant values
const S_WIN64 : &str = "win64";
const S_WIN32 : &str = "win32";
const S_NIX64 : &str = "nix64";
const S_NIX32 : &str = "nix32";
const S_MAC64 : &str = "mac64";
const S_MAC32 : &str = "mac32";

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

    // MEMBER FUNCTIONS /////////////////////////////////

    // creation functions ///////////////////////////////

    pub fn new(platform : &str) -> Platform {
        //! Creates a new platform from a string.
        //!
        //! Works through a number of different possibilities to make sure 
        //! the correct platform is matched based on the string entered.
        //! 
        //! Supports exact matches to the enums [as_str](#method.as_str) and [as_short_str](#method.as_short_str)
        //! as well as pattern matching with keywords such as:
        //! - 64,32
        //! - win,nix,linux,mac
        //! - lxx,wxx,mxx (for short codes)

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
        if string_lower.contains("64") { 
            arch = 64; 
        }
        // platform
        if string_lower.contains("win") { 
            plat = if arch == 32 { Platform::Win32 } 
                   else { Platform::Win64 }; 
        }
        if string_lower.contains("nix") || string_lower.contains("lin") { 
            plat = if arch == 32 { Platform::Nix32 } 
                   else { Platform::Nix64 }; 
        }
        if string_lower.contains("mac") || string_lower.contains("apple") { 
            plat = if arch == 32 { Platform::Mac32 } 
                   else { Platform::Mac64 }; 
        }

        plat
    }

    pub fn clone(&self) -> Platform {
        //! Creates a copy of the enum object.

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

    // display / io functions ///////////////////////////

    pub fn display(&self) -> &'static str { 
        //! Displays the platform object using [as_str](#method.as_str)

        self.as_str() 
    }

    pub fn to_string(&self) -> String { 
        //! returns a String object with the &str value of the platform

        self.as_str().to_string() 
    }

    pub fn as_str(&self) -> &'static str {
        //! Returns a &str constant value for each type.
        //! - Win64 === 'Windows x86_64'
        //! - Win32 === 'Windows x32'
        //! - Nix64 === 'Linux x86_64'
        //! - Nix32 === 'Linux x32'
        //! - Mac64 === 'Mac OS x86_64'
        //! - Mac32 === 'Mac OS x32'
        //! - None === 'None'

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
        //! Returns a short &str constant value
        //! 
        //! - Win64 === 'win64'
        //! - Win32 === 'win32'
        //! - Nix64 === 'nix64'
        //! - Nix32 === 'nix32'
        //! - Mac64 === 'mac64'
        //! - Mac32 === 'mac32'
        //! - None === 'None'

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

    // working functions //////////////////////////////////

    pub fn is_valid_execution_platform(&self) -> bool {
        //! Checks if a a binary were built for the `self` platform
        //! could be executed on the current user platform.
        //! 
        //! Assumes that 32 bit applications can be executed on 64 bit
        //! platforms. If you don't want to assume this then don't use this
        //! funtion and instead check absolute equality (==).

        let valid_platforms = Platform::get_valid_execution_platform();

        for platform in valid_platforms {
            if &platform == self {
                return true;
            }
        }

        false
    }

    pub fn is_compatible(&self,other : &Platform) -> bool {
        //! Checks if this platform is compatible with the other platform,
        //! 
        //! Works on the assumption that 32 bit is compatible with 64 bit.
        
        let exec_plats = Platform::get_execution_platform_for(other);

        for plat in exec_plats {
            if plat == *self { 
                return true;
            }
        }

        false
    }

    // STATIC FUNCTIONS /////////////////////////////////////

    pub fn get_user_platform() -> Platform {
        //! Returns the current user platform

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

    fn get_execution_platform_for(platform : &Platform) -> Vec<Platform> {

        match *platform {
            Platform::Win64 => vec![Platform::Win64,Platform::Win32],
            Platform::Win32 => vec![Platform::Win32],
            Platform::Nix64 => vec![Platform::Nix64,Platform::Nix32],
            Platform::Nix32 => vec![Platform::Nix32],
            Platform::Mac64 => vec![Platform::Mac64,Platform::Mac32],
            Platform::Mac32 => vec![Platform::Mac32],
            Platform::None => Vec::new()
        }
    }

    pub fn get_valid_execution_platform() -> Vec<Platform> {
        //! Returns a Vec of all the available execution platforms
        //!
        //! Typically its a vector of 1, but also returns the 32bit version on 64bit systems.
        
        Platform::get_execution_platform_for(&Platform::get_user_platform())
    }

    pub fn iterator() -> Iter<'static, Platform> {
        //! An iterator that iterates through all the defined platforms

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
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.to_string())
    }
}