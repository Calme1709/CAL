use bitflags::bitflags;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BranchConditions(u16);

bitflags! {
    impl BranchConditions: u16 {
        const NEGATIVE = 1 << 2;
        const ZERO     = 1 << 1;
        const POSITIVE = 1 << 0;
    }
}

impl BranchConditions {
    pub fn as_string(self) -> String {
        vec![
            if (self & Self::NEGATIVE).bits() != 0 {"n"} else {""},
            if (self & Self::ZERO).bits() != 0 {"z"} else {""},
            if (self & Self::POSITIVE).bits() != 0 {"p"} else {""},
        ].join("")
    }
}