#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x04 - control register 2"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    #[doc = "0x08 - control register 3"]
    pub cr3: crate::Reg<cr3::CR3_SPEC>,
    #[doc = "0x0c - baud rate register"]
    pub brr: crate::Reg<brr::BRR_SPEC>,
    #[doc = "0x10 - guard time and prescaler register"]
    pub gtpr: crate::Reg<gtpr::GTPR_SPEC>,
    #[doc = "0x14 - receiver timeout register"]
    pub rtor: crate::Reg<rtor::RTOR_SPEC>,
    #[doc = "0x18 - request register"]
    pub rqr: crate::Reg<rqr::RQR_SPEC>,
    #[doc = "0x1c - interrupt and status register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x20 - interrupt flag clear register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x24 - receive data register"]
    pub rdr: crate::Reg<rdr::RDR_SPEC>,
    #[doc = "0x28 - transmit data register"]
    pub tdr: crate::Reg<tdr::TDR_SPEC>,
    #[doc = "0x2c - prescaler register"]
    pub presc: crate::Reg<presc::PRESC_SPEC>,
}
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "CR3 register accessor: an alias for `Reg<CR3_SPEC>`"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "control register 3"]
pub mod cr3;
#[doc = "BRR register accessor: an alias for `Reg<BRR_SPEC>`"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "baud rate register"]
pub mod brr;
#[doc = "GTPR register accessor: an alias for `Reg<GTPR_SPEC>`"]
pub type GTPR = crate::Reg<gtpr::GTPR_SPEC>;
#[doc = "guard time and prescaler register"]
pub mod gtpr;
#[doc = "RTOR register accessor: an alias for `Reg<RTOR_SPEC>`"]
pub type RTOR = crate::Reg<rtor::RTOR_SPEC>;
#[doc = "receiver timeout register"]
pub mod rtor;
#[doc = "RQR register accessor: an alias for `Reg<RQR_SPEC>`"]
pub type RQR = crate::Reg<rqr::RQR_SPEC>;
#[doc = "request register"]
pub mod rqr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod isr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod icr;
#[doc = "RDR register accessor: an alias for `Reg<RDR_SPEC>`"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "receive data register"]
pub mod rdr;
#[doc = "TDR register accessor: an alias for `Reg<TDR_SPEC>`"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "transmit data register"]
pub mod tdr;
#[doc = "PRESC register accessor: an alias for `Reg<PRESC_SPEC>`"]
pub type PRESC = crate::Reg<presc::PRESC_SPEC>;
#[doc = "prescaler register"]
pub mod presc;
