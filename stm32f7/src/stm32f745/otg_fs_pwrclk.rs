#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)"]
    pub otg_fs_pcgcctl: OTG_FS_PCGCCTL,
}
#[doc = "OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_pcgcctl](otg_fs_pcgcctl) module"]
pub type OTG_FS_PCGCCTL = crate::Reg<u32, _OTG_FS_PCGCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_PCGCCTL;
#[doc = "`read()` method returns [otg_fs_pcgcctl::R](otg_fs_pcgcctl::R) reader structure"]
impl crate::Readable for OTG_FS_PCGCCTL {}
#[doc = "`write(|w| ..)` method takes [otg_fs_pcgcctl::W](otg_fs_pcgcctl::W) writer structure"]
impl crate::Writable for OTG_FS_PCGCCTL {}
#[doc = "OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)"]
pub mod otg_fs_pcgcctl;
