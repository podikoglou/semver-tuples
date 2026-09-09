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

impl Default for MajorVersion {
    fn default() -> Self {
        Self(
            MinorVersion::new(1),
            MinorVersion::new(2),
            MinorVersion::new(3),
            MinorVersion::new(4),
            MinorVersion::new(5),
            MinorVersion::new(6),
            MinorVersion::new(7),
            MinorVersion::new(8),
            MinorVersion::new(9),
            MinorVersion::new(10),
        )
    }
}

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
    u16, // <-- MAJOR ver
);

impl MinorVersion {
    pub fn new(major_ver: u16) -> Self {
        Self(
            PatchVersion::new(1 * major_ver),
            PatchVersion::new(2 * major_ver),
            PatchVersion::new(3 * major_ver),
            PatchVersion::new(4 * major_ver),
            PatchVersion::new(5 * major_ver),
            PatchVersion::new(6 * major_ver),
            PatchVersion::new(7 * major_ver),
            PatchVersion::new(8 * major_ver),
            PatchVersion::new(9 * major_ver),
            PatchVersion::new(10 * major_ver),
            major_ver,
        )
    }
}

pub struct PatchVersion(
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    u16, // <-- MINOR ver
);

impl PatchVersion {
    pub fn new(minor_ver: u16) -> Self {
        Self(
            1 * minor_ver,
            2 * minor_ver,
            3 * minor_ver,
            4 * minor_ver,
            5 * minor_ver,
            6 * minor_ver,
            7 * minor_ver,
            8 * minor_ver,
            9 * minor_ver,
            10 * minor_ver,
            minor_ver,
        )
    }
}

lazy_static! {
    static ref v: MajorVersion = MajorVersion::default();
}

#[cfg(test)]
mod test {
    use crate::v;

    #[test]
    fn test_comparison() {
        assert!(v.0.0.0 == v.0.0.0);

        assert!(v.0.0.4 > v.0.0.3);
        assert!(v.0.2.4 > v.0.2.3);
        assert!(v.1.2.4 > v.1.2.3);
    }
}
