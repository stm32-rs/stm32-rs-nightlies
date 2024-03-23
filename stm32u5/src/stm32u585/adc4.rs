#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adc_isr: ADC_ISR,
    adc_ier: ADC_IER,
    adc_cr: ADC_CR,
    adc_cfgr1: ADC_CFGR1,
    adc_cfgr2: ADC_CFGR2,
    adc_smpr: ADC_SMPR,
    _reserved6: [u8; 0x08],
    adc_awd1tr: ADC_AWD1TR,
    adc_awd2tr: ADC_AWD2TR,
    _reserved_8_adc_chselrmod: [u8; 0x04],
    adc_awd3tr: ADC_AWD3TR,
    _reserved10: [u8; 0x10],
    adc_dr: ADC_DR,
    adc_pwr: ADC_PWR,
    _reserved12: [u8; 0x58],
    adc_awd2cr: ADC_AWD2CR,
    adc_awd3cr: ADC_AWD3CR,
    _reserved14: [u8; 0x1c],
    adc_calfact: ADC_CALFACT,
    _reserved15: [u8; 0x08],
    adc_or: ADC_OR,
    _reserved16: [u8; 0x0234],
    adc_ccr: ADC_CCR,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    #[inline(always)]
    pub const fn adc_isr(&self) -> &ADC_ISR {
        &self.adc_isr
    }
    #[doc = "0x04 - ADC interrupt enable register"]
    #[inline(always)]
    pub const fn adc_ier(&self) -> &ADC_IER {
        &self.adc_ier
    }
    #[doc = "0x08 - ADC control register"]
    #[inline(always)]
    pub const fn adc_cr(&self) -> &ADC_CR {
        &self.adc_cr
    }
    #[doc = "0x0c - ADC configuration register"]
    #[inline(always)]
    pub const fn adc_cfgr1(&self) -> &ADC_CFGR1 {
        &self.adc_cfgr1
    }
    #[doc = "0x10 - ADC configuration register 2"]
    #[inline(always)]
    pub const fn adc_cfgr2(&self) -> &ADC_CFGR2 {
        &self.adc_cfgr2
    }
    #[doc = "0x14 - ADC sample time register"]
    #[inline(always)]
    pub const fn adc_smpr(&self) -> &ADC_SMPR {
        &self.adc_smpr
    }
    #[doc = "0x20 - ADC watchdog threshold register"]
    #[inline(always)]
    pub const fn adc_awd1tr(&self) -> &ADC_AWD1TR {
        &self.adc_awd1tr
    }
    #[doc = "0x24 - ADC watchdog threshold register"]
    #[inline(always)]
    pub const fn adc_awd2tr(&self) -> &ADC_AWD2TR {
        &self.adc_awd2tr
    }
    #[doc = "0x28 - ADC channel selection register \\[alternate\\]"]
    #[inline(always)]
    pub const fn adc_chselrmod1(&self) -> &ADC_CHSELRMOD1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - ADC channel selection register \\[alternate\\]"]
    #[inline(always)]
    pub const fn adc_chselrmod0(&self) -> &ADC_CHSELRMOD0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - ADC watchdog threshold register"]
    #[inline(always)]
    pub const fn adc_awd3tr(&self) -> &ADC_AWD3TR {
        &self.adc_awd3tr
    }
    #[doc = "0x40 - ADC data register"]
    #[inline(always)]
    pub const fn adc_dr(&self) -> &ADC_DR {
        &self.adc_dr
    }
    #[doc = "0x44 - ADC data register"]
    #[inline(always)]
    pub const fn adc_pwr(&self) -> &ADC_PWR {
        &self.adc_pwr
    }
    #[doc = "0xa0 - ADC Analog Watchdog 2 Configuration register"]
    #[inline(always)]
    pub const fn adc_awd2cr(&self) -> &ADC_AWD2CR {
        &self.adc_awd2cr
    }
    #[doc = "0xa4 - ADC Analog Watchdog 3 Configuration register"]
    #[inline(always)]
    pub const fn adc_awd3cr(&self) -> &ADC_AWD3CR {
        &self.adc_awd3cr
    }
    #[doc = "0xc4 - ADC Calibration factor"]
    #[inline(always)]
    pub const fn adc_calfact(&self) -> &ADC_CALFACT {
        &self.adc_calfact
    }
    #[doc = "0xd0 - ADC option register"]
    #[inline(always)]
    pub const fn adc_or(&self) -> &ADC_OR {
        &self.adc_or
    }
    #[doc = "0x308 - ADC common configuration register"]
    #[inline(always)]
    pub const fn adc_ccr(&self) -> &ADC_CCR {
        &self.adc_ccr
    }
}
#[doc = "ADC_ISR (rw) register accessor: ADC interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_isr`]
module"]
pub type ADC_ISR = crate::Reg<adc_isr::ADC_ISRrs>;
#[doc = "ADC interrupt and status register"]
pub mod adc_isr;
#[doc = "ADC_IER (rw) register accessor: ADC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ier`]
module"]
pub type ADC_IER = crate::Reg<adc_ier::ADC_IERrs>;
#[doc = "ADC interrupt enable register"]
pub mod adc_ier;
#[doc = "ADC_CR (rw) register accessor: ADC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_cr`]
module"]
pub type ADC_CR = crate::Reg<adc_cr::ADC_CRrs>;
#[doc = "ADC control register"]
pub mod adc_cr;
#[doc = "ADC_CFGR1 (rw) register accessor: ADC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_cfgr1`]
module"]
pub type ADC_CFGR1 = crate::Reg<adc_cfgr1::ADC_CFGR1rs>;
#[doc = "ADC configuration register"]
pub mod adc_cfgr1;
#[doc = "ADC_CFGR2 (rw) register accessor: ADC configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_cfgr2`]
module"]
pub type ADC_CFGR2 = crate::Reg<adc_cfgr2::ADC_CFGR2rs>;
#[doc = "ADC configuration register 2"]
pub mod adc_cfgr2;
#[doc = "ADC_SMPR (rw) register accessor: ADC sample time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_smpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_smpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_smpr`]
module"]
pub type ADC_SMPR = crate::Reg<adc_smpr::ADC_SMPRrs>;
#[doc = "ADC sample time register"]
pub mod adc_smpr;
#[doc = "ADC_AWD1TR (rw) register accessor: ADC watchdog threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_awd1tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_awd1tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_awd1tr`]
module"]
pub type ADC_AWD1TR = crate::Reg<adc_awd1tr::ADC_AWD1TRrs>;
#[doc = "ADC watchdog threshold register"]
pub mod adc_awd1tr;
#[doc = "ADC_AWD2TR (rw) register accessor: ADC watchdog threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_awd2tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_awd2tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_awd2tr`]
module"]
pub type ADC_AWD2TR = crate::Reg<adc_awd2tr::ADC_AWD2TRrs>;
#[doc = "ADC watchdog threshold register"]
pub mod adc_awd2tr;
#[doc = "ADC_CHSELRMOD0 (rw) register accessor: ADC channel selection register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_chselrmod0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_chselrmod0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_chselrmod0`]
module"]
pub type ADC_CHSELRMOD0 = crate::Reg<adc_chselrmod0::ADC_CHSELRMOD0rs>;
#[doc = "ADC channel selection register \\[alternate\\]"]
pub mod adc_chselrmod0;
#[doc = "ADC_CHSELRMOD1 (rw) register accessor: ADC channel selection register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_chselrmod1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_chselrmod1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_chselrmod1`]
module"]
pub type ADC_CHSELRMOD1 = crate::Reg<adc_chselrmod1::ADC_CHSELRMOD1rs>;
#[doc = "ADC channel selection register \\[alternate\\]"]
pub mod adc_chselrmod1;
#[doc = "ADC_AWD3TR (rw) register accessor: ADC watchdog threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_awd3tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_awd3tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_awd3tr`]
module"]
pub type ADC_AWD3TR = crate::Reg<adc_awd3tr::ADC_AWD3TRrs>;
#[doc = "ADC watchdog threshold register"]
pub mod adc_awd3tr;
#[doc = "ADC_DR (r) register accessor: ADC data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_dr`]
module"]
pub type ADC_DR = crate::Reg<adc_dr::ADC_DRrs>;
#[doc = "ADC data register"]
pub mod adc_dr;
#[doc = "ADC_PWR (rw) register accessor: ADC data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_pwr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_pwr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_pwr`]
module"]
pub type ADC_PWR = crate::Reg<adc_pwr::ADC_PWRrs>;
#[doc = "ADC data register"]
pub mod adc_pwr;
#[doc = "ADC_AWD2CR (rw) register accessor: ADC Analog Watchdog 2 Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_awd2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_awd2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_awd2cr`]
module"]
pub type ADC_AWD2CR = crate::Reg<adc_awd2cr::ADC_AWD2CRrs>;
#[doc = "ADC Analog Watchdog 2 Configuration register"]
pub mod adc_awd2cr;
#[doc = "ADC_AWD3CR (rw) register accessor: ADC Analog Watchdog 3 Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_awd3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_awd3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_awd3cr`]
module"]
pub type ADC_AWD3CR = crate::Reg<adc_awd3cr::ADC_AWD3CRrs>;
#[doc = "ADC Analog Watchdog 3 Configuration register"]
pub mod adc_awd3cr;
#[doc = "ADC_CALFACT (rw) register accessor: ADC Calibration factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_calfact::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_calfact::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_calfact`]
module"]
pub type ADC_CALFACT = crate::Reg<adc_calfact::ADC_CALFACTrs>;
#[doc = "ADC Calibration factor"]
pub mod adc_calfact;
#[doc = "ADC_OR (rw) register accessor: ADC option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_or::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_or::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_or`]
module"]
pub type ADC_OR = crate::Reg<adc_or::ADC_ORrs>;
#[doc = "ADC option register"]
pub mod adc_or;
#[doc = "ADC_CCR (rw) register accessor: ADC common configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ccr`]
module"]
pub type ADC_CCR = crate::Reg<adc_ccr::ADC_CCRrs>;
#[doc = "ADC common configuration register"]
pub mod adc_ccr;
