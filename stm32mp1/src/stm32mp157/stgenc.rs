#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - STGENC control register"]
    pub stgenc_cntcr: crate::Reg<stgenc_cntcr::STGENC_CNTCR_SPEC>,
    #[doc = "0x04 - STGENC status register"]
    pub stgenc_cntsr: crate::Reg<stgenc_cntsr::STGENC_CNTSR_SPEC>,
    #[doc = "0x08 - the control interface must clear the STGENC_CNTCR.EN bit before writing to this register."]
    pub stgenc_cntcvl: crate::Reg<stgenc_cntcvl::STGENC_CNTCVL_SPEC>,
    #[doc = "0x0c - the control interface must clear the STGENC_CNTCR.EN bit before writing to this register."]
    pub stgenc_cntcvu: crate::Reg<stgenc_cntcvu::STGENC_CNTCVU_SPEC>,
    _reserved4: [u8; 0x10],
    #[doc = "0x20 - the control interface must clear the STGEN_CNTCR.EN bit before writing to this register."]
    pub stgenc_cntfid0: crate::Reg<stgenc_cntfid0::STGENC_CNTFID0_SPEC>,
    _reserved5: [u8; 0x0fac],
    #[doc = "0xfd0 - STGENC peripheral ID4 register"]
    pub stgenc_pidr4: crate::Reg<stgenc_pidr4::STGENC_PIDR4_SPEC>,
    #[doc = "0xfd4 - STGENC peripheral ID5 register"]
    pub stgenc_pidr5: crate::Reg<stgenc_pidr5::STGENC_PIDR5_SPEC>,
    #[doc = "0xfd8 - STGENC peripheral ID6 register"]
    pub stgenc_pidr6: crate::Reg<stgenc_pidr6::STGENC_PIDR6_SPEC>,
    #[doc = "0xfdc - STGENC peripheral ID7 register"]
    pub stgenc_pidr7: crate::Reg<stgenc_pidr7::STGENC_PIDR7_SPEC>,
    #[doc = "0xfe0 - STGENC peripheral ID0 register"]
    pub stgenc_pidr0: crate::Reg<stgenc_pidr0::STGENC_PIDR0_SPEC>,
    #[doc = "0xfe4 - STGENC peripheral ID1 register"]
    pub stgenc_pidr1: crate::Reg<stgenc_pidr1::STGENC_PIDR1_SPEC>,
    #[doc = "0xfe8 - STGENC peripheral ID2 register"]
    pub stgenc_pidr2: crate::Reg<stgenc_pidr2::STGENC_PIDR2_SPEC>,
    #[doc = "0xfec - STGENC peripheral ID3 register"]
    pub stgenc_pidr3: crate::Reg<stgenc_pidr3::STGENC_PIDR3_SPEC>,
    #[doc = "0xff0 - STGENC component ID0 register"]
    pub stgenc_cidr0: crate::Reg<stgenc_cidr0::STGENC_CIDR0_SPEC>,
    #[doc = "0xff4 - STGENC component ID1 register"]
    pub stgenc_cidr1: crate::Reg<stgenc_cidr1::STGENC_CIDR1_SPEC>,
    #[doc = "0xff8 - STGENC component ID2 register"]
    pub stgenc_cidr2: crate::Reg<stgenc_cidr2::STGENC_CIDR2_SPEC>,
    #[doc = "0xffc - STGENC component ID3 register"]
    pub stgenc_cidr3: crate::Reg<stgenc_cidr3::STGENC_CIDR3_SPEC>,
}
#[doc = "STGENC_CNTCR register accessor: an alias for `Reg<STGENC_CNTCR_SPEC>`"]
pub type STGENC_CNTCR = crate::Reg<stgenc_cntcr::STGENC_CNTCR_SPEC>;
#[doc = "STGENC control register"]
pub mod stgenc_cntcr;
#[doc = "STGENC_CNTSR register accessor: an alias for `Reg<STGENC_CNTSR_SPEC>`"]
pub type STGENC_CNTSR = crate::Reg<stgenc_cntsr::STGENC_CNTSR_SPEC>;
#[doc = "STGENC status register"]
pub mod stgenc_cntsr;
#[doc = "STGENC_CNTCVL register accessor: an alias for `Reg<STGENC_CNTCVL_SPEC>`"]
pub type STGENC_CNTCVL = crate::Reg<stgenc_cntcvl::STGENC_CNTCVL_SPEC>;
#[doc = "the control interface must clear the STGENC_CNTCR.EN bit before writing to this register."]
pub mod stgenc_cntcvl;
#[doc = "STGENC_CNTCVU register accessor: an alias for `Reg<STGENC_CNTCVU_SPEC>`"]
pub type STGENC_CNTCVU = crate::Reg<stgenc_cntcvu::STGENC_CNTCVU_SPEC>;
#[doc = "the control interface must clear the STGENC_CNTCR.EN bit before writing to this register."]
pub mod stgenc_cntcvu;
#[doc = "STGENC_CNTFID0 register accessor: an alias for `Reg<STGENC_CNTFID0_SPEC>`"]
pub type STGENC_CNTFID0 = crate::Reg<stgenc_cntfid0::STGENC_CNTFID0_SPEC>;
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register."]
pub mod stgenc_cntfid0;
#[doc = "STGENC_PIDR4 register accessor: an alias for `Reg<STGENC_PIDR4_SPEC>`"]
pub type STGENC_PIDR4 = crate::Reg<stgenc_pidr4::STGENC_PIDR4_SPEC>;
#[doc = "STGENC peripheral ID4 register"]
pub mod stgenc_pidr4;
#[doc = "STGENC_PIDR5 register accessor: an alias for `Reg<STGENC_PIDR5_SPEC>`"]
pub type STGENC_PIDR5 = crate::Reg<stgenc_pidr5::STGENC_PIDR5_SPEC>;
#[doc = "STGENC peripheral ID5 register"]
pub mod stgenc_pidr5;
#[doc = "STGENC_PIDR6 register accessor: an alias for `Reg<STGENC_PIDR6_SPEC>`"]
pub type STGENC_PIDR6 = crate::Reg<stgenc_pidr6::STGENC_PIDR6_SPEC>;
#[doc = "STGENC peripheral ID6 register"]
pub mod stgenc_pidr6;
#[doc = "STGENC_PIDR7 register accessor: an alias for `Reg<STGENC_PIDR7_SPEC>`"]
pub type STGENC_PIDR7 = crate::Reg<stgenc_pidr7::STGENC_PIDR7_SPEC>;
#[doc = "STGENC peripheral ID7 register"]
pub mod stgenc_pidr7;
#[doc = "STGENC_PIDR0 register accessor: an alias for `Reg<STGENC_PIDR0_SPEC>`"]
pub type STGENC_PIDR0 = crate::Reg<stgenc_pidr0::STGENC_PIDR0_SPEC>;
#[doc = "STGENC peripheral ID0 register"]
pub mod stgenc_pidr0;
#[doc = "STGENC_PIDR1 register accessor: an alias for `Reg<STGENC_PIDR1_SPEC>`"]
pub type STGENC_PIDR1 = crate::Reg<stgenc_pidr1::STGENC_PIDR1_SPEC>;
#[doc = "STGENC peripheral ID1 register"]
pub mod stgenc_pidr1;
#[doc = "STGENC_PIDR2 register accessor: an alias for `Reg<STGENC_PIDR2_SPEC>`"]
pub type STGENC_PIDR2 = crate::Reg<stgenc_pidr2::STGENC_PIDR2_SPEC>;
#[doc = "STGENC peripheral ID2 register"]
pub mod stgenc_pidr2;
#[doc = "STGENC_PIDR3 register accessor: an alias for `Reg<STGENC_PIDR3_SPEC>`"]
pub type STGENC_PIDR3 = crate::Reg<stgenc_pidr3::STGENC_PIDR3_SPEC>;
#[doc = "STGENC peripheral ID3 register"]
pub mod stgenc_pidr3;
#[doc = "STGENC_CIDR0 register accessor: an alias for `Reg<STGENC_CIDR0_SPEC>`"]
pub type STGENC_CIDR0 = crate::Reg<stgenc_cidr0::STGENC_CIDR0_SPEC>;
#[doc = "STGENC component ID0 register"]
pub mod stgenc_cidr0;
#[doc = "STGENC_CIDR1 register accessor: an alias for `Reg<STGENC_CIDR1_SPEC>`"]
pub type STGENC_CIDR1 = crate::Reg<stgenc_cidr1::STGENC_CIDR1_SPEC>;
#[doc = "STGENC component ID1 register"]
pub mod stgenc_cidr1;
#[doc = "STGENC_CIDR2 register accessor: an alias for `Reg<STGENC_CIDR2_SPEC>`"]
pub type STGENC_CIDR2 = crate::Reg<stgenc_cidr2::STGENC_CIDR2_SPEC>;
#[doc = "STGENC component ID2 register"]
pub mod stgenc_cidr2;
#[doc = "STGENC_CIDR3 register accessor: an alias for `Reg<STGENC_CIDR3_SPEC>`"]
pub type STGENC_CIDR3 = crate::Reg<stgenc_cidr3::STGENC_CIDR3_SPEC>;
#[doc = "STGENC component ID3 register"]
pub mod stgenc_cidr3;
