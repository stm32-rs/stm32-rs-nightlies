#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OctoSPI IO Manager Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - OctoSPI IO Manager Port 1 configuration register"]
    pub p1cr: crate::Reg<p1cr::P1CR_SPEC>,
    #[doc = "0x08 - OctoSPI IO Manager Port 2 configuration register"]
    pub p2cr: crate::Reg<p2cr::P2CR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "OctoSPI IO Manager Control Register"]
pub mod cr;
#[doc = "P1CR register accessor: an alias for `Reg<P1CR_SPEC>`"]
pub type P1CR = crate::Reg<p1cr::P1CR_SPEC>;
#[doc = "OctoSPI IO Manager Port 1 configuration register"]
pub mod p1cr;
#[doc = "P2CR register accessor: an alias for `Reg<P2CR_SPEC>`"]
pub type P2CR = crate::Reg<p2cr::P2CR_SPEC>;
#[doc = "OctoSPI IO Manager Port 2 configuration register"]
pub mod p2cr;
