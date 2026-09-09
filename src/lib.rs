use lazy_static::lazy_static;

#[derive(Default)]
pub struct MajorVersion(
    pub MinorVersion,
    pub MinorVersion,
    pub MinorVersion,
    pub MinorVersion,
    pub MinorVersion,
    pub MinorVersion,
    pub MinorVersion,
    pub MinorVersion,
    pub MinorVersion,
    pub MinorVersion,
);

#[derive(Default)]
pub struct MinorVersion(
    pub PatchVersion,
    pub PatchVersion,
    pub PatchVersion,
    pub PatchVersion,
    pub PatchVersion,
    pub PatchVersion,
    pub PatchVersion,
    pub PatchVersion,
    pub PatchVersion,
    pub PatchVersion,
);

#[derive(Default)]
pub struct PatchVersion(
    pub (),
    pub (),
    pub (),
    pub (),
    pub (),
    pub (),
    pub (),
    pub (),
    pub (),
    pub (),
);

lazy_static! {
    static ref v: MajorVersion = MajorVersion::default();
}

#[cfg(test)]
mod test {
    use crate::v;

    fn test() {
        let a = v.1.2.3;
        let b = v.1.2.4;

        assert!(b > a);
    }
}
