#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dac_cr: DAC_CR,
    dac_swtrgr: DAC_SWTRGR,
    dac_dhr12r1: DAC_DHR12R1,
    dac_dhr12l1: DAC_DHR12L1,
    dac_dhr8r1: DAC_DHR8R1,
    dac_dhr12r2: DAC_DHR12R2,
    dac_dhr12l2: DAC_DHR12L2,
    dac_dhr8r2: DAC_DHR8R2,
    dac_dhr12rd: DAC_DHR12RD,
    dac_dhr12ld: DAC_DHR12LD,
    dac_dhr8rd: DAC_DHR8RD,
    dac_dor1: DAC_DOR1,
    dac_dor2: DAC_DOR2,
    dac_sr: DAC_SR,
    dac_ccr: DAC_CCR,
    dac_mcr: DAC_MCR,
    dac_shsr1: DAC_SHSR1,
    dac_shsr2: DAC_SHSR2,
    dac_shhr: DAC_SHHR,
    dac_shrr: DAC_SHRR,
    _reserved20: [u8; 0x04],
    dac_autocr: DAC_AUTOCR,
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
    ///0x14 - DAC channel2 12-bit right aligned data holding register
    #[inline(always)]
    pub const fn dac_dhr12r2(&self) -> &DAC_DHR12R2 {
        &self.dac_dhr12r2
    }
    ///0x18 - DAC channel2 12-bit left aligned data holding register
    #[inline(always)]
    pub const fn dac_dhr12l2(&self) -> &DAC_DHR12L2 {
        &self.dac_dhr12l2
    }
    ///0x1c - DAC channel2 8-bit right-aligned data holding register
    #[inline(always)]
    pub const fn dac_dhr8r2(&self) -> &DAC_DHR8R2 {
        &self.dac_dhr8r2
    }
    ///0x20 - Dual DAC 12-bit right-aligned data holding register
    #[inline(always)]
    pub const fn dac_dhr12rd(&self) -> &DAC_DHR12RD {
        &self.dac_dhr12rd
    }
    ///0x24 - DUAL DAC 12-bit left aligned data holding register
    #[inline(always)]
    pub const fn dac_dhr12ld(&self) -> &DAC_DHR12LD {
        &self.dac_dhr12ld
    }
    ///0x28 - DUAL DAC 8-bit right aligned data holding register
    #[inline(always)]
    pub const fn dac_dhr8rd(&self) -> &DAC_DHR8RD {
        &self.dac_dhr8rd
    }
    ///0x2c - DAC channel1 data output register
    #[inline(always)]
    pub const fn dac_dor1(&self) -> &DAC_DOR1 {
        &self.dac_dor1
    }
    ///0x30 - DAC channel2 data output register
    #[inline(always)]
    pub const fn dac_dor2(&self) -> &DAC_DOR2 {
        &self.dac_dor2
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
    ///0x40 - DAC Sample and Hold sample time register 1
    #[inline(always)]
    pub const fn dac_shsr1(&self) -> &DAC_SHSR1 {
        &self.dac_shsr1
    }
    ///0x44 - DAC channel2 sample and hold sample time register
    #[inline(always)]
    pub const fn dac_shsr2(&self) -> &DAC_SHSR2 {
        &self.dac_shsr2
    }
    ///0x48 - DAC Sample and Hold hold time register
    #[inline(always)]
    pub const fn dac_shhr(&self) -> &DAC_SHHR {
        &self.dac_shhr
    }
    ///0x4c - DAC Sample and Hold refresh time register
    #[inline(always)]
    pub const fn dac_shrr(&self) -> &DAC_SHRR {
        &self.dac_shrr
    }
    ///0x54 - Autonomous mode control register
    #[inline(always)]
    pub const fn dac_autocr(&self) -> &DAC_AUTOCR {
        &self.dac_autocr
    }
}
/**DAC_CR (rw) register accessor: DAC control register

You can [`read`](crate::Reg::read) this register and get [`dac_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_CR)

For information about available fields see [`mod@dac_cr`]
module*/
pub type DAC_CR = crate::Reg<dac_cr::DAC_CRrs>;
///DAC control register
pub mod dac_cr;
/**DAC_SWTRGR (w) register accessor: DAC software trigger register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_swtrgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_SWTRGR)

For information about available fields see [`mod@dac_swtrgr`]
module*/
pub type DAC_SWTRGR = crate::Reg<dac_swtrgr::DAC_SWTRGRrs>;
///DAC software trigger register
pub mod dac_swtrgr;
/**DAC_DHR12R1 (rw) register accessor: DAC channel1 12-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dac_dhr12r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr12r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_DHR12R1)

For information about available fields see [`mod@dac_dhr12r1`]
module*/
pub type DAC_DHR12R1 = crate::Reg<dac_dhr12r1::DAC_DHR12R1rs>;
///DAC channel1 12-bit right-aligned data holding register
pub mod dac_dhr12r1;
/**DAC_DHR12L1 (rw) register accessor: DAC channel1 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dac_dhr12l1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr12l1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_DHR12L1)

For information about available fields see [`mod@dac_dhr12l1`]
module*/
pub type DAC_DHR12L1 = crate::Reg<dac_dhr12l1::DAC_DHR12L1rs>;
///DAC channel1 12-bit left aligned data holding register
pub mod dac_dhr12l1;
/**DAC_DHR8R1 (rw) register accessor: DAC channel1 8-bit right aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dac_dhr8r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr8r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_DHR8R1)

For information about available fields see [`mod@dac_dhr8r1`]
module*/
pub type DAC_DHR8R1 = crate::Reg<dac_dhr8r1::DAC_DHR8R1rs>;
///DAC channel1 8-bit right aligned data holding register
pub mod dac_dhr8r1;
/**DAC_DHR12R2 (rw) register accessor: DAC channel2 12-bit right aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dac_dhr12r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr12r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_DHR12R2)

For information about available fields see [`mod@dac_dhr12r2`]
module*/
pub type DAC_DHR12R2 = crate::Reg<dac_dhr12r2::DAC_DHR12R2rs>;
///DAC channel2 12-bit right aligned data holding register
pub mod dac_dhr12r2;
/**DAC_DHR12L2 (rw) register accessor: DAC channel2 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dac_dhr12l2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr12l2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_DHR12L2)

For information about available fields see [`mod@dac_dhr12l2`]
module*/
pub type DAC_DHR12L2 = crate::Reg<dac_dhr12l2::DAC_DHR12L2rs>;
///DAC channel2 12-bit left aligned data holding register
pub mod dac_dhr12l2;
/**DAC_DHR8R2 (rw) register accessor: DAC channel2 8-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dac_dhr8r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr8r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_DHR8R2)

For information about available fields see [`mod@dac_dhr8r2`]
module*/
pub type DAC_DHR8R2 = crate::Reg<dac_dhr8r2::DAC_DHR8R2rs>;
///DAC channel2 8-bit right-aligned data holding register
pub mod dac_dhr8r2;
/**DAC_DHR12RD (rw) register accessor: Dual DAC 12-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dac_dhr12rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr12rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_DHR12RD)

For information about available fields see [`mod@dac_dhr12rd`]
module*/
pub type DAC_DHR12RD = crate::Reg<dac_dhr12rd::DAC_DHR12RDrs>;
///Dual DAC 12-bit right-aligned data holding register
pub mod dac_dhr12rd;
/**DAC_DHR12LD (rw) register accessor: DUAL DAC 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dac_dhr12ld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr12ld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_DHR12LD)

For information about available fields see [`mod@dac_dhr12ld`]
module*/
pub type DAC_DHR12LD = crate::Reg<dac_dhr12ld::DAC_DHR12LDrs>;
///DUAL DAC 12-bit left aligned data holding register
pub mod dac_dhr12ld;
/**DAC_DHR8RD (rw) register accessor: DUAL DAC 8-bit right aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dac_dhr8rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr8rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_DHR8RD)

For information about available fields see [`mod@dac_dhr8rd`]
module*/
pub type DAC_DHR8RD = crate::Reg<dac_dhr8rd::DAC_DHR8RDrs>;
///DUAL DAC 8-bit right aligned data holding register
pub mod dac_dhr8rd;
/**DAC_DOR1 (r) register accessor: DAC channel1 data output register

You can [`read`](crate::Reg::read) this register and get [`dac_dor1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_DOR1)

For information about available fields see [`mod@dac_dor1`]
module*/
pub type DAC_DOR1 = crate::Reg<dac_dor1::DAC_DOR1rs>;
///DAC channel1 data output register
pub mod dac_dor1;
/**DAC_DOR2 (r) register accessor: DAC channel2 data output register

You can [`read`](crate::Reg::read) this register and get [`dac_dor2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_DOR2)

For information about available fields see [`mod@dac_dor2`]
module*/
pub type DAC_DOR2 = crate::Reg<dac_dor2::DAC_DOR2rs>;
///DAC channel2 data output register
pub mod dac_dor2;
/**DAC_SR (rw) register accessor: DAC status register

You can [`read`](crate::Reg::read) this register and get [`dac_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_SR)

For information about available fields see [`mod@dac_sr`]
module*/
pub type DAC_SR = crate::Reg<dac_sr::DAC_SRrs>;
///DAC status register
pub mod dac_sr;
/**DAC_CCR (rw) register accessor: DAC calibration control register

You can [`read`](crate::Reg::read) this register and get [`dac_ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_CCR)

For information about available fields see [`mod@dac_ccr`]
module*/
pub type DAC_CCR = crate::Reg<dac_ccr::DAC_CCRrs>;
///DAC calibration control register
pub mod dac_ccr;
/**DAC_MCR (rw) register accessor: DAC mode control register

You can [`read`](crate::Reg::read) this register and get [`dac_mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_MCR)

For information about available fields see [`mod@dac_mcr`]
module*/
pub type DAC_MCR = crate::Reg<dac_mcr::DAC_MCRrs>;
///DAC mode control register
pub mod dac_mcr;
/**DAC_SHSR1 (rw) register accessor: DAC Sample and Hold sample time register 1

You can [`read`](crate::Reg::read) this register and get [`dac_shsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_shsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_SHSR1)

For information about available fields see [`mod@dac_shsr1`]
module*/
pub type DAC_SHSR1 = crate::Reg<dac_shsr1::DAC_SHSR1rs>;
///DAC Sample and Hold sample time register 1
pub mod dac_shsr1;
/**DAC_SHSR2 (rw) register accessor: DAC channel2 sample and hold sample time register

You can [`read`](crate::Reg::read) this register and get [`dac_shsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_shsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_SHSR2)

For information about available fields see [`mod@dac_shsr2`]
module*/
pub type DAC_SHSR2 = crate::Reg<dac_shsr2::DAC_SHSR2rs>;
///DAC channel2 sample and hold sample time register
pub mod dac_shsr2;
/**DAC_SHHR (rw) register accessor: DAC Sample and Hold hold time register

You can [`read`](crate::Reg::read) this register and get [`dac_shhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_shhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_SHHR)

For information about available fields see [`mod@dac_shhr`]
module*/
pub type DAC_SHHR = crate::Reg<dac_shhr::DAC_SHHRrs>;
///DAC Sample and Hold hold time register
pub mod dac_shhr;
/**DAC_SHRR (rw) register accessor: DAC Sample and Hold refresh time register

You can [`read`](crate::Reg::read) this register and get [`dac_shrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_shrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_SHRR)

For information about available fields see [`mod@dac_shrr`]
module*/
pub type DAC_SHRR = crate::Reg<dac_shrr::DAC_SHRRrs>;
///DAC Sample and Hold refresh time register
pub mod dac_shrr;
/**DAC_AUTOCR (rw) register accessor: Autonomous mode control register

You can [`read`](crate::Reg::read) this register and get [`dac_autocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_autocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DAC_AUTOCR)

For information about available fields see [`mod@dac_autocr`]
module*/
pub type DAC_AUTOCR = crate::Reg<dac_autocr::DAC_AUTOCRrs>;
///Autonomous mode control register
pub mod dac_autocr;
