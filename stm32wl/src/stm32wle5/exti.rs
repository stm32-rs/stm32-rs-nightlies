#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - rising trigger selection register"]
    pub rtsr1: crate::Reg<rtsr1::RTSR1_SPEC>,
    #[doc = "0x04 - falling trigger selection register"]
    pub ftsr1: crate::Reg<ftsr1::FTSR1_SPEC>,
    #[doc = "0x08 - software interrupt event register"]
    pub swier1: crate::Reg<swier1::SWIER1_SPEC>,
    #[doc = "0x0c - EXTI pending register"]
    pub pr1: crate::Reg<pr1::PR1_SPEC>,
    _reserved4: [u8; 0x10],
    #[doc = "0x20 - rising trigger selection register"]
    pub rtsr2: crate::Reg<rtsr2::RTSR2_SPEC>,
    #[doc = "0x24 - falling trigger selection register"]
    pub ftsr2: crate::Reg<ftsr2::FTSR2_SPEC>,
    #[doc = "0x28 - software interrupt event register"]
    pub swier2: crate::Reg<swier2::SWIER2_SPEC>,
    #[doc = "0x2c - pending register"]
    pub pr2: crate::Reg<pr2::PR2_SPEC>,
    _reserved8: [u8; 0x50],
    #[doc = "0x80 - interrupt mask register"]
    pub c1imr1: crate::Reg<c1imr1::C1IMR1_SPEC>,
    #[doc = "0x84 - event mask register"]
    pub emr1: crate::Reg<emr1::EMR1_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x90 - interrupt mask register"]
    pub c1imr2: crate::Reg<c1imr2::C1IMR2_SPEC>,
}
#[doc = "RTSR1 register accessor: an alias for `Reg<RTSR1_SPEC>`"]
pub type RTSR1 = crate::Reg<rtsr1::RTSR1_SPEC>;
#[doc = "rising trigger selection register"]
pub mod rtsr1;
#[doc = "FTSR1 register accessor: an alias for `Reg<FTSR1_SPEC>`"]
pub type FTSR1 = crate::Reg<ftsr1::FTSR1_SPEC>;
#[doc = "falling trigger selection register"]
pub mod ftsr1;
#[doc = "SWIER1 register accessor: an alias for `Reg<SWIER1_SPEC>`"]
pub type SWIER1 = crate::Reg<swier1::SWIER1_SPEC>;
#[doc = "software interrupt event register"]
pub mod swier1;
#[doc = "PR1 register accessor: an alias for `Reg<PR1_SPEC>`"]
pub type PR1 = crate::Reg<pr1::PR1_SPEC>;
#[doc = "EXTI pending register"]
pub mod pr1;
#[doc = "RTSR2 register accessor: an alias for `Reg<RTSR2_SPEC>`"]
pub type RTSR2 = crate::Reg<rtsr2::RTSR2_SPEC>;
#[doc = "rising trigger selection register"]
pub mod rtsr2;
#[doc = "FTSR2 register accessor: an alias for `Reg<FTSR2_SPEC>`"]
pub type FTSR2 = crate::Reg<ftsr2::FTSR2_SPEC>;
#[doc = "falling trigger selection register"]
pub mod ftsr2;
#[doc = "SWIER2 register accessor: an alias for `Reg<SWIER2_SPEC>`"]
pub type SWIER2 = crate::Reg<swier2::SWIER2_SPEC>;
#[doc = "software interrupt event register"]
pub mod swier2;
#[doc = "PR2 register accessor: an alias for `Reg<PR2_SPEC>`"]
pub type PR2 = crate::Reg<pr2::PR2_SPEC>;
#[doc = "pending register"]
pub mod pr2;
#[doc = "C1IMR1 register accessor: an alias for `Reg<C1IMR1_SPEC>`"]
pub type C1IMR1 = crate::Reg<c1imr1::C1IMR1_SPEC>;
#[doc = "interrupt mask register"]
pub mod c1imr1;
#[doc = "EMR1 register accessor: an alias for `Reg<EMR1_SPEC>`"]
pub type EMR1 = crate::Reg<emr1::EMR1_SPEC>;
#[doc = "event mask register"]
pub mod emr1;
#[doc = "C1IMR2 register accessor: an alias for `Reg<C1IMR2_SPEC>`"]
pub type C1IMR2 = crate::Reg<c1imr2::C1IMR2_SPEC>;
#[doc = "interrupt mask register"]
pub mod c1imr2;
