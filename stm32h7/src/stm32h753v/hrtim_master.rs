#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Timer Control Register"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x04 - Master Timer Interrupt Status Register"]
    pub misr: crate::Reg<misr::MISR_SPEC>,
    #[doc = "0x08 - Master Timer Interrupt Clear Register"]
    pub micr: crate::Reg<micr::MICR_SPEC>,
    #[doc = "0x0c - MDIER4"]
    pub mdier4: crate::Reg<mdier4::MDIER4_SPEC>,
    #[doc = "0x10 - Master Timer Counter Register"]
    pub mcntr: crate::Reg<mcntr::MCNTR_SPEC>,
    #[doc = "0x14 - Master Timer Period Register"]
    pub mper: crate::Reg<mper::MPER_SPEC>,
    #[doc = "0x18 - Master Timer Repetition Register"]
    pub mrep: crate::Reg<mrep::MREP_SPEC>,
    #[doc = "0x1c - Master Timer Compare 1 Register"]
    pub mcmp1r: crate::Reg<mcmp1r::MCMP1R_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - Master Timer Compare 2 Register"]
    pub mcmp2r: crate::Reg<mcmp2r::MCMP2R_SPEC>,
    #[doc = "0x28 - Master Timer Compare 3 Register"]
    pub mcmp3r: crate::Reg<mcmp3r::MCMP3R_SPEC>,
    #[doc = "0x2c - Master Timer Compare 4 Register"]
    pub mcmp4r: crate::Reg<mcmp4r::MCMP4R_SPEC>,
}
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Master Timer Control Register"]
pub mod mcr;
#[doc = "MISR register accessor: an alias for `Reg<MISR_SPEC>`"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "Master Timer Interrupt Status Register"]
pub mod misr;
#[doc = "MICR register accessor: an alias for `Reg<MICR_SPEC>`"]
pub type MICR = crate::Reg<micr::MICR_SPEC>;
#[doc = "Master Timer Interrupt Clear Register"]
pub mod micr;
#[doc = "MDIER4 register accessor: an alias for `Reg<MDIER4_SPEC>`"]
pub type MDIER4 = crate::Reg<mdier4::MDIER4_SPEC>;
#[doc = "MDIER4"]
pub mod mdier4;
#[doc = "MCNTR register accessor: an alias for `Reg<MCNTR_SPEC>`"]
pub type MCNTR = crate::Reg<mcntr::MCNTR_SPEC>;
#[doc = "Master Timer Counter Register"]
pub mod mcntr;
#[doc = "MPER register accessor: an alias for `Reg<MPER_SPEC>`"]
pub type MPER = crate::Reg<mper::MPER_SPEC>;
#[doc = "Master Timer Period Register"]
pub mod mper;
#[doc = "MREP register accessor: an alias for `Reg<MREP_SPEC>`"]
pub type MREP = crate::Reg<mrep::MREP_SPEC>;
#[doc = "Master Timer Repetition Register"]
pub mod mrep;
#[doc = "MCMP1R register accessor: an alias for `Reg<MCMP1R_SPEC>`"]
pub type MCMP1R = crate::Reg<mcmp1r::MCMP1R_SPEC>;
#[doc = "Master Timer Compare 1 Register"]
pub mod mcmp1r;
#[doc = "MCMP2R register accessor: an alias for `Reg<MCMP2R_SPEC>`"]
pub type MCMP2R = crate::Reg<mcmp2r::MCMP2R_SPEC>;
#[doc = "Master Timer Compare 2 Register"]
pub mod mcmp2r;
#[doc = "MCMP3R register accessor: an alias for `Reg<MCMP3R_SPEC>`"]
pub type MCMP3R = crate::Reg<mcmp3r::MCMP3R_SPEC>;
#[doc = "Master Timer Compare 3 Register"]
pub mod mcmp3r;
#[doc = "MCMP4R register accessor: an alias for `Reg<MCMP4R_SPEC>`"]
pub type MCMP4R = crate::Reg<mcmp4r::MCMP4R_SPEC>;
#[doc = "Master Timer Compare 4 Register"]
pub mod mcmp4r;
