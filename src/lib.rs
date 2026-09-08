use lazy_static::lazy_static;

#[derive(Default)]
pub struct MajorVersion(
    pub Box<MinorVersion>,
    pub Box<MinorVersion>,
    pub Box<MinorVersion>,
    pub Box<MinorVersion>,
    pub Box<MinorVersion>,
    pub Box<MinorVersion>,
    pub Box<MinorVersion>,
    pub Box<MinorVersion>,
    pub Box<MinorVersion>,
    pub Box<MinorVersion>,
);

#[derive(Default)]
pub struct MinorVersion(
    pub Box<PatchVersion>,
    pub Box<PatchVersion>,
    pub Box<PatchVersion>,
    pub Box<PatchVersion>,
    pub Box<PatchVersion>,
    pub Box<PatchVersion>,
    pub Box<PatchVersion>,
    pub Box<PatchVersion>,
    pub Box<PatchVersion>,
    pub Box<PatchVersion>,
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
