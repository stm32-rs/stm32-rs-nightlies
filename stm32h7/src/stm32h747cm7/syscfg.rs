#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - peripheral mode configuration register"]
    pub pmcr: crate::Reg<pmcr::PMCR_SPEC>,
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub exticr1: crate::Reg<exticr1::EXTICR1_SPEC>,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub exticr2: crate::Reg<exticr2::EXTICR2_SPEC>,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub exticr3: crate::Reg<exticr3::EXTICR3_SPEC>,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub exticr4: crate::Reg<exticr4::EXTICR4_SPEC>,
    #[doc = "0x18 - configuration register"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - compensation cell control/status register"]
    pub cccsr: crate::Reg<cccsr::CCCSR_SPEC>,
    #[doc = "0x24 - SYSCFG compensation cell value register"]
    pub ccvr: crate::Reg<ccvr::CCVR_SPEC>,
    #[doc = "0x28 - SYSCFG compensation cell code register"]
    pub cccr: crate::Reg<cccr::CCCR_SPEC>,
    #[doc = "0x2c - SYSCFG power control register"]
    pub pwrcr: crate::Reg<pwrcr::PWRCR_SPEC>,
    _reserved10: [u8; 0xf4],
    #[doc = "0x124 - SYSCFG package register"]
    pub pkgr: crate::Reg<pkgr::PKGR_SPEC>,
    _reserved11: [u8; 0x01d8],
    #[doc = "0x300 - SYSCFG user register 0"]
    pub ur0: crate::Reg<ur0::UR0_SPEC>,
    #[doc = "0x304 - SYSCFG user register 1"]
    pub ur1: crate::Reg<ur1::UR1_SPEC>,
    #[doc = "0x308 - SYSCFG user register 2"]
    pub ur2: crate::Reg<ur2::UR2_SPEC>,
    #[doc = "0x30c - SYSCFG user register 3"]
    pub ur3: crate::Reg<ur3::UR3_SPEC>,
    #[doc = "0x310 - SYSCFG user register 4"]
    pub ur4: crate::Reg<ur4::UR4_SPEC>,
    #[doc = "0x314 - SYSCFG user register 5"]
    pub ur5: crate::Reg<ur5::UR5_SPEC>,
    #[doc = "0x318 - SYSCFG user register 6"]
    pub ur6: crate::Reg<ur6::UR6_SPEC>,
    #[doc = "0x31c - SYSCFG user register 7"]
    pub ur7: crate::Reg<ur7::UR7_SPEC>,
    #[doc = "0x320 - SYSCFG user register 8"]
    pub ur8: crate::Reg<ur8::UR8_SPEC>,
    #[doc = "0x324 - SYSCFG user register 9"]
    pub ur9: crate::Reg<ur9::UR9_SPEC>,
    #[doc = "0x328 - SYSCFG user register 10"]
    pub ur10: crate::Reg<ur10::UR10_SPEC>,
    #[doc = "0x32c - SYSCFG user register 11"]
    pub ur11: crate::Reg<ur11::UR11_SPEC>,
    #[doc = "0x330 - SYSCFG user register 12"]
    pub ur12: crate::Reg<ur12::UR12_SPEC>,
    #[doc = "0x334 - SYSCFG user register 13"]
    pub ur13: crate::Reg<ur13::UR13_SPEC>,
    #[doc = "0x338 - SYSCFG user register 14"]
    pub ur14: crate::Reg<ur14::UR14_SPEC>,
    #[doc = "0x33c - SYSCFG user register 15"]
    pub ur15: crate::Reg<ur15::UR15_SPEC>,
    #[doc = "0x340 - SYSCFG user register 16"]
    pub ur16: crate::Reg<ur16::UR16_SPEC>,
    #[doc = "0x344 - SYSCFG user register 17"]
    pub ur17: crate::Reg<ur17::UR17_SPEC>,
}
#[doc = "PMCR register accessor: an alias for `Reg<PMCR_SPEC>`"]
pub type PMCR = crate::Reg<pmcr::PMCR_SPEC>;
#[doc = "peripheral mode configuration register"]
pub mod pmcr;
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
#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "configuration register"]
pub mod cfgr;
#[doc = "CCCSR register accessor: an alias for `Reg<CCCSR_SPEC>`"]
pub type CCCSR = crate::Reg<cccsr::CCCSR_SPEC>;
#[doc = "compensation cell control/status register"]
pub mod cccsr;
#[doc = "CCVR register accessor: an alias for `Reg<CCVR_SPEC>`"]
pub type CCVR = crate::Reg<ccvr::CCVR_SPEC>;
#[doc = "SYSCFG compensation cell value register"]
pub mod ccvr;
#[doc = "CCCR register accessor: an alias for `Reg<CCCR_SPEC>`"]
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
#[doc = "SYSCFG compensation cell code register"]
pub mod cccr;
#[doc = "PWRCR register accessor: an alias for `Reg<PWRCR_SPEC>`"]
pub type PWRCR = crate::Reg<pwrcr::PWRCR_SPEC>;
#[doc = "SYSCFG power control register"]
pub mod pwrcr;
#[doc = "PKGR register accessor: an alias for `Reg<PKGR_SPEC>`"]
pub type PKGR = crate::Reg<pkgr::PKGR_SPEC>;
#[doc = "SYSCFG package register"]
pub mod pkgr;
#[doc = "UR0 register accessor: an alias for `Reg<UR0_SPEC>`"]
pub type UR0 = crate::Reg<ur0::UR0_SPEC>;
#[doc = "SYSCFG user register 0"]
pub mod ur0;
#[doc = "UR1 register accessor: an alias for `Reg<UR1_SPEC>`"]
pub type UR1 = crate::Reg<ur1::UR1_SPEC>;
#[doc = "SYSCFG user register 1"]
pub mod ur1;
#[doc = "UR2 register accessor: an alias for `Reg<UR2_SPEC>`"]
pub type UR2 = crate::Reg<ur2::UR2_SPEC>;
#[doc = "SYSCFG user register 2"]
pub mod ur2;
#[doc = "UR3 register accessor: an alias for `Reg<UR3_SPEC>`"]
pub type UR3 = crate::Reg<ur3::UR3_SPEC>;
#[doc = "SYSCFG user register 3"]
pub mod ur3;
#[doc = "UR4 register accessor: an alias for `Reg<UR4_SPEC>`"]
pub type UR4 = crate::Reg<ur4::UR4_SPEC>;
#[doc = "SYSCFG user register 4"]
pub mod ur4;
#[doc = "UR5 register accessor: an alias for `Reg<UR5_SPEC>`"]
pub type UR5 = crate::Reg<ur5::UR5_SPEC>;
#[doc = "SYSCFG user register 5"]
pub mod ur5;
#[doc = "UR6 register accessor: an alias for `Reg<UR6_SPEC>`"]
pub type UR6 = crate::Reg<ur6::UR6_SPEC>;
#[doc = "SYSCFG user register 6"]
pub mod ur6;
#[doc = "UR7 register accessor: an alias for `Reg<UR7_SPEC>`"]
pub type UR7 = crate::Reg<ur7::UR7_SPEC>;
#[doc = "SYSCFG user register 7"]
pub mod ur7;
#[doc = "UR8 register accessor: an alias for `Reg<UR8_SPEC>`"]
pub type UR8 = crate::Reg<ur8::UR8_SPEC>;
#[doc = "SYSCFG user register 8"]
pub mod ur8;
#[doc = "UR9 register accessor: an alias for `Reg<UR9_SPEC>`"]
pub type UR9 = crate::Reg<ur9::UR9_SPEC>;
#[doc = "SYSCFG user register 9"]
pub mod ur9;
#[doc = "UR10 register accessor: an alias for `Reg<UR10_SPEC>`"]
pub type UR10 = crate::Reg<ur10::UR10_SPEC>;
#[doc = "SYSCFG user register 10"]
pub mod ur10;
#[doc = "UR11 register accessor: an alias for `Reg<UR11_SPEC>`"]
pub type UR11 = crate::Reg<ur11::UR11_SPEC>;
#[doc = "SYSCFG user register 11"]
pub mod ur11;
#[doc = "UR12 register accessor: an alias for `Reg<UR12_SPEC>`"]
pub type UR12 = crate::Reg<ur12::UR12_SPEC>;
#[doc = "SYSCFG user register 12"]
pub mod ur12;
#[doc = "UR13 register accessor: an alias for `Reg<UR13_SPEC>`"]
pub type UR13 = crate::Reg<ur13::UR13_SPEC>;
#[doc = "SYSCFG user register 13"]
pub mod ur13;
#[doc = "UR14 register accessor: an alias for `Reg<UR14_SPEC>`"]
pub type UR14 = crate::Reg<ur14::UR14_SPEC>;
#[doc = "SYSCFG user register 14"]
pub mod ur14;
#[doc = "UR15 register accessor: an alias for `Reg<UR15_SPEC>`"]
pub type UR15 = crate::Reg<ur15::UR15_SPEC>;
#[doc = "SYSCFG user register 15"]
pub mod ur15;
#[doc = "UR16 register accessor: an alias for `Reg<UR16_SPEC>`"]
pub type UR16 = crate::Reg<ur16::UR16_SPEC>;
#[doc = "SYSCFG user register 16"]
pub mod ur16;
#[doc = "UR17 register accessor: an alias for `Reg<UR17_SPEC>`"]
pub type UR17 = crate::Reg<ur17::UR17_SPEC>;
#[doc = "SYSCFG user register 17"]
pub mod ur17;
