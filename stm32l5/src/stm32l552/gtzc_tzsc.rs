#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TZSC control register"]
    pub tzsc_cr: crate::Reg<tzsc_cr::TZSC_CR_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - TZSC secure configuration register 1"]
    pub tzsc_seccfgr1: crate::Reg<tzsc_seccfgr1::TZSC_SECCFGR1_SPEC>,
    #[doc = "0x14 - TZSC secure configuration register 2"]
    pub tzsc_seccfgr2: crate::Reg<tzsc_seccfgr2::TZSC_SECCFGR2_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x20 - TZSC privilege configuration register 1"]
    pub tzsc_privcfgr1: crate::Reg<tzsc_privcfgr1::TZSC_PRIVCFGR1_SPEC>,
    #[doc = "0x24 - TZSC privilege configuration register 2"]
    pub tzsc_privcfgr2: crate::Reg<tzsc_privcfgr2::TZSC_PRIVCFGR2_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x30 - TZSC external memory non-secure watermark register 1"]
    pub tzsc_mpcwm1_nswmr1: crate::Reg<tzsc_mpcwm1_nswmr1::TZSC_MPCWM1_NSWMR1_SPEC>,
    #[doc = "0x34 - TZSC external memory non-secure watermark register 1"]
    pub tzsc_mpcwm1_nswmr2: crate::Reg<tzsc_mpcwm1_nswmr2::TZSC_MPCWM1_NSWMR2_SPEC>,
    #[doc = "0x38 - TZSC external memory non-secure watermark register 1"]
    pub tzsc_mpcwm2_nswmr1: crate::Reg<tzsc_mpcwm2_nswmr1::TZSC_MPCWM2_NSWMR1_SPEC>,
    #[doc = "0x3c - TZSC external memory non-secure watermark register 2"]
    pub tzsc_mpcwm2_nswmr2: crate::Reg<tzsc_mpcwm2_nswmr2::TZSC_MPCWM2_NSWMR2_SPEC>,
    #[doc = "0x40 - TZSC external memory non-secure watermark register 2"]
    pub tzsc_mpcwm3_nswmr1: crate::Reg<tzsc_mpcwm3_nswmr1::TZSC_MPCWM3_NSWMR1_SPEC>,
}
#[doc = "TZSC_CR register accessor: an alias for `Reg<TZSC_CR_SPEC>`"]
pub type TZSC_CR = crate::Reg<tzsc_cr::TZSC_CR_SPEC>;
#[doc = "TZSC control register"]
pub mod tzsc_cr;
#[doc = "TZSC_SECCFGR1 register accessor: an alias for `Reg<TZSC_SECCFGR1_SPEC>`"]
pub type TZSC_SECCFGR1 = crate::Reg<tzsc_seccfgr1::TZSC_SECCFGR1_SPEC>;
#[doc = "TZSC secure configuration register 1"]
pub mod tzsc_seccfgr1;
#[doc = "TZSC_SECCFGR2 register accessor: an alias for `Reg<TZSC_SECCFGR2_SPEC>`"]
pub type TZSC_SECCFGR2 = crate::Reg<tzsc_seccfgr2::TZSC_SECCFGR2_SPEC>;
#[doc = "TZSC secure configuration register 2"]
pub mod tzsc_seccfgr2;
#[doc = "TZSC_PRIVCFGR1 register accessor: an alias for `Reg<TZSC_PRIVCFGR1_SPEC>`"]
pub type TZSC_PRIVCFGR1 = crate::Reg<tzsc_privcfgr1::TZSC_PRIVCFGR1_SPEC>;
#[doc = "TZSC privilege configuration register 1"]
pub mod tzsc_privcfgr1;
#[doc = "TZSC_PRIVCFGR2 register accessor: an alias for `Reg<TZSC_PRIVCFGR2_SPEC>`"]
pub type TZSC_PRIVCFGR2 = crate::Reg<tzsc_privcfgr2::TZSC_PRIVCFGR2_SPEC>;
#[doc = "TZSC privilege configuration register 2"]
pub mod tzsc_privcfgr2;
#[doc = "TZSC_MPCWM1_NSWMR1 register accessor: an alias for `Reg<TZSC_MPCWM1_NSWMR1_SPEC>`"]
pub type TZSC_MPCWM1_NSWMR1 = crate::Reg<tzsc_mpcwm1_nswmr1::TZSC_MPCWM1_NSWMR1_SPEC>;
#[doc = "TZSC external memory non-secure watermark register 1"]
pub mod tzsc_mpcwm1_nswmr1;
#[doc = "TZSC_MPCWM1_NSWMR2 register accessor: an alias for `Reg<TZSC_MPCWM1_NSWMR2_SPEC>`"]
pub type TZSC_MPCWM1_NSWMR2 = crate::Reg<tzsc_mpcwm1_nswmr2::TZSC_MPCWM1_NSWMR2_SPEC>;
#[doc = "TZSC external memory non-secure watermark register 1"]
pub mod tzsc_mpcwm1_nswmr2;
#[doc = "TZSC_MPCWM2_NSWMR1 register accessor: an alias for `Reg<TZSC_MPCWM2_NSWMR1_SPEC>`"]
pub type TZSC_MPCWM2_NSWMR1 = crate::Reg<tzsc_mpcwm2_nswmr1::TZSC_MPCWM2_NSWMR1_SPEC>;
#[doc = "TZSC external memory non-secure watermark register 1"]
pub mod tzsc_mpcwm2_nswmr1;
#[doc = "TZSC_MPCWM3_NSWMR1 register accessor: an alias for `Reg<TZSC_MPCWM3_NSWMR1_SPEC>`"]
pub type TZSC_MPCWM3_NSWMR1 = crate::Reg<tzsc_mpcwm3_nswmr1::TZSC_MPCWM3_NSWMR1_SPEC>;
#[doc = "TZSC external memory non-secure watermark register 2"]
pub mod tzsc_mpcwm3_nswmr1;
#[doc = "TZSC_MPCWM2_NSWMR2 register accessor: an alias for `Reg<TZSC_MPCWM2_NSWMR2_SPEC>`"]
pub type TZSC_MPCWM2_NSWMR2 = crate::Reg<tzsc_mpcwm2_nswmr2::TZSC_MPCWM2_NSWMR2_SPEC>;
#[doc = "TZSC external memory non-secure watermark register 2"]
pub mod tzsc_mpcwm2_nswmr2;
