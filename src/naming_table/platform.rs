/// A platform identifier.
pub type PlatformID = u16;

enumeration! {
    #[doc = "A platform."]
    pub Platform(PlatformID) {
        0 => Unicode,
        1 => Macintosh,
        3 => Windows,
    }
}
