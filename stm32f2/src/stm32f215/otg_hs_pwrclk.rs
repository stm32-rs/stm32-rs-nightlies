#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power and clock gating control register"]
    pub otg_hs_pcgcr: OTG_HS_PCGCR,
}
#[doc = "Power and clock gating control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_pcgcr](otg_hs_pcgcr) module"]
pub type OTG_HS_PCGCR = crate::Reg<u32, _OTG_HS_PCGCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_PCGCR;
#[doc = "`read()` method returns [otg_hs_pcgcr::R](otg_hs_pcgcr::R) reader structure"]
impl crate::Readable for OTG_HS_PCGCR {}
#[doc = "`write(|w| ..)` method takes [otg_hs_pcgcr::W](otg_hs_pcgcr::W) writer structure"]
impl crate::Writable for OTG_HS_PCGCR {}
#[doc = "Power and clock gating control register"]
pub mod otg_hs_pcgcr;
