use Platform;

/// Platform Partial, designed for 'fuzzy' comparisons.
/// 
/// ```rust
/// use platform_lp::{ Platform, PartialPlatform};
/// 
/// let the_platform = Platform::get_user_platform();
/// # let the_platform = Platform::Win64;
/// assert!(the_platform == PartialPlatform::Windows);
#[derive(Debug,PartialEq,Hash,Eq)]
pub enum PartialPlatform {
    Windows,
    Linux,
    Mac,
}

impl PartialEq<Platform> for PartialPlatform {
    fn eq(&self, other: &Platform) -> bool {
        match (self, other) {
            (PartialPlatform::Linux, Platform::Nix64) => true,
            (PartialPlatform::Linux, Platform::Nix32) => true,
            (PartialPlatform::Windows, Platform::Win64) => true,
            (PartialPlatform::Windows, Platform::Win32) => true,
            (PartialPlatform::Mac, Platform::Mac64) => true,
            (PartialPlatform::Mac, Platform::Mac32) => true,
            (_,_) => false
        }
    }
}