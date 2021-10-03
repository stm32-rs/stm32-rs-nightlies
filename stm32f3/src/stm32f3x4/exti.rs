#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt mask register"]
    pub imr1: crate::Reg<imr1::IMR1_SPEC>,
    #[doc = "0x04 - Event mask register"]
    pub emr1: crate::Reg<emr1::EMR1_SPEC>,
    #[doc = "0x08 - Rising Trigger selection register"]
    pub rtsr1: crate::Reg<rtsr1::RTSR1_SPEC>,
    #[doc = "0x0c - Falling Trigger selection register"]
    pub ftsr1: crate::Reg<ftsr1::FTSR1_SPEC>,
    #[doc = "0x10 - Software interrupt event register"]
    pub swier1: crate::Reg<swier1::SWIER1_SPEC>,
    #[doc = "0x14 - Pending register"]
    pub pr1: crate::Reg<pr1::PR1_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - Interrupt mask register"]
    pub imr2: crate::Reg<imr2::IMR2_SPEC>,
    #[doc = "0x24 - Event mask register"]
    pub emr2: crate::Reg<emr2::EMR2_SPEC>,
    #[doc = "0x28 - Rising Trigger selection register"]
    pub rtsr2: crate::Reg<rtsr2::RTSR2_SPEC>,
    #[doc = "0x2c - Falling Trigger selection register"]
    pub ftsr2: crate::Reg<ftsr2::FTSR2_SPEC>,
    #[doc = "0x30 - Software interrupt event register"]
    pub swier2: crate::Reg<swier2::SWIER2_SPEC>,
    #[doc = "0x34 - Pending register"]
    pub pr2: crate::Reg<pr2::PR2_SPEC>,
}
#[doc = "IMR1 register accessor: an alias for `Reg<IMR1_SPEC>`"]
pub type IMR1 = crate::Reg<imr1::IMR1_SPEC>;
#[doc = "Interrupt mask register"]
pub mod imr1;
#[doc = "EMR1 register accessor: an alias for `Reg<EMR1_SPEC>`"]
pub type EMR1 = crate::Reg<emr1::EMR1_SPEC>;
#[doc = "Event mask register"]
pub mod emr1;
#[doc = "RTSR1 register accessor: an alias for `Reg<RTSR1_SPEC>`"]
pub type RTSR1 = crate::Reg<rtsr1::RTSR1_SPEC>;
#[doc = "Rising Trigger selection register"]
pub mod rtsr1;
#[doc = "FTSR1 register accessor: an alias for `Reg<FTSR1_SPEC>`"]
pub type FTSR1 = crate::Reg<ftsr1::FTSR1_SPEC>;
#[doc = "Falling Trigger selection register"]
pub mod ftsr1;
#[doc = "SWIER1 register accessor: an alias for `Reg<SWIER1_SPEC>`"]
pub type SWIER1 = crate::Reg<swier1::SWIER1_SPEC>;
#[doc = "Software interrupt event register"]
pub mod swier1;
#[doc = "PR1 register accessor: an alias for `Reg<PR1_SPEC>`"]
pub type PR1 = crate::Reg<pr1::PR1_SPEC>;
#[doc = "Pending register"]
pub mod pr1;
#[doc = "IMR2 register accessor: an alias for `Reg<IMR2_SPEC>`"]
pub type IMR2 = crate::Reg<imr2::IMR2_SPEC>;
#[doc = "Interrupt mask register"]
pub mod imr2;
#[doc = "EMR2 register accessor: an alias for `Reg<EMR2_SPEC>`"]
pub type EMR2 = crate::Reg<emr2::EMR2_SPEC>;
#[doc = "Event mask register"]
pub mod emr2;
#[doc = "RTSR2 register accessor: an alias for `Reg<RTSR2_SPEC>`"]
pub type RTSR2 = crate::Reg<rtsr2::RTSR2_SPEC>;
#[doc = "Rising Trigger selection register"]
pub mod rtsr2;
#[doc = "FTSR2 register accessor: an alias for `Reg<FTSR2_SPEC>`"]
pub type FTSR2 = crate::Reg<ftsr2::FTSR2_SPEC>;
#[doc = "Falling Trigger selection register"]
pub mod ftsr2;
#[doc = "SWIER2 register accessor: an alias for `Reg<SWIER2_SPEC>`"]
pub type SWIER2 = crate::Reg<swier2::SWIER2_SPEC>;
#[doc = "Software interrupt event register"]
pub mod swier2;
#[doc = "PR2 register accessor: an alias for `Reg<PR2_SPEC>`"]
pub type PR2 = crate::Reg<pr2::PR2_SPEC>;
#[doc = "Pending register"]
pub mod pr2;
