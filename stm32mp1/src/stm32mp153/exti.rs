#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Contains only register bits for configurable events."]
    pub exti_rtsr1: crate::Reg<exti_rtsr1::EXTI_RTSR1_SPEC>,
    #[doc = "0x04 - Contains only register bits for configurable events."]
    pub exti_ftsr1: crate::Reg<exti_ftsr1::EXTI_FTSR1_SPEC>,
    #[doc = "0x08 - Contains only register bits for configurable events."]
    pub exti_swier1: crate::Reg<exti_swier1::EXTI_SWIER1_SPEC>,
    #[doc = "0x0c - Contains only register bits for configurable events."]
    pub exti_rpr1: crate::Reg<exti_rpr1::EXTI_RPR1_SPEC>,
    #[doc = "0x10 - Contains only register bits for configurable events."]
    pub exti_fpr1: crate::Reg<exti_fpr1::EXTI_FPR1_SPEC>,
    #[doc = "0x14 - This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
    pub exti_tzenr1: crate::Reg<exti_tzenr1::EXTI_TZENR1_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - Contains only register bits for configurable events."]
    pub exti_rtsr2: crate::Reg<exti_rtsr2::EXTI_RTSR2_SPEC>,
    #[doc = "0x24 - Contains only register bits for configurable events."]
    pub exti_ftsr2: crate::Reg<exti_ftsr2::EXTI_FTSR2_SPEC>,
    #[doc = "0x28 - Contains only register bits for configurable events."]
    pub exti_swier2: crate::Reg<exti_swier2::EXTI_SWIER2_SPEC>,
    #[doc = "0x2c - Contains only register bits for configurable events."]
    pub exti_rpr2: crate::Reg<exti_rpr2::EXTI_RPR2_SPEC>,
    #[doc = "0x30 - Contains only register bits for configurable events."]
    pub exti_fpr2: crate::Reg<exti_fpr2::EXTI_FPR2_SPEC>,
    #[doc = "0x34 - This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
    pub exti_tzenr2: crate::Reg<exti_tzenr2::EXTI_TZENR2_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x40 - Contains only register bits for configurable events."]
    pub exti_rtsr3: crate::Reg<exti_rtsr3::EXTI_RTSR3_SPEC>,
    #[doc = "0x44 - Contains only register bits for configurable events."]
    pub exti_ftsr3: crate::Reg<exti_ftsr3::EXTI_FTSR3_SPEC>,
    #[doc = "0x48 - Contains only register bits for configurable events."]
    pub exti_swier3: crate::Reg<exti_swier3::EXTI_SWIER3_SPEC>,
    #[doc = "0x4c - Contains only register bits for configurable events."]
    pub exti_rpr3: crate::Reg<exti_rpr3::EXTI_RPR3_SPEC>,
    #[doc = "0x50 - Contains only register bits for configurable events."]
    pub exti_fpr3: crate::Reg<exti_fpr3::EXTI_FPR3_SPEC>,
    #[doc = "0x54 - This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
    pub exti_tzenr3: crate::Reg<exti_tzenr3::EXTI_TZENR3_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0x60 - EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
    pub exti_exticr1: crate::Reg<exti_exticr1::EXTI_EXTICR1_SPEC>,
    #[doc = "0x64 - EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
    pub exti_exticr2: crate::Reg<exti_exticr2::EXTI_EXTICR2_SPEC>,
    #[doc = "0x68 - EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
    pub exti_exticr3: crate::Reg<exti_exticr3::EXTI_EXTICR3_SPEC>,
    #[doc = "0x6c - EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
    pub exti_exticr4: crate::Reg<exti_exticr4::EXTI_EXTICR4_SPEC>,
    _reserved22: [u8; 0x10],
    #[doc = "0x80 - Contains register bits for configurable events and Direct events."]
    pub exti_imr1: crate::Reg<exti_imr1::EXTI_IMR1_SPEC>,
    #[doc = "0x84 - EXTI CPU wakeup with event mask register"]
    pub exti_emr1: crate::Reg<exti_emr1::EXTI_EMR1_SPEC>,
    _reserved24: [u8; 0x08],
    #[doc = "0x90 - Contains register bits for configurable events and direct events."]
    pub exti_imr2: crate::Reg<exti_imr2::EXTI_IMR2_SPEC>,
    #[doc = "0x94 - EXTI CPU wakeup with event mask register"]
    pub exti_emr2: crate::Reg<exti_emr2::EXTI_EMR2_SPEC>,
    _reserved26: [u8; 0x08],
    #[doc = "0xa0 - Contains register bits for configurable events and direct events."]
    pub exti_imr3: crate::Reg<exti_imr3::EXTI_IMR3_SPEC>,
    #[doc = "0xa4 - EXTI CPU wakeup with event mask register"]
    pub exti_emr3: crate::Reg<exti_emr3::EXTI_EMR3_SPEC>,
    _reserved28: [u8; 0x18],
    #[doc = "0xc0 - Contains register bits for configurable events and Direct events."]
    pub exti_c2imr1: crate::Reg<exti_c2imr1::EXTI_C2IMR1_SPEC>,
    #[doc = "0xc4 - EXTI CPU2 wakeup with event mask register"]
    pub exti_c2emr1: crate::Reg<exti_c2emr1::EXTI_C2EMR1_SPEC>,
    _reserved30: [u8; 0x08],
    #[doc = "0xd0 - Contains register bits for configurable events and direct events."]
    pub exti_c2imr2: crate::Reg<exti_c2imr2::EXTI_C2IMR2_SPEC>,
    #[doc = "0xd4 - EXTI CPU2 wakeup with event mask register"]
    pub exti_c2emr2: crate::Reg<exti_c2emr2::EXTI_C2EMR2_SPEC>,
    _reserved32: [u8; 0x08],
    #[doc = "0xe0 - Contains register bits for configurable events and direct events."]
    pub exti_c2imr3: crate::Reg<exti_c2imr3::EXTI_C2IMR3_SPEC>,
    #[doc = "0xe4 - EXTI CPU2 wakeup with event mask register"]
    pub exti_c2emr3: crate::Reg<exti_c2emr3::EXTI_C2EMR3_SPEC>,
    _reserved34: [u8; 0x02d8],
    #[doc = "0x3c0 - EXTI hardware configuration register 13"]
    pub exti_hwcfgr13: crate::Reg<exti_hwcfgr13::EXTI_HWCFGR13_SPEC>,
    #[doc = "0x3c4 - EXTI hardware configuration register 12"]
    pub exti_hwcfgr12: crate::Reg<exti_hwcfgr12::EXTI_HWCFGR12_SPEC>,
    #[doc = "0x3c8 - EXTI hardware configuration register 11"]
    pub exti_hwcfgr11: crate::Reg<exti_hwcfgr11::EXTI_HWCFGR11_SPEC>,
    #[doc = "0x3cc - EXTI hardware configuration register 10"]
    pub exti_hwcfgr10: crate::Reg<exti_hwcfgr10::EXTI_HWCFGR10_SPEC>,
    #[doc = "0x3d0 - EXTI hardware configuration register 9"]
    pub exti_hwcfgr9: crate::Reg<exti_hwcfgr9::EXTI_HWCFGR9_SPEC>,
    #[doc = "0x3d4 - EXTI hardware configuration register 8"]
    pub exti_hwcfgr8: crate::Reg<exti_hwcfgr8::EXTI_HWCFGR8_SPEC>,
    #[doc = "0x3d8 - EXTI hardware configuration register 7"]
    pub exti_hwcfgr7: crate::Reg<exti_hwcfgr7::EXTI_HWCFGR7_SPEC>,
    #[doc = "0x3dc - EXTI hardware configuration register 6"]
    pub exti_hwcfgr6: crate::Reg<exti_hwcfgr6::EXTI_HWCFGR6_SPEC>,
    #[doc = "0x3e0 - EXTI hardware configuration register 5"]
    pub exti_hwcfgr5: crate::Reg<exti_hwcfgr5::EXTI_HWCFGR5_SPEC>,
    #[doc = "0x3e4 - EXTI hardware configuration register 4"]
    pub exti_hwcfgr4: crate::Reg<exti_hwcfgr4::EXTI_HWCFGR4_SPEC>,
    #[doc = "0x3e8 - EXTI hardware configuration register 3"]
    pub exti_hwcfgr3: crate::Reg<exti_hwcfgr3::EXTI_HWCFGR3_SPEC>,
    #[doc = "0x3ec - EXTI hardware configuration register 2"]
    pub exti_hwcfgr2: crate::Reg<exti_hwcfgr2::EXTI_HWCFGR2_SPEC>,
    #[doc = "0x3f0 - EXTI hardware configuration register 1"]
    pub exti_hwcfgr1: crate::Reg<exti_hwcfgr1::EXTI_HWCFGR1_SPEC>,
    #[doc = "0x3f4 - EXTI IP version register"]
    pub exti_verr: crate::Reg<exti_verr::EXTI_VERR_SPEC>,
    #[doc = "0x3f8 - EXTI identification register"]
    pub exti_ipidr: crate::Reg<exti_ipidr::EXTI_IPIDR_SPEC>,
    #[doc = "0x3fc - EXTI size ID register"]
    pub exti_sidr: crate::Reg<exti_sidr::EXTI_SIDR_SPEC>,
}
#[doc = "EXTI_RTSR1 register accessor: an alias for `Reg<EXTI_RTSR1_SPEC>`"]
pub type EXTI_RTSR1 = crate::Reg<exti_rtsr1::EXTI_RTSR1_SPEC>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rtsr1;
#[doc = "EXTI_FTSR1 register accessor: an alias for `Reg<EXTI_FTSR1_SPEC>`"]
pub type EXTI_FTSR1 = crate::Reg<exti_ftsr1::EXTI_FTSR1_SPEC>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_ftsr1;
#[doc = "EXTI_SWIER1 register accessor: an alias for `Reg<EXTI_SWIER1_SPEC>`"]
pub type EXTI_SWIER1 = crate::Reg<exti_swier1::EXTI_SWIER1_SPEC>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_swier1;
#[doc = "EXTI_RPR1 register accessor: an alias for `Reg<EXTI_RPR1_SPEC>`"]
pub type EXTI_RPR1 = crate::Reg<exti_rpr1::EXTI_RPR1_SPEC>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rpr1;
#[doc = "EXTI_FPR1 register accessor: an alias for `Reg<EXTI_FPR1_SPEC>`"]
pub type EXTI_FPR1 = crate::Reg<exti_fpr1::EXTI_FPR1_SPEC>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_fpr1;
#[doc = "EXTI_TZENR1 register accessor: an alias for `Reg<EXTI_TZENR1_SPEC>`"]
pub type EXTI_TZENR1 = crate::Reg<exti_tzenr1::EXTI_TZENR1_SPEC>;
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
pub mod exti_tzenr1;
#[doc = "EXTI_RTSR2 register accessor: an alias for `Reg<EXTI_RTSR2_SPEC>`"]
pub type EXTI_RTSR2 = crate::Reg<exti_rtsr2::EXTI_RTSR2_SPEC>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rtsr2;
#[doc = "EXTI_FTSR2 register accessor: an alias for `Reg<EXTI_FTSR2_SPEC>`"]
pub type EXTI_FTSR2 = crate::Reg<exti_ftsr2::EXTI_FTSR2_SPEC>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_ftsr2;
#[doc = "EXTI_SWIER2 register accessor: an alias for `Reg<EXTI_SWIER2_SPEC>`"]
pub type EXTI_SWIER2 = crate::Reg<exti_swier2::EXTI_SWIER2_SPEC>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_swier2;
#[doc = "EXTI_RPR2 register accessor: an alias for `Reg<EXTI_RPR2_SPEC>`"]
pub type EXTI_RPR2 = crate::Reg<exti_rpr2::EXTI_RPR2_SPEC>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rpr2;
#[doc = "EXTI_FPR2 register accessor: an alias for `Reg<EXTI_FPR2_SPEC>`"]
pub type EXTI_FPR2 = crate::Reg<exti_fpr2::EXTI_FPR2_SPEC>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_fpr2;
#[doc = "EXTI_TZENR2 register accessor: an alias for `Reg<EXTI_TZENR2_SPEC>`"]
pub type EXTI_TZENR2 = crate::Reg<exti_tzenr2::EXTI_TZENR2_SPEC>;
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
pub mod exti_tzenr2;
#[doc = "EXTI_RTSR3 register accessor: an alias for `Reg<EXTI_RTSR3_SPEC>`"]
pub type EXTI_RTSR3 = crate::Reg<exti_rtsr3::EXTI_RTSR3_SPEC>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rtsr3;
#[doc = "EXTI_FTSR3 register accessor: an alias for `Reg<EXTI_FTSR3_SPEC>`"]
pub type EXTI_FTSR3 = crate::Reg<exti_ftsr3::EXTI_FTSR3_SPEC>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_ftsr3;
#[doc = "EXTI_SWIER3 register accessor: an alias for `Reg<EXTI_SWIER3_SPEC>`"]
pub type EXTI_SWIER3 = crate::Reg<exti_swier3::EXTI_SWIER3_SPEC>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_swier3;
#[doc = "EXTI_RPR3 register accessor: an alias for `Reg<EXTI_RPR3_SPEC>`"]
pub type EXTI_RPR3 = crate::Reg<exti_rpr3::EXTI_RPR3_SPEC>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rpr3;
#[doc = "EXTI_FPR3 register accessor: an alias for `Reg<EXTI_FPR3_SPEC>`"]
pub type EXTI_FPR3 = crate::Reg<exti_fpr3::EXTI_FPR3_SPEC>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_fpr3;
#[doc = "EXTI_TZENR3 register accessor: an alias for `Reg<EXTI_TZENR3_SPEC>`"]
pub type EXTI_TZENR3 = crate::Reg<exti_tzenr3::EXTI_TZENR3_SPEC>;
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
pub mod exti_tzenr3;
#[doc = "EXTI_EXTICR1 register accessor: an alias for `Reg<EXTI_EXTICR1_SPEC>`"]
pub type EXTI_EXTICR1 = crate::Reg<exti_exticr1::EXTI_EXTICR1_SPEC>;
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
pub mod exti_exticr1;
#[doc = "EXTI_EXTICR2 register accessor: an alias for `Reg<EXTI_EXTICR2_SPEC>`"]
pub type EXTI_EXTICR2 = crate::Reg<exti_exticr2::EXTI_EXTICR2_SPEC>;
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
pub mod exti_exticr2;
#[doc = "EXTI_EXTICR3 register accessor: an alias for `Reg<EXTI_EXTICR3_SPEC>`"]
pub type EXTI_EXTICR3 = crate::Reg<exti_exticr3::EXTI_EXTICR3_SPEC>;
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
pub mod exti_exticr3;
#[doc = "EXTI_EXTICR4 register accessor: an alias for `Reg<EXTI_EXTICR4_SPEC>`"]
pub type EXTI_EXTICR4 = crate::Reg<exti_exticr4::EXTI_EXTICR4_SPEC>;
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
pub mod exti_exticr4;
#[doc = "EXTI_IMR1 register accessor: an alias for `Reg<EXTI_IMR1_SPEC>`"]
pub type EXTI_IMR1 = crate::Reg<exti_imr1::EXTI_IMR1_SPEC>;
#[doc = "Contains register bits for configurable events and Direct events."]
pub mod exti_imr1;
#[doc = "EXTI_EMR1 register accessor: an alias for `Reg<EXTI_EMR1_SPEC>`"]
pub type EXTI_EMR1 = crate::Reg<exti_emr1::EXTI_EMR1_SPEC>;
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod exti_emr1;
#[doc = "EXTI_IMR2 register accessor: an alias for `Reg<EXTI_IMR2_SPEC>`"]
pub type EXTI_IMR2 = crate::Reg<exti_imr2::EXTI_IMR2_SPEC>;
#[doc = "Contains register bits for configurable events and direct events."]
pub mod exti_imr2;
#[doc = "EXTI_EMR2 register accessor: an alias for `Reg<EXTI_EMR2_SPEC>`"]
pub type EXTI_EMR2 = crate::Reg<exti_emr2::EXTI_EMR2_SPEC>;
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod exti_emr2;
#[doc = "EXTI_IMR3 register accessor: an alias for `Reg<EXTI_IMR3_SPEC>`"]
pub type EXTI_IMR3 = crate::Reg<exti_imr3::EXTI_IMR3_SPEC>;
#[doc = "Contains register bits for configurable events and direct events."]
pub mod exti_imr3;
#[doc = "EXTI_EMR3 register accessor: an alias for `Reg<EXTI_EMR3_SPEC>`"]
pub type EXTI_EMR3 = crate::Reg<exti_emr3::EXTI_EMR3_SPEC>;
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod exti_emr3;
#[doc = "EXTI_C2IMR1 register accessor: an alias for `Reg<EXTI_C2IMR1_SPEC>`"]
pub type EXTI_C2IMR1 = crate::Reg<exti_c2imr1::EXTI_C2IMR1_SPEC>;
#[doc = "Contains register bits for configurable events and Direct events."]
pub mod exti_c2imr1;
#[doc = "EXTI_C2EMR1 register accessor: an alias for `Reg<EXTI_C2EMR1_SPEC>`"]
pub type EXTI_C2EMR1 = crate::Reg<exti_c2emr1::EXTI_C2EMR1_SPEC>;
#[doc = "EXTI CPU2 wakeup with event mask register"]
pub mod exti_c2emr1;
#[doc = "EXTI_C2IMR2 register accessor: an alias for `Reg<EXTI_C2IMR2_SPEC>`"]
pub type EXTI_C2IMR2 = crate::Reg<exti_c2imr2::EXTI_C2IMR2_SPEC>;
#[doc = "Contains register bits for configurable events and direct events."]
pub mod exti_c2imr2;
#[doc = "EXTI_C2EMR2 register accessor: an alias for `Reg<EXTI_C2EMR2_SPEC>`"]
pub type EXTI_C2EMR2 = crate::Reg<exti_c2emr2::EXTI_C2EMR2_SPEC>;
#[doc = "EXTI CPU2 wakeup with event mask register"]
pub mod exti_c2emr2;
#[doc = "EXTI_C2IMR3 register accessor: an alias for `Reg<EXTI_C2IMR3_SPEC>`"]
pub type EXTI_C2IMR3 = crate::Reg<exti_c2imr3::EXTI_C2IMR3_SPEC>;
#[doc = "Contains register bits for configurable events and direct events."]
pub mod exti_c2imr3;
#[doc = "EXTI_C2EMR3 register accessor: an alias for `Reg<EXTI_C2EMR3_SPEC>`"]
pub type EXTI_C2EMR3 = crate::Reg<exti_c2emr3::EXTI_C2EMR3_SPEC>;
#[doc = "EXTI CPU2 wakeup with event mask register"]
pub mod exti_c2emr3;
#[doc = "EXTI_HWCFGR13 register accessor: an alias for `Reg<EXTI_HWCFGR13_SPEC>`"]
pub type EXTI_HWCFGR13 = crate::Reg<exti_hwcfgr13::EXTI_HWCFGR13_SPEC>;
#[doc = "EXTI hardware configuration register 13"]
pub mod exti_hwcfgr13;
#[doc = "EXTI_HWCFGR12 register accessor: an alias for `Reg<EXTI_HWCFGR12_SPEC>`"]
pub type EXTI_HWCFGR12 = crate::Reg<exti_hwcfgr12::EXTI_HWCFGR12_SPEC>;
#[doc = "EXTI hardware configuration register 12"]
pub mod exti_hwcfgr12;
#[doc = "EXTI_HWCFGR11 register accessor: an alias for `Reg<EXTI_HWCFGR11_SPEC>`"]
pub type EXTI_HWCFGR11 = crate::Reg<exti_hwcfgr11::EXTI_HWCFGR11_SPEC>;
#[doc = "EXTI hardware configuration register 11"]
pub mod exti_hwcfgr11;
#[doc = "EXTI_HWCFGR10 register accessor: an alias for `Reg<EXTI_HWCFGR10_SPEC>`"]
pub type EXTI_HWCFGR10 = crate::Reg<exti_hwcfgr10::EXTI_HWCFGR10_SPEC>;
#[doc = "EXTI hardware configuration register 10"]
pub mod exti_hwcfgr10;
#[doc = "EXTI_HWCFGR9 register accessor: an alias for `Reg<EXTI_HWCFGR9_SPEC>`"]
pub type EXTI_HWCFGR9 = crate::Reg<exti_hwcfgr9::EXTI_HWCFGR9_SPEC>;
#[doc = "EXTI hardware configuration register 9"]
pub mod exti_hwcfgr9;
#[doc = "EXTI_HWCFGR8 register accessor: an alias for `Reg<EXTI_HWCFGR8_SPEC>`"]
pub type EXTI_HWCFGR8 = crate::Reg<exti_hwcfgr8::EXTI_HWCFGR8_SPEC>;
#[doc = "EXTI hardware configuration register 8"]
pub mod exti_hwcfgr8;
#[doc = "EXTI_HWCFGR7 register accessor: an alias for `Reg<EXTI_HWCFGR7_SPEC>`"]
pub type EXTI_HWCFGR7 = crate::Reg<exti_hwcfgr7::EXTI_HWCFGR7_SPEC>;
#[doc = "EXTI hardware configuration register 7"]
pub mod exti_hwcfgr7;
#[doc = "EXTI_HWCFGR6 register accessor: an alias for `Reg<EXTI_HWCFGR6_SPEC>`"]
pub type EXTI_HWCFGR6 = crate::Reg<exti_hwcfgr6::EXTI_HWCFGR6_SPEC>;
#[doc = "EXTI hardware configuration register 6"]
pub mod exti_hwcfgr6;
#[doc = "EXTI_HWCFGR5 register accessor: an alias for `Reg<EXTI_HWCFGR5_SPEC>`"]
pub type EXTI_HWCFGR5 = crate::Reg<exti_hwcfgr5::EXTI_HWCFGR5_SPEC>;
#[doc = "EXTI hardware configuration register 5"]
pub mod exti_hwcfgr5;
#[doc = "EXTI_HWCFGR4 register accessor: an alias for `Reg<EXTI_HWCFGR4_SPEC>`"]
pub type EXTI_HWCFGR4 = crate::Reg<exti_hwcfgr4::EXTI_HWCFGR4_SPEC>;
#[doc = "EXTI hardware configuration register 4"]
pub mod exti_hwcfgr4;
#[doc = "EXTI_HWCFGR3 register accessor: an alias for `Reg<EXTI_HWCFGR3_SPEC>`"]
pub type EXTI_HWCFGR3 = crate::Reg<exti_hwcfgr3::EXTI_HWCFGR3_SPEC>;
#[doc = "EXTI hardware configuration register 3"]
pub mod exti_hwcfgr3;
#[doc = "EXTI_HWCFGR2 register accessor: an alias for `Reg<EXTI_HWCFGR2_SPEC>`"]
pub type EXTI_HWCFGR2 = crate::Reg<exti_hwcfgr2::EXTI_HWCFGR2_SPEC>;
#[doc = "EXTI hardware configuration register 2"]
pub mod exti_hwcfgr2;
#[doc = "EXTI_HWCFGR1 register accessor: an alias for `Reg<EXTI_HWCFGR1_SPEC>`"]
pub type EXTI_HWCFGR1 = crate::Reg<exti_hwcfgr1::EXTI_HWCFGR1_SPEC>;
#[doc = "EXTI hardware configuration register 1"]
pub mod exti_hwcfgr1;
#[doc = "EXTI_VERR register accessor: an alias for `Reg<EXTI_VERR_SPEC>`"]
pub type EXTI_VERR = crate::Reg<exti_verr::EXTI_VERR_SPEC>;
#[doc = "EXTI IP version register"]
pub mod exti_verr;
#[doc = "EXTI_IPIDR register accessor: an alias for `Reg<EXTI_IPIDR_SPEC>`"]
pub type EXTI_IPIDR = crate::Reg<exti_ipidr::EXTI_IPIDR_SPEC>;
#[doc = "EXTI identification register"]
pub mod exti_ipidr;
#[doc = "EXTI_SIDR register accessor: an alias for `Reg<EXTI_SIDR_SPEC>`"]
pub type EXTI_SIDR = crate::Reg<exti_sidr::EXTI_SIDR_SPEC>;
#[doc = "EXTI size ID register"]
pub mod exti_sidr;
