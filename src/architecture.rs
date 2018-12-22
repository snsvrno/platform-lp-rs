use Platform;

/// Architecture Partial, designed for 'fuzzy' comparisons.
/// 
/// ```rust
/// use platform_lp::{ Platform, Architecture};
/// 
/// let the_platform = Platform::get_user_platform();
/// # let the_platform = Platform::Win64;
/// assert!(the_platform == Architecture::X64);
#[derive(Debug,PartialEq,Hash,Eq)]
pub enum Architecture {
    X64,
    X32,
}

impl PartialEq<Platform> for Architecture {
    fn eq(&self, other: &Platform) -> bool {
        match (self, other) {
            (Architecture::X64, Platform::Nix64) => true,
            (Architecture::X32, Platform::Nix32) => true,
            (Architecture::X64, Platform::Win64) => true,
            (Architecture::X32, Platform::Win32) => true,
            (Architecture::X64, Platform::Mac64) => true,
            (Architecture::X32, Platform::Mac32) => true,
            (_,_) => false
        }
    }
}