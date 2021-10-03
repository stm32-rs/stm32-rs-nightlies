#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EXTI rising trigger selection register"]
    pub rtsr1: crate::Reg<rtsr1::RTSR1_SPEC>,
    #[doc = "0x04 - EXTI falling trigger selection register"]
    pub ftsr1: crate::Reg<ftsr1::FTSR1_SPEC>,
    #[doc = "0x08 - EXTI software interrupt event register"]
    pub swier1: crate::Reg<swier1::SWIER1_SPEC>,
    #[doc = "0x0c - EXTI rising edge pending register"]
    pub rpr1: crate::Reg<rpr1::RPR1_SPEC>,
    #[doc = "0x10 - EXTI falling edge pending register"]
    pub fpr1: crate::Reg<fpr1::FPR1_SPEC>,
    #[doc = "0x14 - EXTI security configuration register"]
    pub seccfgr1: crate::Reg<seccfgr1::SECCFGR1_SPEC>,
    #[doc = "0x18 - EXTI privilege configuration register"]
    pub privcfgr1: crate::Reg<privcfgr1::PRIVCFGR1_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - EXTI rising trigger selection register"]
    pub rtsr2: crate::Reg<rtsr2::RTSR2_SPEC>,
    #[doc = "0x24 - EXTI falling trigger selection register"]
    pub ftsr2: crate::Reg<ftsr2::FTSR2_SPEC>,
    #[doc = "0x28 - EXTI software interrupt event register"]
    pub swier2: crate::Reg<swier2::SWIER2_SPEC>,
    #[doc = "0x2c - EXTI rising edge pending register"]
    pub rpr2: crate::Reg<rpr2::RPR2_SPEC>,
    #[doc = "0x30 - EXTI falling edge pending register"]
    pub fpr2: crate::Reg<fpr2::FPR2_SPEC>,
    #[doc = "0x34 - EXTI security enable register"]
    pub privcfgr2: crate::Reg<privcfgr2::PRIVCFGR2_SPEC>,
    #[doc = "0x38 - EXTI security enable register"]
    pub seccfgr2: crate::Reg<seccfgr2::SECCFGR2_SPEC>,
    _reserved14: [u8; 0x24],
    #[doc = "0x60 - EXTI external interrupt selection register"]
    pub exticr1: crate::Reg<exticr1::EXTICR1_SPEC>,
    #[doc = "0x64 - EXTI external interrupt selection register"]
    pub exticr2: crate::Reg<exticr2::EXTICR2_SPEC>,
    #[doc = "0x68 - EXTI external interrupt selection register"]
    pub exticr3: crate::Reg<exticr3::EXTICR3_SPEC>,
    #[doc = "0x6c - EXTI external interrupt selection register"]
    pub exticr4: crate::Reg<exticr4::EXTICR4_SPEC>,
    #[doc = "0x70 - EXTI lock register"]
    pub lockrg: crate::Reg<lockrg::LOCKRG_SPEC>,
    _reserved19: [u8; 0x0c],
    #[doc = "0x80 - EXTI CPU wakeup with interrupt mask register"]
    pub imr1: crate::Reg<imr1::IMR1_SPEC>,
    #[doc = "0x84 - EXTI CPU wakeup with event mask register"]
    pub emr1: crate::Reg<emr1::EMR1_SPEC>,
    _reserved21: [u8; 0x08],
    #[doc = "0x90 - EXTI CPUm wakeup with interrupt mask register"]
    pub imr2: crate::Reg<imr2::IMR2_SPEC>,
    #[doc = "0x94 - EXTI CPU wakeup with event mask register"]
    pub emr2: crate::Reg<emr2::EMR2_SPEC>,
}
#[doc = "RTSR1 register accessor: an alias for `Reg<RTSR1_SPEC>`"]
pub type RTSR1 = crate::Reg<rtsr1::RTSR1_SPEC>;
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr1;
#[doc = "FTSR1 register accessor: an alias for `Reg<FTSR1_SPEC>`"]
pub type FTSR1 = crate::Reg<ftsr1::FTSR1_SPEC>;
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr1;
#[doc = "SWIER1 register accessor: an alias for `Reg<SWIER1_SPEC>`"]
pub type SWIER1 = crate::Reg<swier1::SWIER1_SPEC>;
#[doc = "EXTI software interrupt event register"]
pub mod swier1;
#[doc = "RPR1 register accessor: an alias for `Reg<RPR1_SPEC>`"]
pub type RPR1 = crate::Reg<rpr1::RPR1_SPEC>;
#[doc = "EXTI rising edge pending register"]
pub mod rpr1;
#[doc = "FPR1 register accessor: an alias for `Reg<FPR1_SPEC>`"]
pub type FPR1 = crate::Reg<fpr1::FPR1_SPEC>;
#[doc = "EXTI falling edge pending register"]
pub mod fpr1;
#[doc = "SECCFGR1 register accessor: an alias for `Reg<SECCFGR1_SPEC>`"]
pub type SECCFGR1 = crate::Reg<seccfgr1::SECCFGR1_SPEC>;
#[doc = "EXTI security configuration register"]
pub mod seccfgr1;
#[doc = "PRIVCFGR1 register accessor: an alias for `Reg<PRIVCFGR1_SPEC>`"]
pub type PRIVCFGR1 = crate::Reg<privcfgr1::PRIVCFGR1_SPEC>;
#[doc = "EXTI privilege configuration register"]
pub mod privcfgr1;
#[doc = "RTSR2 register accessor: an alias for `Reg<RTSR2_SPEC>`"]
pub type RTSR2 = crate::Reg<rtsr2::RTSR2_SPEC>;
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr2;
#[doc = "FTSR2 register accessor: an alias for `Reg<FTSR2_SPEC>`"]
pub type FTSR2 = crate::Reg<ftsr2::FTSR2_SPEC>;
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr2;
#[doc = "SWIER2 register accessor: an alias for `Reg<SWIER2_SPEC>`"]
pub type SWIER2 = crate::Reg<swier2::SWIER2_SPEC>;
#[doc = "EXTI software interrupt event register"]
pub mod swier2;
#[doc = "RPR2 register accessor: an alias for `Reg<RPR2_SPEC>`"]
pub type RPR2 = crate::Reg<rpr2::RPR2_SPEC>;
#[doc = "EXTI rising edge pending register"]
pub mod rpr2;
#[doc = "FPR2 register accessor: an alias for `Reg<FPR2_SPEC>`"]
pub type FPR2 = crate::Reg<fpr2::FPR2_SPEC>;
#[doc = "EXTI falling edge pending register"]
pub mod fpr2;
#[doc = "SECCFGR2 register accessor: an alias for `Reg<SECCFGR2_SPEC>`"]
pub type SECCFGR2 = crate::Reg<seccfgr2::SECCFGR2_SPEC>;
#[doc = "EXTI security enable register"]
pub mod seccfgr2;
#[doc = "PRIVCFGR2 register accessor: an alias for `Reg<PRIVCFGR2_SPEC>`"]
pub type PRIVCFGR2 = crate::Reg<privcfgr2::PRIVCFGR2_SPEC>;
#[doc = "EXTI security enable register"]
pub mod privcfgr2;
#[doc = "EXTICR1 register accessor: an alias for `Reg<EXTICR1_SPEC>`"]
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr1;
#[doc = "EXTICR2 register accessor: an alias for `Reg<EXTICR2_SPEC>`"]
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr2;
#[doc = "EXTICR3 register accessor: an alias for `Reg<EXTICR3_SPEC>`"]
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr3;
#[doc = "EXTICR4 register accessor: an alias for `Reg<EXTICR4_SPEC>`"]
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr4;
#[doc = "LOCKRG register accessor: an alias for `Reg<LOCKRG_SPEC>`"]
pub type LOCKRG = crate::Reg<lockrg::LOCKRG_SPEC>;
#[doc = "EXTI lock register"]
pub mod lockrg;
#[doc = "IMR1 register accessor: an alias for `Reg<IMR1_SPEC>`"]
pub type IMR1 = crate::Reg<imr1::IMR1_SPEC>;
#[doc = "EXTI CPU wakeup with interrupt mask register"]
pub mod imr1;
#[doc = "EMR1 register accessor: an alias for `Reg<EMR1_SPEC>`"]
pub type EMR1 = crate::Reg<emr1::EMR1_SPEC>;
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod emr1;
#[doc = "IMR2 register accessor: an alias for `Reg<IMR2_SPEC>`"]
pub type IMR2 = crate::Reg<imr2::IMR2_SPEC>;
#[doc = "EXTI CPUm wakeup with interrupt mask register"]
pub mod imr2;
#[doc = "EMR2 register accessor: an alias for `Reg<EMR2_SPEC>`"]
pub type EMR2 = crate::Reg<emr2::EMR2_SPEC>;
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod emr2;
