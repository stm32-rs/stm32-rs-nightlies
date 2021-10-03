#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - interrupt enable register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x08 - interrupt clear register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x0c - interrupt status register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x10 - I/O hysteresis control register"]
    pub iohcr: crate::Reg<iohcr::IOHCR_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - I/O analog switch control register"]
    pub ioascr: crate::Reg<ioascr::IOASCR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - I/O sampling control register"]
    pub ioscr: crate::Reg<ioscr::IOSCR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x28 - I/O channel control register"]
    pub ioccr: crate::Reg<ioccr::IOCCR_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x30 - I/O group control status register"]
    pub iogcsr: crate::Reg<iogcsr::IOGCSR_SPEC>,
    #[doc = "0x34 - I/O group x counter register"]
    pub iog1cr: crate::Reg<iog1cr::IOG1CR_SPEC>,
    #[doc = "0x38 - I/O group x counter register"]
    pub iog2cr: crate::Reg<iog2cr::IOG2CR_SPEC>,
    #[doc = "0x3c - I/O group x counter register"]
    pub iog3cr: crate::Reg<iog3cr::IOG3CR_SPEC>,
    #[doc = "0x40 - I/O group x counter register"]
    pub iog4cr: crate::Reg<iog4cr::IOG4CR_SPEC>,
    #[doc = "0x44 - I/O group x counter register"]
    pub iog5cr: crate::Reg<iog5cr::IOG5CR_SPEC>,
    #[doc = "0x48 - I/O group x counter register"]
    pub iog6cr: crate::Reg<iog6cr::IOG6CR_SPEC>,
    #[doc = "0x4c - I/O group x counter register"]
    pub iog7cr: crate::Reg<iog7cr::IOG7CR_SPEC>,
    #[doc = "0x50 - I/O group x counter register"]
    pub iog8cr: crate::Reg<iog8cr::IOG8CR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "interrupt status register"]
pub mod isr;
#[doc = "IOHCR register accessor: an alias for `Reg<IOHCR_SPEC>`"]
pub type IOHCR = crate::Reg<iohcr::IOHCR_SPEC>;
#[doc = "I/O hysteresis control register"]
pub mod iohcr;
#[doc = "IOASCR register accessor: an alias for `Reg<IOASCR_SPEC>`"]
pub type IOASCR = crate::Reg<ioascr::IOASCR_SPEC>;
#[doc = "I/O analog switch control register"]
pub mod ioascr;
#[doc = "IOSCR register accessor: an alias for `Reg<IOSCR_SPEC>`"]
pub type IOSCR = crate::Reg<ioscr::IOSCR_SPEC>;
#[doc = "I/O sampling control register"]
pub mod ioscr;
#[doc = "IOCCR register accessor: an alias for `Reg<IOCCR_SPEC>`"]
pub type IOCCR = crate::Reg<ioccr::IOCCR_SPEC>;
#[doc = "I/O channel control register"]
pub mod ioccr;
#[doc = "IOGCSR register accessor: an alias for `Reg<IOGCSR_SPEC>`"]
pub type IOGCSR = crate::Reg<iogcsr::IOGCSR_SPEC>;
#[doc = "I/O group control status register"]
pub mod iogcsr;
#[doc = "IOG1CR register accessor: an alias for `Reg<IOG1CR_SPEC>`"]
pub type IOG1CR = crate::Reg<iog1cr::IOG1CR_SPEC>;
#[doc = "I/O group x counter register"]
pub mod iog1cr;
#[doc = "IOG2CR register accessor: an alias for `Reg<IOG2CR_SPEC>`"]
pub type IOG2CR = crate::Reg<iog2cr::IOG2CR_SPEC>;
#[doc = "I/O group x counter register"]
pub mod iog2cr;
#[doc = "IOG3CR register accessor: an alias for `Reg<IOG3CR_SPEC>`"]
pub type IOG3CR = crate::Reg<iog3cr::IOG3CR_SPEC>;
#[doc = "I/O group x counter register"]
pub mod iog3cr;
#[doc = "IOG4CR register accessor: an alias for `Reg<IOG4CR_SPEC>`"]
pub type IOG4CR = crate::Reg<iog4cr::IOG4CR_SPEC>;
#[doc = "I/O group x counter register"]
pub mod iog4cr;
#[doc = "IOG5CR register accessor: an alias for `Reg<IOG5CR_SPEC>`"]
pub type IOG5CR = crate::Reg<iog5cr::IOG5CR_SPEC>;
#[doc = "I/O group x counter register"]
pub mod iog5cr;
#[doc = "IOG6CR register accessor: an alias for `Reg<IOG6CR_SPEC>`"]
pub type IOG6CR = crate::Reg<iog6cr::IOG6CR_SPEC>;
#[doc = "I/O group x counter register"]
pub mod iog6cr;
#[doc = "IOG7CR register accessor: an alias for `Reg<IOG7CR_SPEC>`"]
pub type IOG7CR = crate::Reg<iog7cr::IOG7CR_SPEC>;
#[doc = "I/O group x counter register"]
pub mod iog7cr;
#[doc = "IOG8CR register accessor: an alias for `Reg<IOG8CR_SPEC>`"]
pub type IOG8CR = crate::Reg<iog8cr::IOG8CR_SPEC>;
#[doc = "I/O group x counter register"]
pub mod iog8cr;
