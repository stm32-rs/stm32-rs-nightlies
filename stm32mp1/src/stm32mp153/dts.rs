#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dts_cfgr1: DTS_CFGR1,
    _reserved1: [u8; 0x04],
    dts_t0valr1: DTS_T0VALR1,
    _reserved2: [u8; 0x04],
    dts_rampvalr: DTS_RAMPVALR,
    dts_itr1: DTS_ITR1,
    _reserved4: [u8; 0x04],
    dts_dr: DTS_DR,
    dts_sr: DTS_SR,
    dts_itenr: DTS_ITENR,
    dts_icifr: DTS_ICIFR,
    dts_or: DTS_OR,
}
impl RegisterBlock {
    #[doc = "0x00 - DTS_CFGR1 is the configuration register for temperature sensor 1."]
    #[inline(always)]
    pub const fn dts_cfgr1(&self) -> &DTS_CFGR1 {
        &self.dts_cfgr1
    }
    #[doc = "0x08 - DTS_T0VALR1 contains the value of the factory calibration temperature (T0) for temperature sensor 1. The system reset value is factory trimmed."]
    #[inline(always)]
    pub const fn dts_t0valr1(&self) -> &DTS_T0VALR1 {
        &self.dts_t0valr1
    }
    #[doc = "0x10 - The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed."]
    #[inline(always)]
    pub const fn dts_rampvalr(&self) -> &DTS_RAMPVALR {
        &self.dts_rampvalr
    }
    #[doc = "0x14 - DTS_ITR1 contains the threshold values for sensor 1."]
    #[inline(always)]
    pub const fn dts_itr1(&self) -> &DTS_ITR1 {
        &self.dts_itr1
    }
    #[doc = "0x1c - The DTS_DR contains the number of REF_CLK cycles used to compute the FM(T) frequency."]
    #[inline(always)]
    pub const fn dts_dr(&self) -> &DTS_DR {
        &self.dts_dr
    }
    #[doc = "0x20 - Temperature sensor status register"]
    #[inline(always)]
    pub const fn dts_sr(&self) -> &DTS_SR {
        &self.dts_sr
    }
    #[doc = "0x24 - Temperature sensor interrupt enable register"]
    #[inline(always)]
    pub const fn dts_itenr(&self) -> &DTS_ITENR {
        &self.dts_itenr
    }
    #[doc = "0x28 - DTS_ICIFR is the control register for the interrupt flags."]
    #[inline(always)]
    pub const fn dts_icifr(&self) -> &DTS_ICIFR {
        &self.dts_icifr
    }
    #[doc = "0x2c - The DTS_OR contains general-purpose option bits."]
    #[inline(always)]
    pub const fn dts_or(&self) -> &DTS_OR {
        &self.dts_or
    }
}
#[doc = "DTS_CFGR1 (rw) register accessor: DTS_CFGR1 is the configuration register for temperature sensor 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts_cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dts_cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dts_cfgr1`]
module"]
pub type DTS_CFGR1 = crate::Reg<dts_cfgr1::DTS_CFGR1rs>;
#[doc = "DTS_CFGR1 is the configuration register for temperature sensor 1."]
pub mod dts_cfgr1;
#[doc = "DTS_T0VALR1 (r) register accessor: DTS_T0VALR1 contains the value of the factory calibration temperature (T0) for temperature sensor 1. The system reset value is factory trimmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts_t0valr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dts_t0valr1`]
module"]
pub type DTS_T0VALR1 = crate::Reg<dts_t0valr1::DTS_T0VALR1rs>;
#[doc = "DTS_T0VALR1 contains the value of the factory calibration temperature (T0) for temperature sensor 1. The system reset value is factory trimmed."]
pub mod dts_t0valr1;
#[doc = "DTS_RAMPVALR (r) register accessor: The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts_rampvalr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dts_rampvalr`]
module"]
pub type DTS_RAMPVALR = crate::Reg<dts_rampvalr::DTS_RAMPVALRrs>;
#[doc = "The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed."]
pub mod dts_rampvalr;
#[doc = "DTS_ITR1 (rw) register accessor: DTS_ITR1 contains the threshold values for sensor 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts_itr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dts_itr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dts_itr1`]
module"]
pub type DTS_ITR1 = crate::Reg<dts_itr1::DTS_ITR1rs>;
#[doc = "DTS_ITR1 contains the threshold values for sensor 1."]
pub mod dts_itr1;
#[doc = "DTS_DR (rw) register accessor: The DTS_DR contains the number of REF_CLK cycles used to compute the FM(T) frequency.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dts_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dts_dr`]
module"]
pub type DTS_DR = crate::Reg<dts_dr::DTS_DRrs>;
#[doc = "The DTS_DR contains the number of REF_CLK cycles used to compute the FM(T) frequency."]
pub mod dts_dr;
#[doc = "DTS_SR (r) register accessor: Temperature sensor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dts_sr`]
module"]
pub type DTS_SR = crate::Reg<dts_sr::DTS_SRrs>;
#[doc = "Temperature sensor status register"]
pub mod dts_sr;
#[doc = "DTS_ITENR (rw) register accessor: Temperature sensor interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts_itenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dts_itenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dts_itenr`]
module"]
pub type DTS_ITENR = crate::Reg<dts_itenr::DTS_ITENRrs>;
#[doc = "Temperature sensor interrupt enable register"]
pub mod dts_itenr;
#[doc = "DTS_ICIFR (rw) register accessor: DTS_ICIFR is the control register for the interrupt flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts_icifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dts_icifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dts_icifr`]
module"]
pub type DTS_ICIFR = crate::Reg<dts_icifr::DTS_ICIFRrs>;
#[doc = "DTS_ICIFR is the control register for the interrupt flags."]
pub mod dts_icifr;
#[doc = "DTS_OR (rw) register accessor: The DTS_OR contains general-purpose option bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts_or::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dts_or::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dts_or`]
module"]
pub type DTS_OR = crate::Reg<dts_or::DTS_ORrs>;
#[doc = "The DTS_OR contains general-purpose option bits."]
pub mod dts_or;
