#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    pmcr: PMCR,
    exticr1: EXTICR1,
    exticr2: EXTICR2,
    exticr3: EXTICR3,
    exticr4: EXTICR4,
    cfgr: CFGR,
    _reserved6: [u8; 0x04],
    cccsr: CCCSR,
    ccvr: CCVR,
    cccr: CCCR,
    _reserved9: [u8; 0x04],
    adc2alt: ADC2ALT,
    _reserved10: [u8; 0xf0],
    pkgr: PKGR,
    _reserved11: [u8; 0x01d8],
    ur0: UR0,
    _reserved12: [u8; 0x04],
    ur2: UR2,
    ur3: UR3,
    ur4: UR4,
    ur5: UR5,
    ur6: UR6,
    ur7: UR7,
    _reserved18: [u8; 0x0c],
    ur11: UR11,
    ur12: UR12,
    ur13: UR13,
    ur14: UR14,
    ur15: UR15,
    ur16: UR16,
    ur17: UR17,
    ur18: UR18,
}
impl RegisterBlock {
    #[doc = "0x04 - peripheral mode configuration register"]
    #[inline(always)]
    pub const fn pmcr(&self) -> &PMCR {
        &self.pmcr
    }
    #[doc = "0x08 - external interrupt configuration register 1"]
    #[inline(always)]
    pub const fn exticr1(&self) -> &EXTICR1 {
        &self.exticr1
    }
    #[doc = "0x0c - external interrupt configuration register 2"]
    #[inline(always)]
    pub const fn exticr2(&self) -> &EXTICR2 {
        &self.exticr2
    }
    #[doc = "0x10 - external interrupt configuration register 3"]
    #[inline(always)]
    pub const fn exticr3(&self) -> &EXTICR3 {
        &self.exticr3
    }
    #[doc = "0x14 - external interrupt configuration register 4"]
    #[inline(always)]
    pub const fn exticr4(&self) -> &EXTICR4 {
        &self.exticr4
    }
    #[doc = "0x18 - Timer break lockup register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x20 - compensation cell control/status register"]
    #[inline(always)]
    pub const fn cccsr(&self) -> &CCCSR {
        &self.cccsr
    }
    #[doc = "0x24 - SYSCFG compensation cell value register"]
    #[inline(always)]
    pub const fn ccvr(&self) -> &CCVR {
        &self.ccvr
    }
    #[doc = "0x28 - SYSCFG compensation cell code register"]
    #[inline(always)]
    pub const fn cccr(&self) -> &CCCR {
        &self.cccr
    }
    #[doc = "0x30 - ADC2 internal input alternate connection"]
    #[inline(always)]
    pub const fn adc2alt(&self) -> &ADC2ALT {
        &self.adc2alt
    }
    #[doc = "0x124 - SYSCFG package register"]
    #[inline(always)]
    pub const fn pkgr(&self) -> &PKGR {
        &self.pkgr
    }
    #[doc = "0x300 - SYSCFG user register 0"]
    #[inline(always)]
    pub const fn ur0(&self) -> &UR0 {
        &self.ur0
    }
    #[doc = "0x308 - SYSCFG user register 2"]
    #[inline(always)]
    pub const fn ur2(&self) -> &UR2 {
        &self.ur2
    }
    #[doc = "0x30c - SYSCFG user register 3"]
    #[inline(always)]
    pub const fn ur3(&self) -> &UR3 {
        &self.ur3
    }
    #[doc = "0x310 - SYSCFG user register 4"]
    #[inline(always)]
    pub const fn ur4(&self) -> &UR4 {
        &self.ur4
    }
    #[doc = "0x314 - SYSCFG user register 5"]
    #[inline(always)]
    pub const fn ur5(&self) -> &UR5 {
        &self.ur5
    }
    #[doc = "0x318 - SYSCFG user register 6"]
    #[inline(always)]
    pub const fn ur6(&self) -> &UR6 {
        &self.ur6
    }
    #[doc = "0x31c - SYSCFG user register 7"]
    #[inline(always)]
    pub const fn ur7(&self) -> &UR7 {
        &self.ur7
    }
    #[doc = "0x32c - SYSCFG user register 11"]
    #[inline(always)]
    pub const fn ur11(&self) -> &UR11 {
        &self.ur11
    }
    #[doc = "0x330 - SYSCFG user register 12"]
    #[inline(always)]
    pub const fn ur12(&self) -> &UR12 {
        &self.ur12
    }
    #[doc = "0x334 - SYSCFG user register 13"]
    #[inline(always)]
    pub const fn ur13(&self) -> &UR13 {
        &self.ur13
    }
    #[doc = "0x338 - SYSCFG user register 14"]
    #[inline(always)]
    pub const fn ur14(&self) -> &UR14 {
        &self.ur14
    }
    #[doc = "0x33c - SYSCFG user register 15"]
    #[inline(always)]
    pub const fn ur15(&self) -> &UR15 {
        &self.ur15
    }
    #[doc = "0x340 - SYSCFG user register 16"]
    #[inline(always)]
    pub const fn ur16(&self) -> &UR16 {
        &self.ur16
    }
    #[doc = "0x344 - SYSCFG user register 17"]
    #[inline(always)]
    pub const fn ur17(&self) -> &UR17 {
        &self.ur17
    }
    #[doc = "0x348 - SYSCFG user register 18"]
    #[inline(always)]
    pub const fn ur18(&self) -> &UR18 {
        &self.ur18
    }
}
#[doc = "PMCR (rw) register accessor: peripheral mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmcr`]
module"]
pub type PMCR = crate::Reg<pmcr::PMCRrs>;
#[doc = "peripheral mode configuration register"]
pub mod pmcr;
#[doc = "EXTICR1 (rw) register accessor: external interrupt configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr1`]
module"]
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1rs>;
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "EXTICR2 (rw) register accessor: external interrupt configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr2`]
module"]
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2rs>;
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "EXTICR3 (rw) register accessor: external interrupt configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr3`]
module"]
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3rs>;
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "EXTICR4 (rw) register accessor: external interrupt configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr4`]
module"]
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4rs>;
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "CCCSR (rw) register accessor: compensation cell control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccsr`]
module"]
pub type CCCSR = crate::Reg<cccsr::CCCSRrs>;
#[doc = "compensation cell control/status register"]
pub mod cccsr;
#[doc = "CCVR (r) register accessor: SYSCFG compensation cell value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccvr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccvr`]
module"]
pub type CCVR = crate::Reg<ccvr::CCVRrs>;
#[doc = "SYSCFG compensation cell value register"]
pub mod ccvr;
#[doc = "CCCR (rw) register accessor: SYSCFG compensation cell code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccr`]
module"]
pub type CCCR = crate::Reg<cccr::CCCRrs>;
#[doc = "SYSCFG compensation cell code register"]
pub mod cccr;
#[doc = "PKGR (r) register accessor: SYSCFG package register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pkgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkgr`]
module"]
pub type PKGR = crate::Reg<pkgr::PKGRrs>;
#[doc = "SYSCFG package register"]
pub mod pkgr;
#[doc = "UR0 (r) register accessor: SYSCFG user register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur0`]
module"]
pub type UR0 = crate::Reg<ur0::UR0rs>;
#[doc = "SYSCFG user register 0"]
pub mod ur0;
#[doc = "UR2 (rw) register accessor: SYSCFG user register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur2`]
module"]
pub type UR2 = crate::Reg<ur2::UR2rs>;
#[doc = "SYSCFG user register 2"]
pub mod ur2;
#[doc = "UR3 (rw) register accessor: SYSCFG user register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur3`]
module"]
pub type UR3 = crate::Reg<ur3::UR3rs>;
#[doc = "SYSCFG user register 3"]
pub mod ur3;
#[doc = "UR4 (r) register accessor: SYSCFG user register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur4`]
module"]
pub type UR4 = crate::Reg<ur4::UR4rs>;
#[doc = "SYSCFG user register 4"]
pub mod ur4;
#[doc = "UR5 (r) register accessor: SYSCFG user register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur5`]
module"]
pub type UR5 = crate::Reg<ur5::UR5rs>;
#[doc = "SYSCFG user register 5"]
pub mod ur5;
#[doc = "UR6 (r) register accessor: SYSCFG user register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur6`]
module"]
pub type UR6 = crate::Reg<ur6::UR6rs>;
#[doc = "SYSCFG user register 6"]
pub mod ur6;
#[doc = "UR7 (r) register accessor: SYSCFG user register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur7`]
module"]
pub type UR7 = crate::Reg<ur7::UR7rs>;
#[doc = "SYSCFG user register 7"]
pub mod ur7;
#[doc = "UR11 (r) register accessor: SYSCFG user register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur11`]
module"]
pub type UR11 = crate::Reg<ur11::UR11rs>;
#[doc = "SYSCFG user register 11"]
pub mod ur11;
#[doc = "UR12 (r) register accessor: SYSCFG user register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur12`]
module"]
pub type UR12 = crate::Reg<ur12::UR12rs>;
#[doc = "SYSCFG user register 12"]
pub mod ur12;
#[doc = "UR13 (r) register accessor: SYSCFG user register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur13`]
module"]
pub type UR13 = crate::Reg<ur13::UR13rs>;
#[doc = "SYSCFG user register 13"]
pub mod ur13;
#[doc = "UR14 (rw) register accessor: SYSCFG user register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur14`]
module"]
pub type UR14 = crate::Reg<ur14::UR14rs>;
#[doc = "SYSCFG user register 14"]
pub mod ur14;
#[doc = "UR15 (r) register accessor: SYSCFG user register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur15::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur15`]
module"]
pub type UR15 = crate::Reg<ur15::UR15rs>;
#[doc = "SYSCFG user register 15"]
pub mod ur15;
#[doc = "UR16 (r) register accessor: SYSCFG user register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur16::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur16`]
module"]
pub type UR16 = crate::Reg<ur16::UR16rs>;
#[doc = "SYSCFG user register 16"]
pub mod ur16;
#[doc = "UR17 (r) register accessor: SYSCFG user register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur17::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur17`]
module"]
pub type UR17 = crate::Reg<ur17::UR17rs>;
#[doc = "SYSCFG user register 17"]
pub mod ur17;
#[doc = "CFGR (rw) register accessor: Timer break lockup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
#[doc = "Timer break lockup register"]
pub mod cfgr;
#[doc = "ADC2ALT (rw) register accessor: ADC2 internal input alternate connection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc2alt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc2alt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc2alt`]
module"]
pub type ADC2ALT = crate::Reg<adc2alt::ADC2ALTrs>;
#[doc = "ADC2 internal input alternate connection"]
pub mod adc2alt;
#[doc = "UR18 (r) register accessor: SYSCFG user register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur18::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur18`]
module"]
pub type UR18 = crate::Reg<ur18::UR18rs>;
#[doc = "SYSCFG user register 18"]
pub mod ur18;
