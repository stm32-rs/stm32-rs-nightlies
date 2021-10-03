#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN_MCR"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x04 - CAN_MSR"]
    pub msr: crate::Reg<msr::MSR_SPEC>,
    #[doc = "0x08 - CAN_TSR"]
    pub tsr: crate::Reg<tsr::TSR_SPEC>,
    #[doc = "0x0c..0x14 - CAN_RF%sR"]
    pub rfr: [crate::Reg<rfr::RFR_SPEC>; 2],
    #[doc = "0x14 - CAN_IER"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x18 - CAN_ESR"]
    pub esr: crate::Reg<esr::ESR_SPEC>,
    #[doc = "0x1c - CAN_BTR"]
    pub btr: crate::Reg<btr::BTR_SPEC>,
    _reserved7: [u8; 0x0160],
    #[doc = "0x180..0x1b0 - CAN Transmit cluster"]
    pub tx: [TX; 3],
    #[doc = "0x1b0..0x1d0 - CAN Receive cluster"]
    pub rx: [RX; 2],
    _reserved9: [u8; 0x30],
    #[doc = "0x200 - CAN_FMR"]
    pub fmr: crate::Reg<fmr::FMR_SPEC>,
    #[doc = "0x204 - CAN_FM1R"]
    pub fm1r: crate::Reg<fm1r::FM1R_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x20c - CAN_FS1R"]
    pub fs1r: crate::Reg<fs1r::FS1R_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x214 - CAN_FFA1R"]
    pub ffa1r: crate::Reg<ffa1r::FFA1R_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x21c - CAN_FA1R"]
    pub fa1r: crate::Reg<fa1r::FA1R_SPEC>,
    _reserved14: [u8; 0x20],
    #[doc = "0x240..0x320 - CAN Filter Bank cluster"]
    pub fb: [FB; 28],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TX {
    #[doc = "0x00 - CAN_TI0R"]
    pub tir: crate::Reg<self::tx::tir::TIR_SPEC>,
    #[doc = "0x04 - CAN_TDT0R"]
    pub tdtr: crate::Reg<self::tx::tdtr::TDTR_SPEC>,
    #[doc = "0x08 - CAN_TDL0R"]
    pub tdlr: crate::Reg<self::tx::tdlr::TDLR_SPEC>,
    #[doc = "0x0c - CAN_TDH0R"]
    pub tdhr: crate::Reg<self::tx::tdhr::TDHR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CAN Transmit cluster"]
pub mod tx;
#[doc = r"Register block"]
#[repr(C)]
pub struct RX {
    #[doc = "0x00 - CAN_RI0R"]
    pub rir: crate::Reg<self::rx::rir::RIR_SPEC>,
    #[doc = "0x04 - CAN_RDT0R"]
    pub rdtr: crate::Reg<self::rx::rdtr::RDTR_SPEC>,
    #[doc = "0x08 - CAN_RDL0R"]
    pub rdlr: crate::Reg<self::rx::rdlr::RDLR_SPEC>,
    #[doc = "0x0c - CAN_RDH0R"]
    pub rdhr: crate::Reg<self::rx::rdhr::RDHR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CAN Receive cluster"]
pub mod rx;
#[doc = r"Register block"]
#[repr(C)]
pub struct FB {
    #[doc = "0x00 - Filter bank 0 register 1"]
    pub fr1: crate::Reg<self::fb::fr1::FR1_SPEC>,
    #[doc = "0x04 - Filter bank 0 register 2"]
    pub fr2: crate::Reg<self::fb::fr2::FR2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CAN Filter Bank cluster"]
pub mod fb;
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "CAN_MCR"]
pub mod mcr;
#[doc = "MSR register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "CAN_MSR"]
pub mod msr;
#[doc = "TSR register accessor: an alias for `Reg<TSR_SPEC>`"]
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
#[doc = "CAN_TSR"]
pub mod tsr;
#[doc = "RFR register accessor: an alias for `Reg<RFR_SPEC>`"]
pub type RFR = crate::Reg<rfr::RFR_SPEC>;
#[doc = "CAN_RF%sR"]
pub mod rfr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "CAN_IER"]
pub mod ier;
#[doc = "ESR register accessor: an alias for `Reg<ESR_SPEC>`"]
pub type ESR = crate::Reg<esr::ESR_SPEC>;
#[doc = "CAN_ESR"]
pub mod esr;
#[doc = "BTR register accessor: an alias for `Reg<BTR_SPEC>`"]
pub type BTR = crate::Reg<btr::BTR_SPEC>;
#[doc = "CAN_BTR"]
pub mod btr;
#[doc = "FMR register accessor: an alias for `Reg<FMR_SPEC>`"]
pub type FMR = crate::Reg<fmr::FMR_SPEC>;
#[doc = "CAN_FMR"]
pub mod fmr;
#[doc = "FM1R register accessor: an alias for `Reg<FM1R_SPEC>`"]
pub type FM1R = crate::Reg<fm1r::FM1R_SPEC>;
#[doc = "CAN_FM1R"]
pub mod fm1r;
#[doc = "FS1R register accessor: an alias for `Reg<FS1R_SPEC>`"]
pub type FS1R = crate::Reg<fs1r::FS1R_SPEC>;
#[doc = "CAN_FS1R"]
pub mod fs1r;
#[doc = "FFA1R register accessor: an alias for `Reg<FFA1R_SPEC>`"]
pub type FFA1R = crate::Reg<ffa1r::FFA1R_SPEC>;
#[doc = "CAN_FFA1R"]
pub mod ffa1r;
#[doc = "FA1R register accessor: an alias for `Reg<FA1R_SPEC>`"]
pub type FA1R = crate::Reg<fa1r::FA1R_SPEC>;
#[doc = "CAN_FA1R"]
pub mod fa1r;
