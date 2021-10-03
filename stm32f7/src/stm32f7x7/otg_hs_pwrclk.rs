#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power and clock gating control register"]
    pub otg_hs_pcgcr: crate::Reg<otg_hs_pcgcr::OTG_HS_PCGCR_SPEC>,
}
#[doc = "OTG_HS_PCGCR register accessor: an alias for `Reg<OTG_HS_PCGCR_SPEC>`"]
pub type OTG_HS_PCGCR = crate::Reg<otg_hs_pcgcr::OTG_HS_PCGCR_SPEC>;
#[doc = "Power and clock gating control register"]
pub mod otg_hs_pcgcr;
