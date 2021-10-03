#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - AConfiguration register 1"]
    pub acr1: crate::Reg<acr1::ACR1_SPEC>,
    #[doc = "0x08 - AConfiguration register 2"]
    pub acr2: crate::Reg<acr2::ACR2_SPEC>,
    #[doc = "0x0c - AFRCR"]
    pub afrcr: crate::Reg<afrcr::AFRCR_SPEC>,
    #[doc = "0x10 - ASlot register"]
    pub aslotr: crate::Reg<aslotr::ASLOTR_SPEC>,
    #[doc = "0x14 - AInterrupt mask register2"]
    pub aim: crate::Reg<aim::AIM_SPEC>,
    #[doc = "0x18 - AStatus register"]
    pub asr: crate::Reg<asr::ASR_SPEC>,
    #[doc = "0x1c - AClear flag register"]
    pub aclrfr: crate::Reg<aclrfr::ACLRFR_SPEC>,
    #[doc = "0x20 - AData register"]
    pub adr: crate::Reg<adr::ADR_SPEC>,
    #[doc = "0x24 - BConfiguration register 1"]
    pub bcr1: crate::Reg<bcr1::BCR1_SPEC>,
    #[doc = "0x28 - BConfiguration register 2"]
    pub bcr2: crate::Reg<bcr2::BCR2_SPEC>,
    #[doc = "0x2c - BFRCR"]
    pub bfrcr: crate::Reg<bfrcr::BFRCR_SPEC>,
    #[doc = "0x30 - BSlot register"]
    pub bslotr: crate::Reg<bslotr::BSLOTR_SPEC>,
    #[doc = "0x34 - BInterrupt mask register2"]
    pub bim: crate::Reg<bim::BIM_SPEC>,
    #[doc = "0x38 - BStatus register"]
    pub bsr: crate::Reg<bsr::BSR_SPEC>,
    #[doc = "0x3c - BClear flag register"]
    pub bclrfr: crate::Reg<bclrfr::BCLRFR_SPEC>,
    #[doc = "0x40 - BData register"]
    pub bdr: crate::Reg<bdr::BDR_SPEC>,
    #[doc = "0x44 - PDM control register"]
    pub pdmcr: crate::Reg<pdmcr::PDMCR_SPEC>,
    #[doc = "0x48 - PDM delay register"]
    pub pdmdly: crate::Reg<pdmdly::PDMDLY_SPEC>,
}
#[doc = "BCR1 register accessor: an alias for `Reg<BCR1_SPEC>`"]
pub type BCR1 = crate::Reg<bcr1::BCR1_SPEC>;
#[doc = "BConfiguration register 1"]
pub mod bcr1;
#[doc = "BCR2 register accessor: an alias for `Reg<BCR2_SPEC>`"]
pub type BCR2 = crate::Reg<bcr2::BCR2_SPEC>;
#[doc = "BConfiguration register 2"]
pub mod bcr2;
#[doc = "BFRCR register accessor: an alias for `Reg<BFRCR_SPEC>`"]
pub type BFRCR = crate::Reg<bfrcr::BFRCR_SPEC>;
#[doc = "BFRCR"]
pub mod bfrcr;
#[doc = "BSLOTR register accessor: an alias for `Reg<BSLOTR_SPEC>`"]
pub type BSLOTR = crate::Reg<bslotr::BSLOTR_SPEC>;
#[doc = "BSlot register"]
pub mod bslotr;
#[doc = "BIM register accessor: an alias for `Reg<BIM_SPEC>`"]
pub type BIM = crate::Reg<bim::BIM_SPEC>;
#[doc = "BInterrupt mask register2"]
pub mod bim;
#[doc = "BSR register accessor: an alias for `Reg<BSR_SPEC>`"]
pub type BSR = crate::Reg<bsr::BSR_SPEC>;
#[doc = "BStatus register"]
pub mod bsr;
#[doc = "BCLRFR register accessor: an alias for `Reg<BCLRFR_SPEC>`"]
pub type BCLRFR = crate::Reg<bclrfr::BCLRFR_SPEC>;
#[doc = "BClear flag register"]
pub mod bclrfr;
#[doc = "BDR register accessor: an alias for `Reg<BDR_SPEC>`"]
pub type BDR = crate::Reg<bdr::BDR_SPEC>;
#[doc = "BData register"]
pub mod bdr;
#[doc = "ACR1 register accessor: an alias for `Reg<ACR1_SPEC>`"]
pub type ACR1 = crate::Reg<acr1::ACR1_SPEC>;
#[doc = "AConfiguration register 1"]
pub mod acr1;
#[doc = "ACR2 register accessor: an alias for `Reg<ACR2_SPEC>`"]
pub type ACR2 = crate::Reg<acr2::ACR2_SPEC>;
#[doc = "AConfiguration register 2"]
pub mod acr2;
#[doc = "AFRCR register accessor: an alias for `Reg<AFRCR_SPEC>`"]
pub type AFRCR = crate::Reg<afrcr::AFRCR_SPEC>;
#[doc = "AFRCR"]
pub mod afrcr;
#[doc = "ASLOTR register accessor: an alias for `Reg<ASLOTR_SPEC>`"]
pub type ASLOTR = crate::Reg<aslotr::ASLOTR_SPEC>;
#[doc = "ASlot register"]
pub mod aslotr;
#[doc = "AIM register accessor: an alias for `Reg<AIM_SPEC>`"]
pub type AIM = crate::Reg<aim::AIM_SPEC>;
#[doc = "AInterrupt mask register2"]
pub mod aim;
#[doc = "ASR register accessor: an alias for `Reg<ASR_SPEC>`"]
pub type ASR = crate::Reg<asr::ASR_SPEC>;
#[doc = "AStatus register"]
pub mod asr;
#[doc = "ACLRFR register accessor: an alias for `Reg<ACLRFR_SPEC>`"]
pub type ACLRFR = crate::Reg<aclrfr::ACLRFR_SPEC>;
#[doc = "AClear flag register"]
pub mod aclrfr;
#[doc = "ADR register accessor: an alias for `Reg<ADR_SPEC>`"]
pub type ADR = crate::Reg<adr::ADR_SPEC>;
#[doc = "AData register"]
pub mod adr;
#[doc = "PDMCR register accessor: an alias for `Reg<PDMCR_SPEC>`"]
pub type PDMCR = crate::Reg<pdmcr::PDMCR_SPEC>;
#[doc = "PDM control register"]
pub mod pdmcr;
#[doc = "PDMDLY register accessor: an alias for `Reg<PDMDLY_SPEC>`"]
pub type PDMDLY = crate::Reg<pdmdly::PDMDLY_SPEC>;
#[doc = "PDM delay register"]
pub mod pdmdly;
