#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dac_cr: DAC_CR,
    dac_swtrgr: DAC_SWTRGR,
    dac_dhr12r1: DAC_DHR12R1,
    dac_dhr12l1: DAC_DHR12L1,
    dac_dhr8r1: DAC_DHR8R1,
    _reserved5: [u8; 0x18],
    dac_dor1: DAC_DOR1,
    _reserved6: [u8; 0x04],
    dac_sr: DAC_SR,
    dac_ccr: DAC_CCR,
    dac_mcr: DAC_MCR,
    dac_shsr1: DAC_SHSR1,
    _reserved10: [u8; 0x04],
    dac_shhr: DAC_SHHR,
    dac_shrr: DAC_SHRR,
}
impl RegisterBlock {
    ///0x00 - DAC control register
    #[inline(always)]
    pub const fn dac_cr(&self) -> &DAC_CR {
        &self.dac_cr
    }
    ///0x04 - DAC software trigger register
    #[inline(always)]
    pub const fn dac_swtrgr(&self) -> &DAC_SWTRGR {
        &self.dac_swtrgr
    }
    ///0x08 - DAC channel1 12-bit right-aligned data holding register
    #[inline(always)]
    pub const fn dac_dhr12r1(&self) -> &DAC_DHR12R1 {
        &self.dac_dhr12r1
    }
    ///0x0c - DAC channel1 12-bit left aligned data holding register
    #[inline(always)]
    pub const fn dac_dhr12l1(&self) -> &DAC_DHR12L1 {
        &self.dac_dhr12l1
    }
    ///0x10 - DAC channel1 8-bit right aligned data holding register
    #[inline(always)]
    pub const fn dac_dhr8r1(&self) -> &DAC_DHR8R1 {
        &self.dac_dhr8r1
    }
    ///0x2c - DAC channel1 data output register
    #[inline(always)]
    pub const fn dac_dor1(&self) -> &DAC_DOR1 {
        &self.dac_dor1
    }
    ///0x34 - DAC status register
    #[inline(always)]
    pub const fn dac_sr(&self) -> &DAC_SR {
        &self.dac_sr
    }
    ///0x38 - DAC calibration control register
    #[inline(always)]
    pub const fn dac_ccr(&self) -> &DAC_CCR {
        &self.dac_ccr
    }
    ///0x3c - DAC mode control register
    #[inline(always)]
    pub const fn dac_mcr(&self) -> &DAC_MCR {
        &self.dac_mcr
    }
    ///0x40 - DAC channel1 sample and hold sample time register
    #[inline(always)]
    pub const fn dac_shsr1(&self) -> &DAC_SHSR1 {
        &self.dac_shsr1
    }
    ///0x48 - DAC sample and hold time register
    #[inline(always)]
    pub const fn dac_shhr(&self) -> &DAC_SHHR {
        &self.dac_shhr
    }
    ///0x4c - DAC sample and hold refresh time register
    #[inline(always)]
    pub const fn dac_shrr(&self) -> &DAC_SHRR {
        &self.dac_shrr
    }
}
/**DAC_CR (rw) register accessor: DAC control register

You can [`read`](crate::Reg::read) this register and get [`dac_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DAC:DAC_CR)

For information about available fields see [`mod@dac_cr`]
module*/
pub type DAC_CR = crate::Reg<dac_cr::DAC_CRrs>;
///DAC control register
pub mod dac_cr;
/**DAC_SWTRGR (w) register accessor: DAC software trigger register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_swtrgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DAC:DAC_SWTRGR)

For information about available fields see [`mod@dac_swtrgr`]
module*/
pub type DAC_SWTRGR = crate::Reg<dac_swtrgr::DAC_SWTRGRrs>;
///DAC software trigger register
pub mod dac_swtrgr;
/**DAC_DHR12R1 (rw) register accessor: DAC channel1 12-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dac_dhr12r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr12r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DAC:DAC_DHR12R1)

For information about available fields see [`mod@dac_dhr12r1`]
module*/
pub type DAC_DHR12R1 = crate::Reg<dac_dhr12r1::DAC_DHR12R1rs>;
///DAC channel1 12-bit right-aligned data holding register
pub mod dac_dhr12r1;
/**DAC_DHR12L1 (rw) register accessor: DAC channel1 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dac_dhr12l1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr12l1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DAC:DAC_DHR12L1)

For information about available fields see [`mod@dac_dhr12l1`]
module*/
pub type DAC_DHR12L1 = crate::Reg<dac_dhr12l1::DAC_DHR12L1rs>;
///DAC channel1 12-bit left aligned data holding register
pub mod dac_dhr12l1;
/**DAC_DHR8R1 (rw) register accessor: DAC channel1 8-bit right aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dac_dhr8r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr8r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DAC:DAC_DHR8R1)

For information about available fields see [`mod@dac_dhr8r1`]
module*/
pub type DAC_DHR8R1 = crate::Reg<dac_dhr8r1::DAC_DHR8R1rs>;
///DAC channel1 8-bit right aligned data holding register
pub mod dac_dhr8r1;
/**DAC_DOR1 (r) register accessor: DAC channel1 data output register

You can [`read`](crate::Reg::read) this register and get [`dac_dor1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DAC:DAC_DOR1)

For information about available fields see [`mod@dac_dor1`]
module*/
pub type DAC_DOR1 = crate::Reg<dac_dor1::DAC_DOR1rs>;
///DAC channel1 data output register
pub mod dac_dor1;
/**DAC_SR (rw) register accessor: DAC status register

You can [`read`](crate::Reg::read) this register and get [`dac_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DAC:DAC_SR)

For information about available fields see [`mod@dac_sr`]
module*/
pub type DAC_SR = crate::Reg<dac_sr::DAC_SRrs>;
///DAC status register
pub mod dac_sr;
/**DAC_CCR (rw) register accessor: DAC calibration control register

You can [`read`](crate::Reg::read) this register and get [`dac_ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DAC:DAC_CCR)

For information about available fields see [`mod@dac_ccr`]
module*/
pub type DAC_CCR = crate::Reg<dac_ccr::DAC_CCRrs>;
///DAC calibration control register
pub mod dac_ccr;
/**DAC_MCR (rw) register accessor: DAC mode control register

You can [`read`](crate::Reg::read) this register and get [`dac_mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DAC:DAC_MCR)

For information about available fields see [`mod@dac_mcr`]
module*/
pub type DAC_MCR = crate::Reg<dac_mcr::DAC_MCRrs>;
///DAC mode control register
pub mod dac_mcr;
/**DAC_SHSR1 (rw) register accessor: DAC channel1 sample and hold sample time register

You can [`read`](crate::Reg::read) this register and get [`dac_shsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_shsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DAC:DAC_SHSR1)

For information about available fields see [`mod@dac_shsr1`]
module*/
pub type DAC_SHSR1 = crate::Reg<dac_shsr1::DAC_SHSR1rs>;
///DAC channel1 sample and hold sample time register
pub mod dac_shsr1;
/**DAC_SHHR (rw) register accessor: DAC sample and hold time register

You can [`read`](crate::Reg::read) this register and get [`dac_shhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_shhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DAC:DAC_SHHR)

For information about available fields see [`mod@dac_shhr`]
module*/
pub type DAC_SHHR = crate::Reg<dac_shhr::DAC_SHHRrs>;
///DAC sample and hold time register
pub mod dac_shhr;
/**DAC_SHRR (rw) register accessor: DAC sample and hold refresh time register

You can [`read`](crate::Reg::read) this register and get [`dac_shrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_shrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DAC:DAC_SHRR)

For information about available fields see [`mod@dac_shrr`]
module*/
pub type DAC_SHRR = crate::Reg<dac_shrr::DAC_SHRRrs>;
///DAC sample and hold refresh time register
pub mod dac_shrr;
