#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - endpoint %s register"]
    pub epr: [crate::Reg<epr::EPR_SPEC>; 8],
    _reserved1: [u8; 0x20],
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
#[doc = "EPR register accessor: an alias for `Reg<EPR_SPEC>`"]
pub type EPR = crate::Reg<epr::EPR_SPEC>;
#[doc = "endpoint %s register"]
pub mod epr;
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
