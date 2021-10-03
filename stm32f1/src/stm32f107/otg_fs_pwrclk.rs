#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)"]
    pub fs_pcgcctl: crate::Reg<fs_pcgcctl::FS_PCGCCTL_SPEC>,
}
#[doc = "FS_PCGCCTL register accessor: an alias for `Reg<FS_PCGCCTL_SPEC>`"]
pub type FS_PCGCCTL = crate::Reg<fs_pcgcctl::FS_PCGCCTL_SPEC>;
#[doc = "OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)"]
pub mod fs_pcgcctl;
