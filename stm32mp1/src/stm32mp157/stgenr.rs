#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - the control interface must clear the STGEN_CNTCR.EN bit before writing to this register."]
    pub stgenr_cntcvl: crate::Reg<stgenr_cntcvl::STGENR_CNTCVL_SPEC>,
    #[doc = "0x04 - the control interface must clear the STGEN_CNTCR.EN bit before writing to this register."]
    pub stgenr_cntcvu: crate::Reg<stgenr_cntcvu::STGENR_CNTCVU_SPEC>,
    _reserved2: [u8; 0x0fc8],
    #[doc = "0xfd0 - STGENR peripheral ID4 register"]
    pub stgenr_pidr4: crate::Reg<stgenr_pidr4::STGENR_PIDR4_SPEC>,
    #[doc = "0xfd4 - STGENR peripheral ID5 register"]
    pub stgenr_pidr5: crate::Reg<stgenr_pidr5::STGENR_PIDR5_SPEC>,
    #[doc = "0xfd8 - STGENR peripheral ID6 register"]
    pub stgenr_pidr6: crate::Reg<stgenr_pidr6::STGENR_PIDR6_SPEC>,
    #[doc = "0xfdc - STGENR peripheral ID7 register"]
    pub stgenr_pidr7: crate::Reg<stgenr_pidr7::STGENR_PIDR7_SPEC>,
    #[doc = "0xfe0 - STGENR peripheral ID0 register"]
    pub stgenr_pidr0: crate::Reg<stgenr_pidr0::STGENR_PIDR0_SPEC>,
    #[doc = "0xfe4 - STGENR peripheral ID1 register"]
    pub stgenr_pidr1: crate::Reg<stgenr_pidr1::STGENR_PIDR1_SPEC>,
    #[doc = "0xfe8 - STGENR peripheral ID2 register"]
    pub stgenr_pidr2: crate::Reg<stgenr_pidr2::STGENR_PIDR2_SPEC>,
    #[doc = "0xfec - STGENR peripheral ID3 register"]
    pub stgenr_pidr3: crate::Reg<stgenr_pidr3::STGENR_PIDR3_SPEC>,
    #[doc = "0xff0 - STGENR component ID0 register"]
    pub stgenr_cidr0: crate::Reg<stgenr_cidr0::STGENR_CIDR0_SPEC>,
    #[doc = "0xff4 - STGENR component ID1 register"]
    pub stgenr_cidr1: crate::Reg<stgenr_cidr1::STGENR_CIDR1_SPEC>,
    #[doc = "0xff8 - STGENR component ID2 register"]
    pub stgenr_cidr2: crate::Reg<stgenr_cidr2::STGENR_CIDR2_SPEC>,
    #[doc = "0xffc - STGENR component ID3 register"]
    pub stgenr_cidr3: crate::Reg<stgenr_cidr3::STGENR_CIDR3_SPEC>,
}
#[doc = "STGENR_CNTCVL register accessor: an alias for `Reg<STGENR_CNTCVL_SPEC>`"]
pub type STGENR_CNTCVL = crate::Reg<stgenr_cntcvl::STGENR_CNTCVL_SPEC>;
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register."]
pub mod stgenr_cntcvl;
#[doc = "STGENR_CNTCVU register accessor: an alias for `Reg<STGENR_CNTCVU_SPEC>`"]
pub type STGENR_CNTCVU = crate::Reg<stgenr_cntcvu::STGENR_CNTCVU_SPEC>;
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register."]
pub mod stgenr_cntcvu;
#[doc = "STGENR_PIDR4 register accessor: an alias for `Reg<STGENR_PIDR4_SPEC>`"]
pub type STGENR_PIDR4 = crate::Reg<stgenr_pidr4::STGENR_PIDR4_SPEC>;
#[doc = "STGENR peripheral ID4 register"]
pub mod stgenr_pidr4;
#[doc = "STGENR_PIDR5 register accessor: an alias for `Reg<STGENR_PIDR5_SPEC>`"]
pub type STGENR_PIDR5 = crate::Reg<stgenr_pidr5::STGENR_PIDR5_SPEC>;
#[doc = "STGENR peripheral ID5 register"]
pub mod stgenr_pidr5;
#[doc = "STGENR_PIDR6 register accessor: an alias for `Reg<STGENR_PIDR6_SPEC>`"]
pub type STGENR_PIDR6 = crate::Reg<stgenr_pidr6::STGENR_PIDR6_SPEC>;
#[doc = "STGENR peripheral ID6 register"]
pub mod stgenr_pidr6;
#[doc = "STGENR_PIDR7 register accessor: an alias for `Reg<STGENR_PIDR7_SPEC>`"]
pub type STGENR_PIDR7 = crate::Reg<stgenr_pidr7::STGENR_PIDR7_SPEC>;
#[doc = "STGENR peripheral ID7 register"]
pub mod stgenr_pidr7;
#[doc = "STGENR_PIDR0 register accessor: an alias for `Reg<STGENR_PIDR0_SPEC>`"]
pub type STGENR_PIDR0 = crate::Reg<stgenr_pidr0::STGENR_PIDR0_SPEC>;
#[doc = "STGENR peripheral ID0 register"]
pub mod stgenr_pidr0;
#[doc = "STGENR_PIDR1 register accessor: an alias for `Reg<STGENR_PIDR1_SPEC>`"]
pub type STGENR_PIDR1 = crate::Reg<stgenr_pidr1::STGENR_PIDR1_SPEC>;
#[doc = "STGENR peripheral ID1 register"]
pub mod stgenr_pidr1;
#[doc = "STGENR_PIDR2 register accessor: an alias for `Reg<STGENR_PIDR2_SPEC>`"]
pub type STGENR_PIDR2 = crate::Reg<stgenr_pidr2::STGENR_PIDR2_SPEC>;
#[doc = "STGENR peripheral ID2 register"]
pub mod stgenr_pidr2;
#[doc = "STGENR_PIDR3 register accessor: an alias for `Reg<STGENR_PIDR3_SPEC>`"]
pub type STGENR_PIDR3 = crate::Reg<stgenr_pidr3::STGENR_PIDR3_SPEC>;
#[doc = "STGENR peripheral ID3 register"]
pub mod stgenr_pidr3;
#[doc = "STGENR_CIDR0 register accessor: an alias for `Reg<STGENR_CIDR0_SPEC>`"]
pub type STGENR_CIDR0 = crate::Reg<stgenr_cidr0::STGENR_CIDR0_SPEC>;
#[doc = "STGENR component ID0 register"]
pub mod stgenr_cidr0;
#[doc = "STGENR_CIDR1 register accessor: an alias for `Reg<STGENR_CIDR1_SPEC>`"]
pub type STGENR_CIDR1 = crate::Reg<stgenr_cidr1::STGENR_CIDR1_SPEC>;
#[doc = "STGENR component ID1 register"]
pub mod stgenr_cidr1;
#[doc = "STGENR_CIDR2 register accessor: an alias for `Reg<STGENR_CIDR2_SPEC>`"]
pub type STGENR_CIDR2 = crate::Reg<stgenr_cidr2::STGENR_CIDR2_SPEC>;
#[doc = "STGENR component ID2 register"]
pub mod stgenr_cidr2;
#[doc = "STGENR_CIDR3 register accessor: an alias for `Reg<STGENR_CIDR3_SPEC>`"]
pub type STGENR_CIDR3 = crate::Reg<stgenr_cidr3::STGENR_CIDR3_SPEC>;
#[doc = "STGENR component ID3 register"]
pub mod stgenr_cidr3;
