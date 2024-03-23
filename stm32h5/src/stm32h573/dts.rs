#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfgr1: CFGR1,
    _reserved1: [u8; 0x04],
    t0valr1: T0VALR1,
    _reserved2: [u8; 0x04],
    rampvalr: RAMPVALR,
    itr1: ITR1,
    _reserved4: [u8; 0x04],
    dr: DR,
    sr: SR,
    itenr: ITENR,
    icifr: ICIFR,
    or: OR,
}
impl RegisterBlock {
    #[doc = "0x00 - Temperature sensor configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    #[doc = "0x08 - Temperature sensor T0 value register 1"]
    #[inline(always)]
    pub const fn t0valr1(&self) -> &T0VALR1 {
        &self.t0valr1
    }
    #[doc = "0x10 - Temperature sensor ramp value register"]
    #[inline(always)]
    pub const fn rampvalr(&self) -> &RAMPVALR {
        &self.rampvalr
    }
    #[doc = "0x14 - Temperature sensor interrupt threshold register 1"]
    #[inline(always)]
    pub const fn itr1(&self) -> &ITR1 {
        &self.itr1
    }
    #[doc = "0x1c - Temperature sensor data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x20 - Temperature sensor status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x24 - Temperature sensor interrupt enable register"]
    #[inline(always)]
    pub const fn itenr(&self) -> &ITENR {
        &self.itenr
    }
    #[doc = "0x28 - Temperature sensor clear interrupt flag register"]
    #[inline(always)]
    pub const fn icifr(&self) -> &ICIFR {
        &self.icifr
    }
    #[doc = "0x2c - Temperature sensor option register"]
    #[inline(always)]
    pub const fn or(&self) -> &OR {
        &self.or
    }
}
#[doc = "CFGR1 (rw) register accessor: Temperature sensor configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`]
module"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
#[doc = "Temperature sensor configuration register 1"]
pub mod cfgr1;
#[doc = "T0VALR1 (r) register accessor: Temperature sensor T0 value register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0valr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0valr1`]
module"]
pub type T0VALR1 = crate::Reg<t0valr1::T0VALR1rs>;
#[doc = "Temperature sensor T0 value register 1"]
pub mod t0valr1;
#[doc = "RAMPVALR (r) register accessor: Temperature sensor ramp value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rampvalr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rampvalr`]
module"]
pub type RAMPVALR = crate::Reg<rampvalr::RAMPVALRrs>;
#[doc = "Temperature sensor ramp value register"]
pub mod rampvalr;
#[doc = "ITR1 (rw) register accessor: Temperature sensor interrupt threshold register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itr1`]
module"]
pub type ITR1 = crate::Reg<itr1::ITR1rs>;
#[doc = "Temperature sensor interrupt threshold register 1"]
pub mod itr1;
#[doc = "DR (rw) register accessor: Temperature sensor data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "Temperature sensor data register"]
pub mod dr;
#[doc = "SR (r) register accessor: Temperature sensor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "Temperature sensor status register"]
pub mod sr;
#[doc = "ITENR (rw) register accessor: Temperature sensor interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itenr`]
module"]
pub type ITENR = crate::Reg<itenr::ITENRrs>;
#[doc = "Temperature sensor interrupt enable register"]
pub mod itenr;
#[doc = "ICIFR (rw) register accessor: Temperature sensor clear interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icifr`]
module"]
pub type ICIFR = crate::Reg<icifr::ICIFRrs>;
#[doc = "Temperature sensor clear interrupt flag register"]
pub mod icifr;
#[doc = "OR (rw) register accessor: Temperature sensor option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@or`]
module"]
pub type OR = crate::Reg<or::ORrs>;
#[doc = "Temperature sensor option register"]
pub mod or;
