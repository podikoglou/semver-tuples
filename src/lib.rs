use lazy_static::lazy_static;

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

pub struct PatchVersion(
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
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
