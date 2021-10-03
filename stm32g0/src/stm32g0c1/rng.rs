#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub rng_cr: crate::Reg<rng_cr::RNG_CR_SPEC>,
    #[doc = "0x04 - status register"]
    pub rng_sr: crate::Reg<rng_sr::RNG_SR_SPEC>,
    #[doc = "0x08 - data register"]
    pub rng_dr: crate::Reg<rng_dr::RNG_DR_SPEC>,
}
#[doc = "RNG_CR register accessor: an alias for `Reg<RNG_CR_SPEC>`"]
pub type RNG_CR = crate::Reg<rng_cr::RNG_CR_SPEC>;
#[doc = "control register"]
pub mod rng_cr;
#[doc = "RNG_SR register accessor: an alias for `Reg<RNG_SR_SPEC>`"]
pub type RNG_SR = crate::Reg<rng_sr::RNG_SR_SPEC>;
#[doc = "status register"]
pub mod rng_sr;
#[doc = "RNG_DR register accessor: an alias for `Reg<RNG_DR_SPEC>`"]
pub type RNG_DR = crate::Reg<rng_dr::RNG_DR_SPEC>;
#[doc = "data register"]
pub mod rng_dr;
