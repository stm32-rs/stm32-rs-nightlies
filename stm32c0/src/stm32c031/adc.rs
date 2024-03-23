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
    #[doc = "0x20 - ADC watchdog threshold register"]
    #[inline(always)]
    pub const fn awd1tr(&self) -> &AWD1TR {
        &self.awd1tr
    }
    #[doc = "0x24 - ADC watchdog threshold register"]
    #[inline(always)]
    pub const fn awd2tr(&self) -> &AWD2TR {
        &self.awd2tr
    }
    #[doc = "0x28 - ADC channel selection register \\[alternate\\]"]
    #[inline(always)]
    pub const fn chselr1(&self) -> &CHSELR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - ADC channel selection register \\[alternate\\]"]
    #[inline(always)]
    pub const fn chselr0(&self) -> &CHSELR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - ADC watchdog threshold register"]
    #[inline(always)]
    pub const fn awd3tr(&self) -> &AWD3TR {
        &self.awd3tr
    }
    #[doc = "0x40 - ADC data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0xa0 - ADC Analog Watchdog 2 Configuration register"]
    #[inline(always)]
    pub const fn awd2cr(&self) -> &AWD2CR {
        &self.awd2cr
    }
    #[doc = "0xa4 - ADC Analog Watchdog 3 Configuration register"]
    #[inline(always)]
    pub const fn awd3cr(&self) -> &AWD3CR {
        &self.awd3cr
    }
    #[doc = "0xb4 - ADC Calibration factor"]
    #[inline(always)]
    pub const fn calfact(&self) -> &CALFACT {
        &self.calfact
    }
    #[doc = "0x308 - ADC common configuration register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
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
#[doc = "AWD1TR (rw) register accessor: ADC watchdog threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd1tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd1tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd1tr`]
module"]
pub type AWD1TR = crate::Reg<awd1tr::AWD1TRrs>;
#[doc = "ADC watchdog threshold register"]
pub mod awd1tr;
#[doc = "AWD2TR (rw) register accessor: ADC watchdog threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd2tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd2tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd2tr`]
module"]
pub type AWD2TR = crate::Reg<awd2tr::AWD2TRrs>;
#[doc = "ADC watchdog threshold register"]
pub mod awd2tr;
#[doc = "CHSELR0 (rw) register accessor: ADC channel selection register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chselr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chselr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chselr0`]
module"]
pub type CHSELR0 = crate::Reg<chselr0::CHSELR0rs>;
#[doc = "ADC channel selection register \\[alternate\\]"]
pub mod chselr0;
#[doc = "CHSELR1 (rw) register accessor: ADC channel selection register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chselr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chselr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chselr1`]
module"]
pub type CHSELR1 = crate::Reg<chselr1::CHSELR1rs>;
#[doc = "ADC channel selection register \\[alternate\\]"]
pub mod chselr1;
#[doc = "AWD3TR (rw) register accessor: ADC watchdog threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd3tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd3tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd3tr`]
module"]
pub type AWD3TR = crate::Reg<awd3tr::AWD3TRrs>;
#[doc = "ADC watchdog threshold register"]
pub mod awd3tr;
#[doc = "DR (r) register accessor: ADC data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "ADC data register"]
pub mod dr;
#[doc = "AWD2CR (rw) register accessor: ADC Analog Watchdog 2 Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd2cr`]
module"]
pub type AWD2CR = crate::Reg<awd2cr::AWD2CRrs>;
#[doc = "ADC Analog Watchdog 2 Configuration register"]
pub mod awd2cr;
#[doc = "AWD3CR (rw) register accessor: ADC Analog Watchdog 3 Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd3cr`]
module"]
pub type AWD3CR = crate::Reg<awd3cr::AWD3CRrs>;
#[doc = "ADC Analog Watchdog 3 Configuration register"]
pub mod awd3cr;
#[doc = "CALFACT (rw) register accessor: ADC Calibration factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calfact::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calfact::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calfact`]
module"]
pub type CALFACT = crate::Reg<calfact::CALFACTrs>;
#[doc = "ADC Calibration factor"]
pub mod calfact;
#[doc = "CCR (rw) register accessor: ADC common configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "ADC common configuration register"]
pub mod ccr;
