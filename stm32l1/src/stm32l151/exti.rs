#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IMR"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x04 - EMR"]
    pub emr: crate::Reg<emr::EMR_SPEC>,
    #[doc = "0x08 - RTSR"]
    pub rtsr: crate::Reg<rtsr::RTSR_SPEC>,
    #[doc = "0x0c - FTSR"]
    pub ftsr: crate::Reg<ftsr::FTSR_SPEC>,
    #[doc = "0x10 - SWIER"]
    pub swier: crate::Reg<swier::SWIER_SPEC>,
    #[doc = "0x14 - PR"]
    pub pr: crate::Reg<pr::PR_SPEC>,
}
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "IMR"]
pub mod imr;
#[doc = "EMR register accessor: an alias for `Reg<EMR_SPEC>`"]
pub type EMR = crate::Reg<emr::EMR_SPEC>;
#[doc = "EMR"]
pub mod emr;
#[doc = "RTSR register accessor: an alias for `Reg<RTSR_SPEC>`"]
pub type RTSR = crate::Reg<rtsr::RTSR_SPEC>;
#[doc = "RTSR"]
pub mod rtsr;
#[doc = "FTSR register accessor: an alias for `Reg<FTSR_SPEC>`"]
pub type FTSR = crate::Reg<ftsr::FTSR_SPEC>;
#[doc = "FTSR"]
pub mod ftsr;
#[doc = "SWIER register accessor: an alias for `Reg<SWIER_SPEC>`"]
pub type SWIER = crate::Reg<swier::SWIER_SPEC>;
#[doc = "SWIER"]
pub mod swier;
#[doc = "PR register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "PR"]
pub mod pr;
