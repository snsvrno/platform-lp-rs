use platform_lp as platform;

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
    assert_tokens(&platform,&[Token::Str(super::S_NIX64)]);

}