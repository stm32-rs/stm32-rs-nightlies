#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timbcr: crate::Reg<timbcr::TIMBCR_SPEC>,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timbisr: crate::Reg<timbisr::TIMBISR_SPEC>,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timbicr: crate::Reg<timbicr::TIMBICR_SPEC>,
    #[doc = "0x0c - TIMxDIER5"]
    pub timbdier5: crate::Reg<timbdier5::TIMBDIER5_SPEC>,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntr: crate::Reg<cntr::CNTR_SPEC>,
    #[doc = "0x14 - Timerx Period Register"]
    pub perbr: crate::Reg<perbr::PERBR_SPEC>,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repbr: crate::Reg<repbr::REPBR_SPEC>,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1br: crate::Reg<cmp1br::CMP1BR_SPEC>,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1cbr: crate::Reg<cmp1cbr::CMP1CBR_SPEC>,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2br: crate::Reg<cmp2br::CMP2BR_SPEC>,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3br: crate::Reg<cmp3br::CMP3BR_SPEC>,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4br: crate::Reg<cmp4br::CMP4BR_SPEC>,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1br: crate::Reg<cpt1br::CPT1BR_SPEC>,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2br: crate::Reg<cpt2br::CPT2BR_SPEC>,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtbr: crate::Reg<dtbr::DTBR_SPEC>,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub setb1r: crate::Reg<setb1r::SETB1R_SPEC>,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rstb1r: crate::Reg<rstb1r::RSTB1R_SPEC>,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub setb2r: crate::Reg<setb2r::SETB2R_SPEC>,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rstb2r: crate::Reg<rstb2r::RSTB2R_SPEC>,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefbr1: crate::Reg<eefbr1::EEFBR1_SPEC>,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefbr2: crate::Reg<eefbr2::EEFBR2_SPEC>,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstbr: crate::Reg<rstbr::RSTBR_SPEC>,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpbr: crate::Reg<chpbr::CHPBR_SPEC>,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1bcr: crate::Reg<cpt1bcr::CPT1BCR_SPEC>,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2bcr: crate::Reg<cpt2bcr::CPT2BCR_SPEC>,
    #[doc = "0x64 - Timerx Output Register"]
    pub outbr: crate::Reg<outbr::OUTBR_SPEC>,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltbr: crate::Reg<fltbr::FLTBR_SPEC>,
}
#[doc = "TIMBCR register accessor: an alias for `Reg<TIMBCR_SPEC>`"]
pub type TIMBCR = crate::Reg<timbcr::TIMBCR_SPEC>;
#[doc = "Timerx Control Register"]
pub mod timbcr;
#[doc = "TIMBISR register accessor: an alias for `Reg<TIMBISR_SPEC>`"]
pub type TIMBISR = crate::Reg<timbisr::TIMBISR_SPEC>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timbisr;
#[doc = "TIMBICR register accessor: an alias for `Reg<TIMBICR_SPEC>`"]
pub type TIMBICR = crate::Reg<timbicr::TIMBICR_SPEC>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timbicr;
#[doc = "TIMBDIER5 register accessor: an alias for `Reg<TIMBDIER5_SPEC>`"]
pub type TIMBDIER5 = crate::Reg<timbdier5::TIMBDIER5_SPEC>;
#[doc = "TIMxDIER5"]
pub mod timbdier5;
#[doc = "CNTR register accessor: an alias for `Reg<CNTR_SPEC>`"]
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
#[doc = "Timerx Counter Register"]
pub mod cntr;
#[doc = "PERBR register accessor: an alias for `Reg<PERBR_SPEC>`"]
pub type PERBR = crate::Reg<perbr::PERBR_SPEC>;
#[doc = "Timerx Period Register"]
pub mod perbr;
#[doc = "REPBR register accessor: an alias for `Reg<REPBR_SPEC>`"]
pub type REPBR = crate::Reg<repbr::REPBR_SPEC>;
#[doc = "Timerx Repetition Register"]
pub mod repbr;
#[doc = "CMP1BR register accessor: an alias for `Reg<CMP1BR_SPEC>`"]
pub type CMP1BR = crate::Reg<cmp1br::CMP1BR_SPEC>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1br;
#[doc = "CMP1CBR register accessor: an alias for `Reg<CMP1CBR_SPEC>`"]
pub type CMP1CBR = crate::Reg<cmp1cbr::CMP1CBR_SPEC>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cbr;
#[doc = "CMP2BR register accessor: an alias for `Reg<CMP2BR_SPEC>`"]
pub type CMP2BR = crate::Reg<cmp2br::CMP2BR_SPEC>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2br;
#[doc = "CMP3BR register accessor: an alias for `Reg<CMP3BR_SPEC>`"]
pub type CMP3BR = crate::Reg<cmp3br::CMP3BR_SPEC>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3br;
#[doc = "CMP4BR register accessor: an alias for `Reg<CMP4BR_SPEC>`"]
pub type CMP4BR = crate::Reg<cmp4br::CMP4BR_SPEC>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4br;
#[doc = "CPT1BR register accessor: an alias for `Reg<CPT1BR_SPEC>`"]
pub type CPT1BR = crate::Reg<cpt1br::CPT1BR_SPEC>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1br;
#[doc = "CPT2BR register accessor: an alias for `Reg<CPT2BR_SPEC>`"]
pub type CPT2BR = crate::Reg<cpt2br::CPT2BR_SPEC>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2br;
#[doc = "DTBR register accessor: an alias for `Reg<DTBR_SPEC>`"]
pub type DTBR = crate::Reg<dtbr::DTBR_SPEC>;
#[doc = "Timerx Deadtime Register"]
pub mod dtbr;
#[doc = "SETB1R register accessor: an alias for `Reg<SETB1R_SPEC>`"]
pub type SETB1R = crate::Reg<setb1r::SETB1R_SPEC>;
#[doc = "Timerx Output1 Set Register"]
pub mod setb1r;
#[doc = "RSTB1R register accessor: an alias for `Reg<RSTB1R_SPEC>`"]
pub type RSTB1R = crate::Reg<rstb1r::RSTB1R_SPEC>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rstb1r;
#[doc = "SETB2R register accessor: an alias for `Reg<SETB2R_SPEC>`"]
pub type SETB2R = crate::Reg<setb2r::SETB2R_SPEC>;
#[doc = "Timerx Output2 Set Register"]
pub mod setb2r;
#[doc = "RSTB2R register accessor: an alias for `Reg<RSTB2R_SPEC>`"]
pub type RSTB2R = crate::Reg<rstb2r::RSTB2R_SPEC>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rstb2r;
#[doc = "EEFBR1 register accessor: an alias for `Reg<EEFBR1_SPEC>`"]
pub type EEFBR1 = crate::Reg<eefbr1::EEFBR1_SPEC>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefbr1;
#[doc = "EEFBR2 register accessor: an alias for `Reg<EEFBR2_SPEC>`"]
pub type EEFBR2 = crate::Reg<eefbr2::EEFBR2_SPEC>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefbr2;
#[doc = "RSTBR register accessor: an alias for `Reg<RSTBR_SPEC>`"]
pub type RSTBR = crate::Reg<rstbr::RSTBR_SPEC>;
#[doc = "TimerA Reset Register"]
pub mod rstbr;
#[doc = "CHPBR register accessor: an alias for `Reg<CHPBR_SPEC>`"]
pub type CHPBR = crate::Reg<chpbr::CHPBR_SPEC>;
#[doc = "Timerx Chopper Register"]
pub mod chpbr;
#[doc = "CPT1BCR register accessor: an alias for `Reg<CPT1BCR_SPEC>`"]
pub type CPT1BCR = crate::Reg<cpt1bcr::CPT1BCR_SPEC>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1bcr;
#[doc = "CPT2BCR register accessor: an alias for `Reg<CPT2BCR_SPEC>`"]
pub type CPT2BCR = crate::Reg<cpt2bcr::CPT2BCR_SPEC>;
#[doc = "CPT2xCR"]
pub mod cpt2bcr;
#[doc = "OUTBR register accessor: an alias for `Reg<OUTBR_SPEC>`"]
pub type OUTBR = crate::Reg<outbr::OUTBR_SPEC>;
#[doc = "Timerx Output Register"]
pub mod outbr;
#[doc = "FLTBR register accessor: an alias for `Reg<FLTBR_SPEC>`"]
pub type FLTBR = crate::Reg<fltbr::FLTBR_SPEC>;
#[doc = "Timerx Fault Register"]
pub mod fltbr;
