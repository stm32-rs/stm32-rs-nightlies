#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)"]
    pub otg_fs_pcgcctl: crate::Reg<otg_fs_pcgcctl::OTG_FS_PCGCCTL_SPEC>,
}
#[doc = "OTG_FS_PCGCCTL register accessor: an alias for `Reg<OTG_FS_PCGCCTL_SPEC>`"]
pub type OTG_FS_PCGCCTL = crate::Reg<otg_fs_pcgcctl::OTG_FS_PCGCCTL_SPEC>;
#[doc = "OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)"]
pub mod otg_fs_pcgcctl;
