extern crate serde_test;
extern crate platform_lp as platform; use platform::{PartialPlatform,Architecture,Platform};

#[test]
fn new() {
    //! checks string parser works to get the correct platform

    assert_eq!(Platform::new("linux64"),Platform::Nix64);
    assert_eq!(Platform::new("w32"),Platform::Win32);
    assert_eq!(Platform::new("linux i686"),Platform::Nix32);
    assert_eq!(Platform::new("aPpLe 64 bit"),Platform::Mac64);
}

#[test]
fn clone() {
    //! makes sure clone works, check the address of the objects to make sure they are different.

    let plat1 = Platform::new("linux64");
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

    let platform = Platform::new("linux 64)");
    let token = platform.as_short_str();
    assert_tokens(&platform,&[Token::Str(token)]);

}

#[test]
fn trait_display() {
    let platform = Platform::new("linux 64)");
    println!("{}",platform);
}

#[test]
fn partials() {
    let platform = Platform::new("linux 64)");

    assert!(platform == PartialPlatform::Linux);
    assert!(platform != PartialPlatform::Windows);
    assert!(platform != Architecture::X32);
    assert!(platform == Architecture::X64);
}