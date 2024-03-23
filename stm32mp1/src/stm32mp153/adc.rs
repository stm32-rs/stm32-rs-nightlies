#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adc_isr: ADC_ISR,
    adc_ier: ADC_IER,
    adc_cr: ADC_CR,
    adc_cfgr: ADC_CFGR,
    adc_cfgr2: ADC_CFGR2,
    adc_smpr1: ADC_SMPR1,
    adc_smpr2: ADC_SMPR2,
    adc_pcsel: ADC_PCSEL,
    adc_ltr1: ADC_LTR1,
    adc_htr1: ADC_HTR1,
    _reserved10: [u8; 0x08],
    adc_sqr1: ADC_SQR1,
    adc_sqr2: ADC_SQR2,
    adc_sqr3: ADC_SQR3,
    adc_sqr4: ADC_SQR4,
    adc_dr: ADC_DR,
    _reserved15: [u8; 0x08],
    adc_jsqr: ADC_JSQR,
    _reserved16: [u8; 0x10],
    adc_ofr1: ADC_OFR1,
    adc_ofr2: ADC_OFR2,
    adc_ofr3: ADC_OFR3,
    adc_ofr4: ADC_OFR4,
    _reserved20: [u8; 0x10],
    adc_jdr1: ADC_JDR1,
    adc_jdr2: ADC_JDR2,
    adc_jdr3: ADC_JDR3,
    adc_jdr4: ADC_JDR4,
    _reserved24: [u8; 0x10],
    adc_awd2cr: ADC_AWD2CR,
    adc_awd3cr: ADC_AWD3CR,
    _reserved26: [u8; 0x08],
    adc_ltr2: ADC_LTR2,
    adc_htr2: ADC_HTR2,
    adc_ltr3: ADC_LTR3,
    adc_htr3: ADC_HTR3,
    adc_difsel: ADC_DIFSEL,
    adc_calfact: ADC_CALFACT,
    adc_calfact2: ADC_CALFACT2,
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
    pub const fn adc_cfgr(&self) -> &ADC_CFGR {
        &self.adc_cfgr
    }
    #[doc = "0x10 - ADC configuration register 2"]
    #[inline(always)]
    pub const fn adc_cfgr2(&self) -> &ADC_CFGR2 {
        &self.adc_cfgr2
    }
    #[doc = "0x14 - ADC sample time register 1"]
    #[inline(always)]
    pub const fn adc_smpr1(&self) -> &ADC_SMPR1 {
        &self.adc_smpr1
    }
    #[doc = "0x18 - ADC sample time register 2"]
    #[inline(always)]
    pub const fn adc_smpr2(&self) -> &ADC_SMPR2 {
        &self.adc_smpr2
    }
    #[doc = "0x1c - ADC channel preselection register"]
    #[inline(always)]
    pub const fn adc_pcsel(&self) -> &ADC_PCSEL {
        &self.adc_pcsel
    }
    #[doc = "0x20 - ADC watchdog threshold register 1"]
    #[inline(always)]
    pub const fn adc_ltr1(&self) -> &ADC_LTR1 {
        &self.adc_ltr1
    }
    #[doc = "0x24 - ADC watchdog threshold register 1"]
    #[inline(always)]
    pub const fn adc_htr1(&self) -> &ADC_HTR1 {
        &self.adc_htr1
    }
    #[doc = "0x30 - ADC regular sequence register 1"]
    #[inline(always)]
    pub const fn adc_sqr1(&self) -> &ADC_SQR1 {
        &self.adc_sqr1
    }
    #[doc = "0x34 - ADC regular sequence register 2"]
    #[inline(always)]
    pub const fn adc_sqr2(&self) -> &ADC_SQR2 {
        &self.adc_sqr2
    }
    #[doc = "0x38 - ADC regular sequence register 3"]
    #[inline(always)]
    pub const fn adc_sqr3(&self) -> &ADC_SQR3 {
        &self.adc_sqr3
    }
    #[doc = "0x3c - ADC regular sequence register 4"]
    #[inline(always)]
    pub const fn adc_sqr4(&self) -> &ADC_SQR4 {
        &self.adc_sqr4
    }
    #[doc = "0x40 - ADC regular Data Register"]
    #[inline(always)]
    pub const fn adc_dr(&self) -> &ADC_DR {
        &self.adc_dr
    }
    #[doc = "0x4c - ADC injected sequence register"]
    #[inline(always)]
    pub const fn adc_jsqr(&self) -> &ADC_JSQR {
        &self.adc_jsqr
    }
    #[doc = "0x60 - ADC offset register"]
    #[inline(always)]
    pub const fn adc_ofr1(&self) -> &ADC_OFR1 {
        &self.adc_ofr1
    }
    #[doc = "0x64 - ADC offset register"]
    #[inline(always)]
    pub const fn adc_ofr2(&self) -> &ADC_OFR2 {
        &self.adc_ofr2
    }
    #[doc = "0x68 - ADC offset register"]
    #[inline(always)]
    pub const fn adc_ofr3(&self) -> &ADC_OFR3 {
        &self.adc_ofr3
    }
    #[doc = "0x6c - ADC offset register"]
    #[inline(always)]
    pub const fn adc_ofr4(&self) -> &ADC_OFR4 {
        &self.adc_ofr4
    }
    #[doc = "0x80 - ADC injected data register"]
    #[inline(always)]
    pub const fn adc_jdr1(&self) -> &ADC_JDR1 {
        &self.adc_jdr1
    }
    #[doc = "0x84 - ADC injected data register"]
    #[inline(always)]
    pub const fn adc_jdr2(&self) -> &ADC_JDR2 {
        &self.adc_jdr2
    }
    #[doc = "0x88 - ADC injected data register"]
    #[inline(always)]
    pub const fn adc_jdr3(&self) -> &ADC_JDR3 {
        &self.adc_jdr3
    }
    #[doc = "0x8c - ADC injected data register"]
    #[inline(always)]
    pub const fn adc_jdr4(&self) -> &ADC_JDR4 {
        &self.adc_jdr4
    }
    #[doc = "0xa0 - ADC analog watchdog 2 configuration register"]
    #[inline(always)]
    pub const fn adc_awd2cr(&self) -> &ADC_AWD2CR {
        &self.adc_awd2cr
    }
    #[doc = "0xa4 - ADC analog watchdog 3 configuration register"]
    #[inline(always)]
    pub const fn adc_awd3cr(&self) -> &ADC_AWD3CR {
        &self.adc_awd3cr
    }
    #[doc = "0xb0 - ADC watchdog lower threshold register 2"]
    #[inline(always)]
    pub const fn adc_ltr2(&self) -> &ADC_LTR2 {
        &self.adc_ltr2
    }
    #[doc = "0xb4 - ADC watchdog higher threshold register 2"]
    #[inline(always)]
    pub const fn adc_htr2(&self) -> &ADC_HTR2 {
        &self.adc_htr2
    }
    #[doc = "0xb8 - ADC watchdog lower threshold register 3"]
    #[inline(always)]
    pub const fn adc_ltr3(&self) -> &ADC_LTR3 {
        &self.adc_ltr3
    }
    #[doc = "0xbc - ADC watchdog higher threshold register 3"]
    #[inline(always)]
    pub const fn adc_htr3(&self) -> &ADC_HTR3 {
        &self.adc_htr3
    }
    #[doc = "0xc0 - ADC differential mode selection register"]
    #[inline(always)]
    pub const fn adc_difsel(&self) -> &ADC_DIFSEL {
        &self.adc_difsel
    }
    #[doc = "0xc4 - ADC calibration factors register"]
    #[inline(always)]
    pub const fn adc_calfact(&self) -> &ADC_CALFACT {
        &self.adc_calfact
    }
    #[doc = "0xc8 - ADC calibration factor register 2"]
    #[inline(always)]
    pub const fn adc_calfact2(&self) -> &ADC_CALFACT2 {
        &self.adc_calfact2
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
#[doc = "ADC_CFGR (rw) register accessor: ADC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_cfgr`]
module"]
pub type ADC_CFGR = crate::Reg<adc_cfgr::ADC_CFGRrs>;
#[doc = "ADC configuration register"]
pub mod adc_cfgr;
#[doc = "ADC_CFGR2 (rw) register accessor: ADC configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_cfgr2`]
module"]
pub type ADC_CFGR2 = crate::Reg<adc_cfgr2::ADC_CFGR2rs>;
#[doc = "ADC configuration register 2"]
pub mod adc_cfgr2;
#[doc = "ADC_SMPR1 (rw) register accessor: ADC sample time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_smpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_smpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_smpr1`]
module"]
pub type ADC_SMPR1 = crate::Reg<adc_smpr1::ADC_SMPR1rs>;
#[doc = "ADC sample time register 1"]
pub mod adc_smpr1;
#[doc = "ADC_SMPR2 (rw) register accessor: ADC sample time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_smpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_smpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_smpr2`]
module"]
pub type ADC_SMPR2 = crate::Reg<adc_smpr2::ADC_SMPR2rs>;
#[doc = "ADC sample time register 2"]
pub mod adc_smpr2;
#[doc = "ADC_PCSEL (rw) register accessor: ADC channel preselection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_pcsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_pcsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_pcsel`]
module"]
pub type ADC_PCSEL = crate::Reg<adc_pcsel::ADC_PCSELrs>;
#[doc = "ADC channel preselection register"]
pub mod adc_pcsel;
#[doc = "ADC_LTR1 (rw) register accessor: ADC watchdog threshold register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ltr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ltr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ltr1`]
module"]
pub type ADC_LTR1 = crate::Reg<adc_ltr1::ADC_LTR1rs>;
#[doc = "ADC watchdog threshold register 1"]
pub mod adc_ltr1;
#[doc = "ADC_HTR1 (rw) register accessor: ADC watchdog threshold register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_htr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_htr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_htr1`]
module"]
pub type ADC_HTR1 = crate::Reg<adc_htr1::ADC_HTR1rs>;
#[doc = "ADC watchdog threshold register 1"]
pub mod adc_htr1;
#[doc = "ADC_SQR1 (rw) register accessor: ADC regular sequence register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_sqr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_sqr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_sqr1`]
module"]
pub type ADC_SQR1 = crate::Reg<adc_sqr1::ADC_SQR1rs>;
#[doc = "ADC regular sequence register 1"]
pub mod adc_sqr1;
#[doc = "ADC_SQR2 (rw) register accessor: ADC regular sequence register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_sqr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_sqr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_sqr2`]
module"]
pub type ADC_SQR2 = crate::Reg<adc_sqr2::ADC_SQR2rs>;
#[doc = "ADC regular sequence register 2"]
pub mod adc_sqr2;
#[doc = "ADC_SQR3 (rw) register accessor: ADC regular sequence register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_sqr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_sqr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_sqr3`]
module"]
pub type ADC_SQR3 = crate::Reg<adc_sqr3::ADC_SQR3rs>;
#[doc = "ADC regular sequence register 3"]
pub mod adc_sqr3;
#[doc = "ADC_SQR4 (rw) register accessor: ADC regular sequence register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_sqr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_sqr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_sqr4`]
module"]
pub type ADC_SQR4 = crate::Reg<adc_sqr4::ADC_SQR4rs>;
#[doc = "ADC regular sequence register 4"]
pub mod adc_sqr4;
#[doc = "ADC_DR (r) register accessor: ADC regular Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_dr`]
module"]
pub type ADC_DR = crate::Reg<adc_dr::ADC_DRrs>;
#[doc = "ADC regular Data Register"]
pub mod adc_dr;
#[doc = "ADC_JSQR (rw) register accessor: ADC injected sequence register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_jsqr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_jsqr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_jsqr`]
module"]
pub type ADC_JSQR = crate::Reg<adc_jsqr::ADC_JSQRrs>;
#[doc = "ADC injected sequence register"]
pub mod adc_jsqr;
#[doc = "ADC_OFR1 (rw) register accessor: ADC offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ofr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ofr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ofr1`]
module"]
pub type ADC_OFR1 = crate::Reg<adc_ofr1::ADC_OFR1rs>;
#[doc = "ADC offset register"]
pub mod adc_ofr1;
#[doc = "ADC_OFR2 (rw) register accessor: ADC offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ofr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ofr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ofr2`]
module"]
pub type ADC_OFR2 = crate::Reg<adc_ofr2::ADC_OFR2rs>;
#[doc = "ADC offset register"]
pub mod adc_ofr2;
#[doc = "ADC_OFR3 (rw) register accessor: ADC offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ofr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ofr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ofr3`]
module"]
pub type ADC_OFR3 = crate::Reg<adc_ofr3::ADC_OFR3rs>;
#[doc = "ADC offset register"]
pub mod adc_ofr3;
#[doc = "ADC_OFR4 (rw) register accessor: ADC offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ofr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ofr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ofr4`]
module"]
pub type ADC_OFR4 = crate::Reg<adc_ofr4::ADC_OFR4rs>;
#[doc = "ADC offset register"]
pub mod adc_ofr4;
#[doc = "ADC_JDR1 (r) register accessor: ADC injected data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_jdr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_jdr1`]
module"]
pub type ADC_JDR1 = crate::Reg<adc_jdr1::ADC_JDR1rs>;
#[doc = "ADC injected data register"]
pub mod adc_jdr1;
#[doc = "ADC_JDR2 (r) register accessor: ADC injected data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_jdr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_jdr2`]
module"]
pub type ADC_JDR2 = crate::Reg<adc_jdr2::ADC_JDR2rs>;
#[doc = "ADC injected data register"]
pub mod adc_jdr2;
#[doc = "ADC_JDR3 (r) register accessor: ADC injected data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_jdr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_jdr3`]
module"]
pub type ADC_JDR3 = crate::Reg<adc_jdr3::ADC_JDR3rs>;
#[doc = "ADC injected data register"]
pub mod adc_jdr3;
#[doc = "ADC_JDR4 (r) register accessor: ADC injected data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_jdr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_jdr4`]
module"]
pub type ADC_JDR4 = crate::Reg<adc_jdr4::ADC_JDR4rs>;
#[doc = "ADC injected data register"]
pub mod adc_jdr4;
#[doc = "ADC_AWD2CR (rw) register accessor: ADC analog watchdog 2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_awd2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_awd2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_awd2cr`]
module"]
pub type ADC_AWD2CR = crate::Reg<adc_awd2cr::ADC_AWD2CRrs>;
#[doc = "ADC analog watchdog 2 configuration register"]
pub mod adc_awd2cr;
#[doc = "ADC_AWD3CR (rw) register accessor: ADC analog watchdog 3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_awd3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_awd3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_awd3cr`]
module"]
pub type ADC_AWD3CR = crate::Reg<adc_awd3cr::ADC_AWD3CRrs>;
#[doc = "ADC analog watchdog 3 configuration register"]
pub mod adc_awd3cr;
#[doc = "ADC_LTR2 (rw) register accessor: ADC watchdog lower threshold register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ltr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ltr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ltr2`]
module"]
pub type ADC_LTR2 = crate::Reg<adc_ltr2::ADC_LTR2rs>;
#[doc = "ADC watchdog lower threshold register 2"]
pub mod adc_ltr2;
#[doc = "ADC_HTR2 (rw) register accessor: ADC watchdog higher threshold register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_htr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_htr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_htr2`]
module"]
pub type ADC_HTR2 = crate::Reg<adc_htr2::ADC_HTR2rs>;
#[doc = "ADC watchdog higher threshold register 2"]
pub mod adc_htr2;
#[doc = "ADC_LTR3 (rw) register accessor: ADC watchdog lower threshold register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ltr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ltr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ltr3`]
module"]
pub type ADC_LTR3 = crate::Reg<adc_ltr3::ADC_LTR3rs>;
#[doc = "ADC watchdog lower threshold register 3"]
pub mod adc_ltr3;
#[doc = "ADC_HTR3 (rw) register accessor: ADC watchdog higher threshold register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_htr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_htr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_htr3`]
module"]
pub type ADC_HTR3 = crate::Reg<adc_htr3::ADC_HTR3rs>;
#[doc = "ADC watchdog higher threshold register 3"]
pub mod adc_htr3;
#[doc = "ADC_DIFSEL (rw) register accessor: ADC differential mode selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_difsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_difsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_difsel`]
module"]
pub type ADC_DIFSEL = crate::Reg<adc_difsel::ADC_DIFSELrs>;
#[doc = "ADC differential mode selection register"]
pub mod adc_difsel;
#[doc = "ADC_CALFACT (rw) register accessor: ADC calibration factors register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_calfact::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_calfact::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_calfact`]
module"]
pub type ADC_CALFACT = crate::Reg<adc_calfact::ADC_CALFACTrs>;
#[doc = "ADC calibration factors register"]
pub mod adc_calfact;
#[doc = "ADC_CALFACT2 (rw) register accessor: ADC calibration factor register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_calfact2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_calfact2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_calfact2`]
module"]
pub type ADC_CALFACT2 = crate::Reg<adc_calfact2::ADC_CALFACT2rs>;
#[doc = "ADC calibration factor register 2"]
pub mod adc_calfact2;
