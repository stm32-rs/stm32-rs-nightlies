#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RNG control register"]
    pub rng_cr: crate::Reg<rng_cr::RNG_CR_SPEC>,
    #[doc = "0x04 - RNG status register"]
    pub rng_sr: crate::Reg<rng_sr::RNG_SR_SPEC>,
    #[doc = "0x08 - The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0."]
    pub rng_dr: crate::Reg<rng_dr::RNG_DR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0."]
    pub rng_htcr: crate::Reg<rng_htcr::RNG_HTCR_SPEC>,
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
#[doc = "The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0."]
pub mod rng_dr;
#[doc = "RNG_HTCR register accessor: an alias for `Reg<RNG_HTCR_SPEC>`"]
pub type RNG_HTCR = crate::Reg<rng_htcr::RNG_HTCR_SPEC>;
#[doc = "The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0."]
pub mod rng_htcr;
