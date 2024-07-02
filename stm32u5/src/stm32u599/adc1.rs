#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    adc_isr: ADC_ISR,
    adc_ier: ADC_IER,
    adc_cr: ADC_CR,
    adc_cfgr1: ADC_CFGR1,
    adc_cfgr2: ADC_CFGR2,
    adc_smpr1: ADC_SMPR1,
    adc_smpr2: ADC_SMPR2,
    adc_pcsel: ADC_PCSEL,
    _reserved8: [u8; 0x10],
    adc_sqr1: ADC_SQR1,
    adc_sqr2: ADC_SQR2,
    adc_sqr3: ADC_SQR3,
    adc_sqr4: ADC_SQR4,
    adc_dr: ADC_DR,
    _reserved13: [u8; 0x08],
    adc_jsqr: ADC_JSQR,
    _reserved14: [u8; 0x10],
    adc_ofr1: ADC_OFR1,
    adc_ofr2: ADC_OFR2,
    adc_ofr3: ADC_OFR3,
    adc_ofr4: ADC_OFR4,
    adc_gcomp: ADC_GCOMP,
    _reserved19: [u8; 0x0c],
    adc_jdr1: ADC_JDR1,
    adc_jdr2: ADC_JDR2,
    adc_jdr3: ADC_JDR3,
    adc_jdr4: ADC_JDR4,
    _reserved23: [u8; 0x10],
    adc_awd2cr: ADC_AWD2CR,
    adc_awd3cr: ADC_AWD3CR,
    adc_ltr1: ADC_LTR1,
    adc_htr1: ADC_HTR1,
    adc_ltr2: ADC_LTR2,
    adc_htr2: ADC_HTR2,
    adc_ltr3: ADC_LTR3,
    adc_htr3: ADC_HTR3,
    adc_difsel: ADC_DIFSEL,
    adc_calfact: ADC_CALFACT,
    adc_calfact2: ADC_CALFACT2,
}
impl RegisterBlock {
    ///0x00 - ADC interrupt and status register
    #[inline(always)]
    pub const fn adc_isr(&self) -> &ADC_ISR {
        &self.adc_isr
    }
    ///0x04 - ADC interrupt enable register
    #[inline(always)]
    pub const fn adc_ier(&self) -> &ADC_IER {
        &self.adc_ier
    }
    ///0x08 - ADC control register
    #[inline(always)]
    pub const fn adc_cr(&self) -> &ADC_CR {
        &self.adc_cr
    }
    ///0x0c - ADC configuration register
    #[inline(always)]
    pub const fn adc_cfgr1(&self) -> &ADC_CFGR1 {
        &self.adc_cfgr1
    }
    ///0x10 - ADC configuration register 2
    #[inline(always)]
    pub const fn adc_cfgr2(&self) -> &ADC_CFGR2 {
        &self.adc_cfgr2
    }
    ///0x14 - ADC sample time register 1
    #[inline(always)]
    pub const fn adc_smpr1(&self) -> &ADC_SMPR1 {
        &self.adc_smpr1
    }
    ///0x18 - ADC sample time register 2
    #[inline(always)]
    pub const fn adc_smpr2(&self) -> &ADC_SMPR2 {
        &self.adc_smpr2
    }
    ///0x1c - ADC channel preselection register
    #[inline(always)]
    pub const fn adc_pcsel(&self) -> &ADC_PCSEL {
        &self.adc_pcsel
    }
    ///0x30 - ADC regular sequence register 1
    #[inline(always)]
    pub const fn adc_sqr1(&self) -> &ADC_SQR1 {
        &self.adc_sqr1
    }
    ///0x34 - ADC regular sequence register 2
    #[inline(always)]
    pub const fn adc_sqr2(&self) -> &ADC_SQR2 {
        &self.adc_sqr2
    }
    ///0x38 - ADC regular sequence register 3
    #[inline(always)]
    pub const fn adc_sqr3(&self) -> &ADC_SQR3 {
        &self.adc_sqr3
    }
    ///0x3c - ADC regular sequence register 4
    #[inline(always)]
    pub const fn adc_sqr4(&self) -> &ADC_SQR4 {
        &self.adc_sqr4
    }
    ///0x40 - ADC regular Data Register
    #[inline(always)]
    pub const fn adc_dr(&self) -> &ADC_DR {
        &self.adc_dr
    }
    ///0x4c - ADC injected sequence register
    #[inline(always)]
    pub const fn adc_jsqr(&self) -> &ADC_JSQR {
        &self.adc_jsqr
    }
    ///0x60 - ADC offset register
    #[inline(always)]
    pub const fn adc_ofr1(&self) -> &ADC_OFR1 {
        &self.adc_ofr1
    }
    ///0x64 - ADC offset register
    #[inline(always)]
    pub const fn adc_ofr2(&self) -> &ADC_OFR2 {
        &self.adc_ofr2
    }
    ///0x68 - ADC offset register
    #[inline(always)]
    pub const fn adc_ofr3(&self) -> &ADC_OFR3 {
        &self.adc_ofr3
    }
    ///0x6c - ADC offset register
    #[inline(always)]
    pub const fn adc_ofr4(&self) -> &ADC_OFR4 {
        &self.adc_ofr4
    }
    ///0x70 - ADC gain compensation register
    #[inline(always)]
    pub const fn adc_gcomp(&self) -> &ADC_GCOMP {
        &self.adc_gcomp
    }
    ///0x80 - ADC injected data register
    #[inline(always)]
    pub const fn adc_jdr1(&self) -> &ADC_JDR1 {
        &self.adc_jdr1
    }
    ///0x84 - ADC injected data register
    #[inline(always)]
    pub const fn adc_jdr2(&self) -> &ADC_JDR2 {
        &self.adc_jdr2
    }
    ///0x88 - ADC injected data register
    #[inline(always)]
    pub const fn adc_jdr3(&self) -> &ADC_JDR3 {
        &self.adc_jdr3
    }
    ///0x8c - ADC injected data register
    #[inline(always)]
    pub const fn adc_jdr4(&self) -> &ADC_JDR4 {
        &self.adc_jdr4
    }
    ///0xa0 - ADC analog watchdog 2 configuration register
    #[inline(always)]
    pub const fn adc_awd2cr(&self) -> &ADC_AWD2CR {
        &self.adc_awd2cr
    }
    ///0xa4 - ADC analog watchdog 3 configuration register
    #[inline(always)]
    pub const fn adc_awd3cr(&self) -> &ADC_AWD3CR {
        &self.adc_awd3cr
    }
    ///0xa8 - ADC watchdog threshold register 1
    #[inline(always)]
    pub const fn adc_ltr1(&self) -> &ADC_LTR1 {
        &self.adc_ltr1
    }
    ///0xac - ADC watchdog threshold register 1
    #[inline(always)]
    pub const fn adc_htr1(&self) -> &ADC_HTR1 {
        &self.adc_htr1
    }
    ///0xb0 - ADC watchdog lower threshold register 2
    #[inline(always)]
    pub const fn adc_ltr2(&self) -> &ADC_LTR2 {
        &self.adc_ltr2
    }
    ///0xb4 - ADC watchdog higher threshold register 2
    #[inline(always)]
    pub const fn adc_htr2(&self) -> &ADC_HTR2 {
        &self.adc_htr2
    }
    ///0xb8 - ADC watchdog lower threshold register 3
    #[inline(always)]
    pub const fn adc_ltr3(&self) -> &ADC_LTR3 {
        &self.adc_ltr3
    }
    ///0xbc - ADC watchdog higher threshold register 3
    #[inline(always)]
    pub const fn adc_htr3(&self) -> &ADC_HTR3 {
        &self.adc_htr3
    }
    ///0xc0 - ADC differential mode selection register
    #[inline(always)]
    pub const fn adc_difsel(&self) -> &ADC_DIFSEL {
        &self.adc_difsel
    }
    ///0xc4 - ADC user control register
    #[inline(always)]
    pub const fn adc_calfact(&self) -> &ADC_CALFACT {
        &self.adc_calfact
    }
    ///0xc8 - ADC calibration factor register
    #[inline(always)]
    pub const fn adc_calfact2(&self) -> &ADC_CALFACT2 {
        &self.adc_calfact2
    }
}
/**ADC_ISR (rw) register accessor: ADC interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`adc_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_ISR)

For information about available fields see [`mod@adc_isr`]
module*/
pub type ADC_ISR = crate::Reg<adc_isr::ADC_ISRrs>;
///ADC interrupt and status register
pub mod adc_isr;
/**ADC_IER (rw) register accessor: ADC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`adc_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_IER)

For information about available fields see [`mod@adc_ier`]
module*/
pub type ADC_IER = crate::Reg<adc_ier::ADC_IERrs>;
///ADC interrupt enable register
pub mod adc_ier;
/**ADC_CR (rw) register accessor: ADC control register

You can [`read`](crate::Reg::read) this register and get [`adc_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_CR)

For information about available fields see [`mod@adc_cr`]
module*/
pub type ADC_CR = crate::Reg<adc_cr::ADC_CRrs>;
///ADC control register
pub mod adc_cr;
/**ADC_CFGR1 (rw) register accessor: ADC configuration register

You can [`read`](crate::Reg::read) this register and get [`adc_cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_CFGR1)

For information about available fields see [`mod@adc_cfgr1`]
module*/
pub type ADC_CFGR1 = crate::Reg<adc_cfgr1::ADC_CFGR1rs>;
///ADC configuration register
pub mod adc_cfgr1;
/**ADC_CFGR2 (rw) register accessor: ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`adc_cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_CFGR2)

For information about available fields see [`mod@adc_cfgr2`]
module*/
pub type ADC_CFGR2 = crate::Reg<adc_cfgr2::ADC_CFGR2rs>;
///ADC configuration register 2
pub mod adc_cfgr2;
/**ADC_SMPR1 (rw) register accessor: ADC sample time register 1

You can [`read`](crate::Reg::read) this register and get [`adc_smpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_smpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_SMPR1)

For information about available fields see [`mod@adc_smpr1`]
module*/
pub type ADC_SMPR1 = crate::Reg<adc_smpr1::ADC_SMPR1rs>;
///ADC sample time register 1
pub mod adc_smpr1;
/**ADC_SMPR2 (rw) register accessor: ADC sample time register 2

You can [`read`](crate::Reg::read) this register and get [`adc_smpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_smpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_SMPR2)

For information about available fields see [`mod@adc_smpr2`]
module*/
pub type ADC_SMPR2 = crate::Reg<adc_smpr2::ADC_SMPR2rs>;
///ADC sample time register 2
pub mod adc_smpr2;
/**ADC_PCSEL (rw) register accessor: ADC channel preselection register

You can [`read`](crate::Reg::read) this register and get [`adc_pcsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_pcsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_PCSEL)

For information about available fields see [`mod@adc_pcsel`]
module*/
pub type ADC_PCSEL = crate::Reg<adc_pcsel::ADC_PCSELrs>;
///ADC channel preselection register
pub mod adc_pcsel;
/**ADC_SQR1 (rw) register accessor: ADC regular sequence register 1

You can [`read`](crate::Reg::read) this register and get [`adc_sqr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_sqr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_SQR1)

For information about available fields see [`mod@adc_sqr1`]
module*/
pub type ADC_SQR1 = crate::Reg<adc_sqr1::ADC_SQR1rs>;
///ADC regular sequence register 1
pub mod adc_sqr1;
/**ADC_SQR2 (rw) register accessor: ADC regular sequence register 2

You can [`read`](crate::Reg::read) this register and get [`adc_sqr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_sqr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_SQR2)

For information about available fields see [`mod@adc_sqr2`]
module*/
pub type ADC_SQR2 = crate::Reg<adc_sqr2::ADC_SQR2rs>;
///ADC regular sequence register 2
pub mod adc_sqr2;
/**ADC_SQR3 (rw) register accessor: ADC regular sequence register 3

You can [`read`](crate::Reg::read) this register and get [`adc_sqr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_sqr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_SQR3)

For information about available fields see [`mod@adc_sqr3`]
module*/
pub type ADC_SQR3 = crate::Reg<adc_sqr3::ADC_SQR3rs>;
///ADC regular sequence register 3
pub mod adc_sqr3;
/**ADC_SQR4 (rw) register accessor: ADC regular sequence register 4

You can [`read`](crate::Reg::read) this register and get [`adc_sqr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_sqr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_SQR4)

For information about available fields see [`mod@adc_sqr4`]
module*/
pub type ADC_SQR4 = crate::Reg<adc_sqr4::ADC_SQR4rs>;
///ADC regular sequence register 4
pub mod adc_sqr4;
/**ADC_DR (r) register accessor: ADC regular Data Register

You can [`read`](crate::Reg::read) this register and get [`adc_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_DR)

For information about available fields see [`mod@adc_dr`]
module*/
pub type ADC_DR = crate::Reg<adc_dr::ADC_DRrs>;
///ADC regular Data Register
pub mod adc_dr;
/**ADC_JSQR (rw) register accessor: ADC injected sequence register

You can [`read`](crate::Reg::read) this register and get [`adc_jsqr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_jsqr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_JSQR)

For information about available fields see [`mod@adc_jsqr`]
module*/
pub type ADC_JSQR = crate::Reg<adc_jsqr::ADC_JSQRrs>;
///ADC injected sequence register
pub mod adc_jsqr;
/**ADC_OFR1 (rw) register accessor: ADC offset register

You can [`read`](crate::Reg::read) this register and get [`adc_ofr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ofr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_OFR1)

For information about available fields see [`mod@adc_ofr1`]
module*/
pub type ADC_OFR1 = crate::Reg<adc_ofr1::ADC_OFR1rs>;
///ADC offset register
pub mod adc_ofr1;
/**ADC_OFR2 (rw) register accessor: ADC offset register

You can [`read`](crate::Reg::read) this register and get [`adc_ofr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ofr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_OFR2)

For information about available fields see [`mod@adc_ofr2`]
module*/
pub type ADC_OFR2 = crate::Reg<adc_ofr2::ADC_OFR2rs>;
///ADC offset register
pub mod adc_ofr2;
/**ADC_OFR3 (rw) register accessor: ADC offset register

You can [`read`](crate::Reg::read) this register and get [`adc_ofr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ofr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_OFR3)

For information about available fields see [`mod@adc_ofr3`]
module*/
pub type ADC_OFR3 = crate::Reg<adc_ofr3::ADC_OFR3rs>;
///ADC offset register
pub mod adc_ofr3;
/**ADC_OFR4 (rw) register accessor: ADC offset register

You can [`read`](crate::Reg::read) this register and get [`adc_ofr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ofr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_OFR4)

For information about available fields see [`mod@adc_ofr4`]
module*/
pub type ADC_OFR4 = crate::Reg<adc_ofr4::ADC_OFR4rs>;
///ADC offset register
pub mod adc_ofr4;
/**ADC_GCOMP (rw) register accessor: ADC gain compensation register

You can [`read`](crate::Reg::read) this register and get [`adc_gcomp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_gcomp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_GCOMP)

For information about available fields see [`mod@adc_gcomp`]
module*/
pub type ADC_GCOMP = crate::Reg<adc_gcomp::ADC_GCOMPrs>;
///ADC gain compensation register
pub mod adc_gcomp;
/**ADC_JDR1 (r) register accessor: ADC injected data register

You can [`read`](crate::Reg::read) this register and get [`adc_jdr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_JDR1)

For information about available fields see [`mod@adc_jdr1`]
module*/
pub type ADC_JDR1 = crate::Reg<adc_jdr1::ADC_JDR1rs>;
///ADC injected data register
pub mod adc_jdr1;
/**ADC_JDR2 (r) register accessor: ADC injected data register

You can [`read`](crate::Reg::read) this register and get [`adc_jdr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_JDR2)

For information about available fields see [`mod@adc_jdr2`]
module*/
pub type ADC_JDR2 = crate::Reg<adc_jdr2::ADC_JDR2rs>;
///ADC injected data register
pub mod adc_jdr2;
/**ADC_JDR3 (r) register accessor: ADC injected data register

You can [`read`](crate::Reg::read) this register and get [`adc_jdr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_JDR3)

For information about available fields see [`mod@adc_jdr3`]
module*/
pub type ADC_JDR3 = crate::Reg<adc_jdr3::ADC_JDR3rs>;
///ADC injected data register
pub mod adc_jdr3;
/**ADC_JDR4 (r) register accessor: ADC injected data register

You can [`read`](crate::Reg::read) this register and get [`adc_jdr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_JDR4)

For information about available fields see [`mod@adc_jdr4`]
module*/
pub type ADC_JDR4 = crate::Reg<adc_jdr4::ADC_JDR4rs>;
///ADC injected data register
pub mod adc_jdr4;
/**ADC_AWD2CR (rw) register accessor: ADC analog watchdog 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`adc_awd2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_awd2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_AWD2CR)

For information about available fields see [`mod@adc_awd2cr`]
module*/
pub type ADC_AWD2CR = crate::Reg<adc_awd2cr::ADC_AWD2CRrs>;
///ADC analog watchdog 2 configuration register
pub mod adc_awd2cr;
/**ADC_AWD3CR (rw) register accessor: ADC analog watchdog 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`adc_awd3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_awd3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_AWD3CR)

For information about available fields see [`mod@adc_awd3cr`]
module*/
pub type ADC_AWD3CR = crate::Reg<adc_awd3cr::ADC_AWD3CRrs>;
///ADC analog watchdog 3 configuration register
pub mod adc_awd3cr;
/**ADC_LTR1 (rw) register accessor: ADC watchdog threshold register 1

You can [`read`](crate::Reg::read) this register and get [`adc_ltr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ltr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_LTR1)

For information about available fields see [`mod@adc_ltr1`]
module*/
pub type ADC_LTR1 = crate::Reg<adc_ltr1::ADC_LTR1rs>;
///ADC watchdog threshold register 1
pub mod adc_ltr1;
/**ADC_HTR1 (rw) register accessor: ADC watchdog threshold register 1

You can [`read`](crate::Reg::read) this register and get [`adc_htr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_htr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_HTR1)

For information about available fields see [`mod@adc_htr1`]
module*/
pub type ADC_HTR1 = crate::Reg<adc_htr1::ADC_HTR1rs>;
///ADC watchdog threshold register 1
pub mod adc_htr1;
/**ADC_LTR2 (rw) register accessor: ADC watchdog lower threshold register 2

You can [`read`](crate::Reg::read) this register and get [`adc_ltr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ltr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_LTR2)

For information about available fields see [`mod@adc_ltr2`]
module*/
pub type ADC_LTR2 = crate::Reg<adc_ltr2::ADC_LTR2rs>;
///ADC watchdog lower threshold register 2
pub mod adc_ltr2;
/**ADC_HTR2 (rw) register accessor: ADC watchdog higher threshold register 2

You can [`read`](crate::Reg::read) this register and get [`adc_htr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_htr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_HTR2)

For information about available fields see [`mod@adc_htr2`]
module*/
pub type ADC_HTR2 = crate::Reg<adc_htr2::ADC_HTR2rs>;
///ADC watchdog higher threshold register 2
pub mod adc_htr2;
/**ADC_LTR3 (rw) register accessor: ADC watchdog lower threshold register 3

You can [`read`](crate::Reg::read) this register and get [`adc_ltr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ltr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_LTR3)

For information about available fields see [`mod@adc_ltr3`]
module*/
pub type ADC_LTR3 = crate::Reg<adc_ltr3::ADC_LTR3rs>;
///ADC watchdog lower threshold register 3
pub mod adc_ltr3;
/**ADC_HTR3 (rw) register accessor: ADC watchdog higher threshold register 3

You can [`read`](crate::Reg::read) this register and get [`adc_htr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_htr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_HTR3)

For information about available fields see [`mod@adc_htr3`]
module*/
pub type ADC_HTR3 = crate::Reg<adc_htr3::ADC_HTR3rs>;
///ADC watchdog higher threshold register 3
pub mod adc_htr3;
/**ADC_DIFSEL (rw) register accessor: ADC differential mode selection register

You can [`read`](crate::Reg::read) this register and get [`adc_difsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_difsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_DIFSEL)

For information about available fields see [`mod@adc_difsel`]
module*/
pub type ADC_DIFSEL = crate::Reg<adc_difsel::ADC_DIFSELrs>;
///ADC differential mode selection register
pub mod adc_difsel;
/**ADC_CALFACT (rw) register accessor: ADC user control register

You can [`read`](crate::Reg::read) this register and get [`adc_calfact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_calfact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_CALFACT)

For information about available fields see [`mod@adc_calfact`]
module*/
pub type ADC_CALFACT = crate::Reg<adc_calfact::ADC_CALFACTrs>;
///ADC user control register
pub mod adc_calfact;
/**ADC_CALFACT2 (rw) register accessor: ADC calibration factor register

You can [`read`](crate::Reg::read) this register and get [`adc_calfact2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_calfact2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_CALFACT2)

For information about available fields see [`mod@adc_calfact2`]
module*/
pub type ADC_CALFACT2 = crate::Reg<adc_calfact2::ADC_CALFACT2rs>;
///ADC calibration factor register
pub mod adc_calfact2;
