#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - memory remap register"]
    pub memrmp: crate::Reg<memrmp::MEMRMP_SPEC>,
    #[doc = "0x04 - configuration register 1"]
    pub cfgr1: crate::Reg<cfgr1::CFGR1_SPEC>,
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub exticr1: crate::Reg<exticr1::EXTICR1_SPEC>,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub exticr2: crate::Reg<exticr2::EXTICR2_SPEC>,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub exticr3: crate::Reg<exticr3::EXTICR3_SPEC>,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub exticr4: crate::Reg<exticr4::EXTICR4_SPEC>,
    #[doc = "0x18 - SCSR"]
    pub scsr: crate::Reg<scsr::SCSR_SPEC>,
    #[doc = "0x1c - CFGR2"]
    pub cfgr2: crate::Reg<cfgr2::CFGR2_SPEC>,
    #[doc = "0x20 - SRAM2 write protection register"]
    pub swpr: crate::Reg<swpr::SWPR_SPEC>,
    #[doc = "0x24 - SKR"]
    pub skr: crate::Reg<skr::SKR_SPEC>,
    #[doc = "0x28 - SRAM2 write protection register 2"]
    pub swpr2: crate::Reg<swpr2::SWPR2_SPEC>,
    _reserved11: [u8; 0xd4],
    #[doc = "0x100 - CPU1 interrupt mask register 1"]
    pub imr1: crate::Reg<imr1::IMR1_SPEC>,
    #[doc = "0x104 - CPU1 interrupt mask register 2"]
    pub imr2: crate::Reg<imr2::IMR2_SPEC>,
    #[doc = "0x108 - CPU2 interrupt mask register 1"]
    pub c2imr1: crate::Reg<c2imr1::C2IMR1_SPEC>,
    #[doc = "0x10c - CPU2 interrupt mask register 1"]
    pub c2imr2: crate::Reg<c2imr2::C2IMR2_SPEC>,
    #[doc = "0x110 - secure IP control register"]
    pub sipcr: crate::Reg<sipcr::SIPCR_SPEC>,
}
#[doc = "MEMRMP register accessor: an alias for `Reg<MEMRMP_SPEC>`"]
pub type MEMRMP = crate::Reg<memrmp::MEMRMP_SPEC>;
#[doc = "memory remap register"]
pub mod memrmp;
#[doc = "CFGR1 register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "configuration register 1"]
pub mod cfgr1;
#[doc = "EXTICR1 register accessor: an alias for `Reg<EXTICR1_SPEC>`"]
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "EXTICR2 register accessor: an alias for `Reg<EXTICR2_SPEC>`"]
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "EXTICR3 register accessor: an alias for `Reg<EXTICR3_SPEC>`"]
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "EXTICR4 register accessor: an alias for `Reg<EXTICR4_SPEC>`"]
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "SCSR register accessor: an alias for `Reg<SCSR_SPEC>`"]
pub type SCSR = crate::Reg<scsr::SCSR_SPEC>;
#[doc = "SCSR"]
pub mod scsr;
#[doc = "CFGR2 register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "CFGR2"]
pub mod cfgr2;
#[doc = "SWPR register accessor: an alias for `Reg<SWPR_SPEC>`"]
pub type SWPR = crate::Reg<swpr::SWPR_SPEC>;
#[doc = "SRAM2 write protection register"]
pub mod swpr;
#[doc = "SKR register accessor: an alias for `Reg<SKR_SPEC>`"]
pub type SKR = crate::Reg<skr::SKR_SPEC>;
#[doc = "SKR"]
pub mod skr;
#[doc = "SWPR2 register accessor: an alias for `Reg<SWPR2_SPEC>`"]
pub type SWPR2 = crate::Reg<swpr2::SWPR2_SPEC>;
#[doc = "SRAM2 write protection register 2"]
pub mod swpr2;
#[doc = "IMR1 register accessor: an alias for `Reg<IMR1_SPEC>`"]
pub type IMR1 = crate::Reg<imr1::IMR1_SPEC>;
#[doc = "CPU1 interrupt mask register 1"]
pub mod imr1;
#[doc = "IMR2 register accessor: an alias for `Reg<IMR2_SPEC>`"]
pub type IMR2 = crate::Reg<imr2::IMR2_SPEC>;
#[doc = "CPU1 interrupt mask register 2"]
pub mod imr2;
#[doc = "C2IMR1 register accessor: an alias for `Reg<C2IMR1_SPEC>`"]
pub type C2IMR1 = crate::Reg<c2imr1::C2IMR1_SPEC>;
#[doc = "CPU2 interrupt mask register 1"]
pub mod c2imr1;
#[doc = "C2IMR2 register accessor: an alias for `Reg<C2IMR2_SPEC>`"]
pub type C2IMR2 = crate::Reg<c2imr2::C2IMR2_SPEC>;
#[doc = "CPU2 interrupt mask register 1"]
pub mod c2imr2;
#[doc = "SIPCR register accessor: an alias for `Reg<SIPCR_SPEC>`"]
pub type SIPCR = crate::Reg<sipcr::SIPCR_SPEC>;
#[doc = "secure IP control register"]
pub mod sipcr;
