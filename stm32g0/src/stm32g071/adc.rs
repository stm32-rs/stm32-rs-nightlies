#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: ISR,
    ier: IER,
    cr: CR,
    cfgr1: CFGR1,
    cfgr2: CFGR2,
    smpr: SMPR,
    _reserved6: [u8; 0x08],
    awd1tr: AWD1TR,
    awd2tr: AWD2TR,
    _reserved_8_chselr0: [u8; 0x04],
    awd3tr: AWD3TR,
    _reserved10: [u8; 0x10],
    dr: DR,
    _reserved11: [u8; 0x5c],
    awd2cr: AWD2CR,
    awd3cr: AWD3CR,
    _reserved13: [u8; 0x0c],
    calfact: CALFACT,
    _reserved14: [u8; 0x0250],
    ccr: CCR,
    _reserved15: [u8; 0xcc],
    hwcfgr6: HWCFGR6,
    hwcfgr5: HWCFGR5,
    hwcfgr4: HWCFGR4,
    hwcfgr3: HWCFGR3,
    hwcfgr2: HWCFGR2,
    hwcfgr1: HWCFGR1,
    hwcfgr0: HWCFGR0,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x04 - ADC interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x08 - ADC control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x0c - ADC configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    #[doc = "0x10 - ADC configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    #[doc = "0x14 - ADC sampling time register"]
    #[inline(always)]
    pub const fn smpr(&self) -> &SMPR {
        &self.smpr
    }
    #[doc = "0x20 - watchdog threshold register"]
    #[inline(always)]
    pub const fn awd1tr(&self) -> &AWD1TR {
        &self.awd1tr
    }
    #[doc = "0x24 - watchdog threshold register"]
    #[inline(always)]
    pub const fn awd2tr(&self) -> &AWD2TR {
        &self.awd2tr
    }
    #[doc = "0x28 - channel selection register CHSELRMOD = 1 in ADC_CFGR1"]
    #[inline(always)]
    pub const fn chselr1(&self) -> &CHSELR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - channel selection register"]
    #[inline(always)]
    pub const fn chselr0(&self) -> &CHSELR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - watchdog threshold register"]
    #[inline(always)]
    pub const fn awd3tr(&self) -> &AWD3TR {
        &self.awd3tr
    }
    #[doc = "0x40 - ADC group regular conversion data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0xa0 - ADC analog watchdog 2 configuration register"]
    #[inline(always)]
    pub const fn awd2cr(&self) -> &AWD2CR {
        &self.awd2cr
    }
    #[doc = "0xa4 - ADC analog watchdog 3 configuration register"]
    #[inline(always)]
    pub const fn awd3cr(&self) -> &AWD3CR {
        &self.awd3cr
    }
    #[doc = "0xb4 - ADC calibration factors register"]
    #[inline(always)]
    pub const fn calfact(&self) -> &CALFACT {
        &self.calfact
    }
    #[doc = "0x308 - ADC common control register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0x3d8 - Hardware Configuration Register"]
    #[inline(always)]
    pub const fn hwcfgr6(&self) -> &HWCFGR6 {
        &self.hwcfgr6
    }
    #[doc = "0x3dc - Hardware Configuration Register"]
    #[inline(always)]
    pub const fn hwcfgr5(&self) -> &HWCFGR5 {
        &self.hwcfgr5
    }
    #[doc = "0x3e0 - Hardware Configuration Register"]
    #[inline(always)]
    pub const fn hwcfgr4(&self) -> &HWCFGR4 {
        &self.hwcfgr4
    }
    #[doc = "0x3e4 - Hardware Configuration Register"]
    #[inline(always)]
    pub const fn hwcfgr3(&self) -> &HWCFGR3 {
        &self.hwcfgr3
    }
    #[doc = "0x3e8 - Hardware Configuration Register"]
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    #[doc = "0x3ec - Hardware Configuration Register"]
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
    #[doc = "0x3f0 - Hardware Configuration Register"]
    #[inline(always)]
    pub const fn hwcfgr0(&self) -> &HWCFGR0 {
        &self.hwcfgr0
    }
    #[doc = "0x3f4 - EXTI IP Version register"]
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    #[doc = "0x3f8 - EXTI Identification register"]
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    #[doc = "0x3fc - EXTI Size ID register"]
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
#[doc = "ISR (rw) register accessor: ADC interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "ADC interrupt and status register"]
pub mod isr;
#[doc = "IER (rw) register accessor: ADC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "ADC interrupt enable register"]
pub mod ier;
#[doc = "CR (rw) register accessor: ADC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "ADC control register"]
pub mod cr;
#[doc = "CFGR1 (rw) register accessor: ADC configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`]
module"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
#[doc = "ADC configuration register 1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: ADC configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
#[doc = "ADC configuration register 2"]
pub mod cfgr2;
#[doc = "SMPR (rw) register accessor: ADC sampling time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr`]
module"]
pub type SMPR = crate::Reg<smpr::SMPRrs>;
#[doc = "ADC sampling time register"]
pub mod smpr;
#[doc = "AWD1TR (rw) register accessor: watchdog threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd1tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd1tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd1tr`]
module"]
pub type AWD1TR = crate::Reg<awd1tr::AWD1TRrs>;
#[doc = "watchdog threshold register"]
pub mod awd1tr;
#[doc = "AWD2TR (rw) register accessor: watchdog threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd2tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd2tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd2tr`]
module"]
pub type AWD2TR = crate::Reg<awd2tr::AWD2TRrs>;
#[doc = "watchdog threshold register"]
pub mod awd2tr;
#[doc = "CHSELR0 (rw) register accessor: channel selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chselr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chselr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chselr0`]
module"]
pub type CHSELR0 = crate::Reg<chselr0::CHSELR0rs>;
#[doc = "channel selection register"]
pub mod chselr0;
#[doc = "CHSELR1 (rw) register accessor: channel selection register CHSELRMOD = 1 in ADC_CFGR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chselr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chselr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chselr1`]
module"]
pub type CHSELR1 = crate::Reg<chselr1::CHSELR1rs>;
#[doc = "channel selection register CHSELRMOD = 1 in ADC_CFGR1"]
pub mod chselr1;
#[doc = "AWD3TR (rw) register accessor: watchdog threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd3tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd3tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd3tr`]
module"]
pub type AWD3TR = crate::Reg<awd3tr::AWD3TRrs>;
#[doc = "watchdog threshold register"]
pub mod awd3tr;
#[doc = "DR (r) register accessor: ADC group regular conversion data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "ADC group regular conversion data register"]
pub mod dr;
#[doc = "AWD2CR (rw) register accessor: ADC analog watchdog 2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd2cr`]
module"]
pub type AWD2CR = crate::Reg<awd2cr::AWD2CRrs>;
#[doc = "ADC analog watchdog 2 configuration register"]
pub mod awd2cr;
#[doc = "AWD3CR (rw) register accessor: ADC analog watchdog 3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd3cr`]
module"]
pub type AWD3CR = crate::Reg<awd3cr::AWD3CRrs>;
#[doc = "ADC analog watchdog 3 configuration register"]
pub mod awd3cr;
#[doc = "CALFACT (rw) register accessor: ADC calibration factors register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calfact::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calfact::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calfact`]
module"]
pub type CALFACT = crate::Reg<calfact::CALFACTrs>;
#[doc = "ADC calibration factors register"]
pub mod calfact;
#[doc = "CCR (rw) register accessor: ADC common control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "ADC common control register"]
pub mod ccr;
#[doc = "HWCFGR6 (rw) register accessor: Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr6`]
module"]
pub type HWCFGR6 = crate::Reg<hwcfgr6::HWCFGR6rs>;
#[doc = "Hardware Configuration Register"]
pub mod hwcfgr6;
#[doc = "HWCFGR5 (rw) register accessor: Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr5`]
module"]
pub type HWCFGR5 = crate::Reg<hwcfgr5::HWCFGR5rs>;
#[doc = "Hardware Configuration Register"]
pub mod hwcfgr5;
#[doc = "HWCFGR4 (rw) register accessor: Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr4`]
module"]
pub type HWCFGR4 = crate::Reg<hwcfgr4::HWCFGR4rs>;
#[doc = "Hardware Configuration Register"]
pub mod hwcfgr4;
#[doc = "HWCFGR3 (rw) register accessor: Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr3`]
module"]
pub type HWCFGR3 = crate::Reg<hwcfgr3::HWCFGR3rs>;
#[doc = "Hardware Configuration Register"]
pub mod hwcfgr3;
#[doc = "HWCFGR2 (rw) register accessor: Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr2`]
module"]
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
#[doc = "Hardware Configuration Register"]
pub mod hwcfgr2;
#[doc = "HWCFGR1 (rw) register accessor: Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr1`]
module"]
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
#[doc = "Hardware Configuration Register"]
pub mod hwcfgr1;
#[doc = "HWCFGR0 (r) register accessor: Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr0`]
module"]
pub type HWCFGR0 = crate::Reg<hwcfgr0::HWCFGR0rs>;
#[doc = "Hardware Configuration Register"]
pub mod hwcfgr0;
#[doc = "VERR (r) register accessor: EXTI IP Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verr`]
module"]
pub type VERR = crate::Reg<verr::VERRrs>;
#[doc = "EXTI IP Version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: EXTI Identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipidr`]
module"]
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
#[doc = "EXTI Identification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: EXTI Size ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidr`]
module"]
pub type SIDR = crate::Reg<sidr::SIDRrs>;
#[doc = "EXTI Size ID register"]
pub mod sidr;
