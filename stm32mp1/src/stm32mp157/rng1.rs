#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RNG control register"]
    pub rng_cr: crate::Reg<rng_cr::RNG_CR_SPEC>,
    #[doc = "0x04 - RNG status register"]
    pub rng_sr: crate::Reg<rng_sr::RNG_SR_SPEC>,
    #[doc = "0x08 - The RNG_DR register is a read-only register."]
    pub rng_dr: crate::Reg<rng_dr::RNG_DR_SPEC>,
    _reserved3: [u8; 0x03e4],
    #[doc = "0x3f0 - RNG hardware configuration register"]
    pub rng_hwcfgr: crate::Reg<rng_hwcfgr::RNG_HWCFGR_SPEC>,
    #[doc = "0x3f4 - RNG version register"]
    pub rng_verr: crate::Reg<rng_verr::RNG_VERR_SPEC>,
    #[doc = "0x3f8 - RNG identification register"]
    pub rng_ipidr: crate::Reg<rng_ipidr::RNG_IPIDR_SPEC>,
    #[doc = "0x3fc - RNG size ID register"]
    pub rng_sidr: crate::Reg<rng_sidr::RNG_SIDR_SPEC>,
}
#[doc = "RNG_CR register accessor: an alias for `Reg<RNG_CR_SPEC>`"]
pub type RNG_CR = crate::Reg<rng_cr::RNG_CR_SPEC>;
#[doc = "RNG control register"]
pub mod rng_cr;
#[doc = "RNG_SR register accessor: an alias for `Reg<RNG_SR_SPEC>`"]
pub type RNG_SR = crate::Reg<rng_sr::RNG_SR_SPEC>;
#[doc = "RNG status register"]
pub mod rng_sr;
#[doc = "RNG_DR register accessor: an alias for `Reg<RNG_DR_SPEC>`"]
pub type RNG_DR = crate::Reg<rng_dr::RNG_DR_SPEC>;
#[doc = "The RNG_DR register is a read-only register."]
pub mod rng_dr;
#[doc = "RNG_HWCFGR register accessor: an alias for `Reg<RNG_HWCFGR_SPEC>`"]
pub type RNG_HWCFGR = crate::Reg<rng_hwcfgr::RNG_HWCFGR_SPEC>;
#[doc = "RNG hardware configuration register"]
pub mod rng_hwcfgr;
#[doc = "RNG_VERR register accessor: an alias for `Reg<RNG_VERR_SPEC>`"]
pub type RNG_VERR = crate::Reg<rng_verr::RNG_VERR_SPEC>;
#[doc = "RNG version register"]
pub mod rng_verr;
#[doc = "RNG_IPIDR register accessor: an alias for `Reg<RNG_IPIDR_SPEC>`"]
pub type RNG_IPIDR = crate::Reg<rng_ipidr::RNG_IPIDR_SPEC>;
#[doc = "RNG identification register"]
pub mod rng_ipidr;
#[doc = "RNG_SIDR register accessor: an alias for `Reg<RNG_SIDR_SPEC>`"]
pub type RNG_SIDR = crate::Reg<rng_sidr::RNG_SIDR_SPEC>;
#[doc = "RNG size ID register"]
pub mod rng_sidr;
