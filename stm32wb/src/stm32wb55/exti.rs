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
    #[doc = "0x80 - CPUm wakeup with interrupt mask register"]
    pub imr1: crate::Reg<imr1::IMR1_SPEC>,
    #[doc = "0x84 - CPUm wakeup with event mask register"]
    pub emr1: crate::Reg<emr1::EMR1_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x90 - CPUm wakeup with interrupt mask register"]
    pub imr2: crate::Reg<imr2::IMR2_SPEC>,
    #[doc = "0x94 - CPUm wakeup with event mask register"]
    pub emr2: crate::Reg<emr2::EMR2_SPEC>,
    _reserved12: [u8; 0x28],
    #[doc = "0xc0 - CPUm wakeup with interrupt mask register"]
    pub c2imr1: crate::Reg<c2imr1::C2IMR1_SPEC>,
    #[doc = "0xc4 - CPUm wakeup with event mask register"]
    pub c2emr1: crate::Reg<c2emr1::C2EMR1_SPEC>,
    _reserved14: [u8; 0x08],
    #[doc = "0xd0 - CPUm wakeup with interrupt mask register"]
    pub c2imr2: crate::Reg<c2imr2::C2IMR2_SPEC>,
    #[doc = "0xd4 - CPUm wakeup with event mask register"]
    pub c2emr2: crate::Reg<c2emr2::C2EMR2_SPEC>,
    _reserved16: [u8; 0x0300],
    #[doc = "0x3d8 - EXTI Hardware configuration registers"]
    pub hwcfgr7: crate::Reg<hwcfgr7::HWCFGR7_SPEC>,
    #[doc = "0x3dc - Hardware configuration registers"]
    pub hwcfgr6: crate::Reg<hwcfgr6::HWCFGR6_SPEC>,
    #[doc = "0x3e0 - Hardware configuration registers"]
    pub hwcfgr5: crate::Reg<hwcfgr5::HWCFGR5_SPEC>,
    #[doc = "0x3e4 - Hardware configuration registers"]
    pub hwcfgr4: crate::Reg<hwcfgr4::HWCFGR4_SPEC>,
    #[doc = "0x3e8 - Hardware configuration registers"]
    pub hwcfgr3: crate::Reg<hwcfgr3::HWCFGR3_SPEC>,
    #[doc = "0x3ec - Hardware configuration registers"]
    pub hwcfgr2: crate::Reg<hwcfgr2::HWCFGR2_SPEC>,
    #[doc = "0x3f0 - Hardware configuration register 1"]
    pub hwcfgr1: crate::Reg<hwcfgr1::HWCFGR1_SPEC>,
    #[doc = "0x3f4 - EXTI IP Version register"]
    pub verr: crate::Reg<verr::VERR_SPEC>,
    #[doc = "0x3f8 - Identification register"]
    pub ipidr: crate::Reg<ipidr::IPIDR_SPEC>,
    #[doc = "0x3fc - Size ID register"]
    pub sidr: crate::Reg<sidr::SIDR_SPEC>,
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
#[doc = "IMR1 register accessor: an alias for `Reg<IMR1_SPEC>`"]
pub type IMR1 = crate::Reg<imr1::IMR1_SPEC>;
#[doc = "CPUm wakeup with interrupt mask register"]
pub mod imr1;
#[doc = "C2IMR1 register accessor: an alias for `Reg<C2IMR1_SPEC>`"]
pub type C2IMR1 = crate::Reg<c2imr1::C2IMR1_SPEC>;
#[doc = "CPUm wakeup with interrupt mask register"]
pub mod c2imr1;
#[doc = "EMR1 register accessor: an alias for `Reg<EMR1_SPEC>`"]
pub type EMR1 = crate::Reg<emr1::EMR1_SPEC>;
#[doc = "CPUm wakeup with event mask register"]
pub mod emr1;
#[doc = "C2EMR1 register accessor: an alias for `Reg<C2EMR1_SPEC>`"]
pub type C2EMR1 = crate::Reg<c2emr1::C2EMR1_SPEC>;
#[doc = "CPUm wakeup with event mask register"]
pub mod c2emr1;
#[doc = "IMR2 register accessor: an alias for `Reg<IMR2_SPEC>`"]
pub type IMR2 = crate::Reg<imr2::IMR2_SPEC>;
#[doc = "CPUm wakeup with interrupt mask register"]
pub mod imr2;
#[doc = "C2IMR2 register accessor: an alias for `Reg<C2IMR2_SPEC>`"]
pub type C2IMR2 = crate::Reg<c2imr2::C2IMR2_SPEC>;
#[doc = "CPUm wakeup with interrupt mask register"]
pub mod c2imr2;
#[doc = "EMR2 register accessor: an alias for `Reg<EMR2_SPEC>`"]
pub type EMR2 = crate::Reg<emr2::EMR2_SPEC>;
#[doc = "CPUm wakeup with event mask register"]
pub mod emr2;
#[doc = "C2EMR2 register accessor: an alias for `Reg<C2EMR2_SPEC>`"]
pub type C2EMR2 = crate::Reg<c2emr2::C2EMR2_SPEC>;
#[doc = "CPUm wakeup with event mask register"]
pub mod c2emr2;
#[doc = "HWCFGR5 register accessor: an alias for `Reg<HWCFGR5_SPEC>`"]
pub type HWCFGR5 = crate::Reg<hwcfgr5::HWCFGR5_SPEC>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr5;
#[doc = "HWCFGR6 register accessor: an alias for `Reg<HWCFGR6_SPEC>`"]
pub type HWCFGR6 = crate::Reg<hwcfgr6::HWCFGR6_SPEC>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr6;
#[doc = "HWCFGR7 register accessor: an alias for `Reg<HWCFGR7_SPEC>`"]
pub type HWCFGR7 = crate::Reg<hwcfgr7::HWCFGR7_SPEC>;
#[doc = "EXTI Hardware configuration registers"]
pub mod hwcfgr7;
#[doc = "HWCFGR2 register accessor: an alias for `Reg<HWCFGR2_SPEC>`"]
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2_SPEC>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr2;
#[doc = "HWCFGR3 register accessor: an alias for `Reg<HWCFGR3_SPEC>`"]
pub type HWCFGR3 = crate::Reg<hwcfgr3::HWCFGR3_SPEC>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr3;
#[doc = "HWCFGR4 register accessor: an alias for `Reg<HWCFGR4_SPEC>`"]
pub type HWCFGR4 = crate::Reg<hwcfgr4::HWCFGR4_SPEC>;
#[doc = "Hardware configuration registers"]
pub mod hwcfgr4;
#[doc = "HWCFGR1 register accessor: an alias for `Reg<HWCFGR1_SPEC>`"]
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1_SPEC>;
#[doc = "Hardware configuration register 1"]
pub mod hwcfgr1;
#[doc = "VERR register accessor: an alias for `Reg<VERR_SPEC>`"]
pub type VERR = crate::Reg<verr::VERR_SPEC>;
#[doc = "EXTI IP Version register"]
pub mod verr;
#[doc = "IPIDR register accessor: an alias for `Reg<IPIDR_SPEC>`"]
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
#[doc = "Identification register"]
pub mod ipidr;
#[doc = "SIDR register accessor: an alias for `Reg<SIDR_SPEC>`"]
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
#[doc = "Size ID register"]
pub mod sidr;
