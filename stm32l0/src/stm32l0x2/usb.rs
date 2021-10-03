#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - endpoint register"]
    pub ep0r: crate::Reg<ep0r::EP0R_SPEC>,
    #[doc = "0x04 - endpoint register"]
    pub ep1r: crate::Reg<ep1r::EP1R_SPEC>,
    #[doc = "0x08 - endpoint register"]
    pub ep2r: crate::Reg<ep2r::EP2R_SPEC>,
    #[doc = "0x0c - endpoint register"]
    pub ep3r: crate::Reg<ep3r::EP3R_SPEC>,
    #[doc = "0x10 - endpoint register"]
    pub ep4r: crate::Reg<ep4r::EP4R_SPEC>,
    #[doc = "0x14 - endpoint register"]
    pub ep5r: crate::Reg<ep5r::EP5R_SPEC>,
    #[doc = "0x18 - endpoint register"]
    pub ep6r: crate::Reg<ep6r::EP6R_SPEC>,
    #[doc = "0x1c - endpoint register"]
    pub ep7r: crate::Reg<ep7r::EP7R_SPEC>,
    _reserved8: [u8; 0x20],
    #[doc = "0x40 - control register"]
    pub cntr: crate::Reg<cntr::CNTR_SPEC>,
    #[doc = "0x44 - interrupt status register"]
    pub istr: crate::Reg<istr::ISTR_SPEC>,
    #[doc = "0x48 - frame number register"]
    pub fnr: crate::Reg<fnr::FNR_SPEC>,
    #[doc = "0x4c - device address"]
    pub daddr: crate::Reg<daddr::DADDR_SPEC>,
    #[doc = "0x50 - Buffer table address"]
    pub btable: crate::Reg<btable::BTABLE_SPEC>,
    #[doc = "0x54 - LPM control and status register"]
    pub lpmcsr: crate::Reg<lpmcsr::LPMCSR_SPEC>,
    #[doc = "0x58 - Battery charging detector"]
    pub bcdr: crate::Reg<bcdr::BCDR_SPEC>,
}
#[doc = "EP0R register accessor: an alias for `Reg<EP0R_SPEC>`"]
pub type EP0R = crate::Reg<ep0r::EP0R_SPEC>;
#[doc = "endpoint register"]
pub mod ep0r;
#[doc = "EP1R register accessor: an alias for `Reg<EP1R_SPEC>`"]
pub type EP1R = crate::Reg<ep1r::EP1R_SPEC>;
#[doc = "endpoint register"]
pub mod ep1r;
#[doc = "EP2R register accessor: an alias for `Reg<EP2R_SPEC>`"]
pub type EP2R = crate::Reg<ep2r::EP2R_SPEC>;
#[doc = "endpoint register"]
pub mod ep2r;
#[doc = "EP3R register accessor: an alias for `Reg<EP3R_SPEC>`"]
pub type EP3R = crate::Reg<ep3r::EP3R_SPEC>;
#[doc = "endpoint register"]
pub mod ep3r;
#[doc = "EP4R register accessor: an alias for `Reg<EP4R_SPEC>`"]
pub type EP4R = crate::Reg<ep4r::EP4R_SPEC>;
#[doc = "endpoint register"]
pub mod ep4r;
#[doc = "EP5R register accessor: an alias for `Reg<EP5R_SPEC>`"]
pub type EP5R = crate::Reg<ep5r::EP5R_SPEC>;
#[doc = "endpoint register"]
pub mod ep5r;
#[doc = "EP6R register accessor: an alias for `Reg<EP6R_SPEC>`"]
pub type EP6R = crate::Reg<ep6r::EP6R_SPEC>;
#[doc = "endpoint register"]
pub mod ep6r;
#[doc = "EP7R register accessor: an alias for `Reg<EP7R_SPEC>`"]
pub type EP7R = crate::Reg<ep7r::EP7R_SPEC>;
#[doc = "endpoint register"]
pub mod ep7r;
#[doc = "CNTR register accessor: an alias for `Reg<CNTR_SPEC>`"]
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
#[doc = "control register"]
pub mod cntr;
#[doc = "ISTR register accessor: an alias for `Reg<ISTR_SPEC>`"]
pub type ISTR = crate::Reg<istr::ISTR_SPEC>;
#[doc = "interrupt status register"]
pub mod istr;
#[doc = "FNR register accessor: an alias for `Reg<FNR_SPEC>`"]
pub type FNR = crate::Reg<fnr::FNR_SPEC>;
#[doc = "frame number register"]
pub mod fnr;
#[doc = "DADDR register accessor: an alias for `Reg<DADDR_SPEC>`"]
pub type DADDR = crate::Reg<daddr::DADDR_SPEC>;
#[doc = "device address"]
pub mod daddr;
#[doc = "BTABLE register accessor: an alias for `Reg<BTABLE_SPEC>`"]
pub type BTABLE = crate::Reg<btable::BTABLE_SPEC>;
#[doc = "Buffer table address"]
pub mod btable;
#[doc = "LPMCSR register accessor: an alias for `Reg<LPMCSR_SPEC>`"]
pub type LPMCSR = crate::Reg<lpmcsr::LPMCSR_SPEC>;
#[doc = "LPM control and status register"]
pub mod lpmcsr;
#[doc = "BCDR register accessor: an alias for `Reg<BCDR_SPEC>`"]
pub type BCDR = crate::Reg<bcdr::BCDR_SPEC>;
#[doc = "Battery charging detector"]
pub mod bcdr;
