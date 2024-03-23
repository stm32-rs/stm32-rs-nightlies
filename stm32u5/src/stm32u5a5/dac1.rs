#[repr(C)]
#[doc = "Register block"]
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
    #[doc = "0x00 - DAC control register"]
    #[inline(always)]
    pub const fn dac_cr(&self) -> &DAC_CR {
        &self.dac_cr
    }
    #[doc = "0x04 - DAC software trigger register"]
    #[inline(always)]
    pub const fn dac_swtrgr(&self) -> &DAC_SWTRGR {
        &self.dac_swtrgr
    }
    #[doc = "0x08 - DAC channel1 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr12r1(&self) -> &DAC_DHR12R1 {
        &self.dac_dhr12r1
    }
    #[doc = "0x0c - DAC channel1 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr12l1(&self) -> &DAC_DHR12L1 {
        &self.dac_dhr12l1
    }
    #[doc = "0x10 - DAC channel1 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr8r1(&self) -> &DAC_DHR8R1 {
        &self.dac_dhr8r1
    }
    #[doc = "0x14 - DAC channel2 12-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr12r2(&self) -> &DAC_DHR12R2 {
        &self.dac_dhr12r2
    }
    #[doc = "0x18 - DAC channel2 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr12l2(&self) -> &DAC_DHR12L2 {
        &self.dac_dhr12l2
    }
    #[doc = "0x1c - DAC channel2 8-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr8r2(&self) -> &DAC_DHR8R2 {
        &self.dac_dhr8r2
    }
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr12rd(&self) -> &DAC_DHR12RD {
        &self.dac_dhr12rd
    }
    #[doc = "0x24 - DUAL DAC 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr12ld(&self) -> &DAC_DHR12LD {
        &self.dac_dhr12ld
    }
    #[doc = "0x28 - DUAL DAC 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr8rd(&self) -> &DAC_DHR8RD {
        &self.dac_dhr8rd
    }
    #[doc = "0x2c - DAC channel1 data output register"]
    #[inline(always)]
    pub const fn dac_dor1(&self) -> &DAC_DOR1 {
        &self.dac_dor1
    }
    #[doc = "0x30 - DAC channel2 data output register"]
    #[inline(always)]
    pub const fn dac_dor2(&self) -> &DAC_DOR2 {
        &self.dac_dor2
    }
    #[doc = "0x34 - DAC status register"]
    #[inline(always)]
    pub const fn dac_sr(&self) -> &DAC_SR {
        &self.dac_sr
    }
    #[doc = "0x38 - DAC calibration control register"]
    #[inline(always)]
    pub const fn dac_ccr(&self) -> &DAC_CCR {
        &self.dac_ccr
    }
    #[doc = "0x3c - DAC mode control register"]
    #[inline(always)]
    pub const fn dac_mcr(&self) -> &DAC_MCR {
        &self.dac_mcr
    }
    #[doc = "0x40 - DAC Sample and Hold sample time register 1"]
    #[inline(always)]
    pub const fn dac_shsr1(&self) -> &DAC_SHSR1 {
        &self.dac_shsr1
    }
    #[doc = "0x44 - DAC channel2 sample and hold sample time register"]
    #[inline(always)]
    pub const fn dac_shsr2(&self) -> &DAC_SHSR2 {
        &self.dac_shsr2
    }
    #[doc = "0x48 - DAC Sample and Hold hold time register"]
    #[inline(always)]
    pub const fn dac_shhr(&self) -> &DAC_SHHR {
        &self.dac_shhr
    }
    #[doc = "0x4c - DAC Sample and Hold refresh time register"]
    #[inline(always)]
    pub const fn dac_shrr(&self) -> &DAC_SHRR {
        &self.dac_shrr
    }
    #[doc = "0x54 - Autonomous mode control register"]
    #[inline(always)]
    pub const fn dac_autocr(&self) -> &DAC_AUTOCR {
        &self.dac_autocr
    }
}
#[doc = "DAC_CR (rw) register accessor: DAC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_cr`]
module"]
pub type DAC_CR = crate::Reg<dac_cr::DAC_CRrs>;
#[doc = "DAC control register"]
pub mod dac_cr;
#[doc = "DAC_SWTRGR (w) register accessor: DAC software trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_swtrgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_swtrgr`]
module"]
pub type DAC_SWTRGR = crate::Reg<dac_swtrgr::DAC_SWTRGRrs>;
#[doc = "DAC software trigger register"]
pub mod dac_swtrgr;
#[doc = "DAC_DHR12R1 (rw) register accessor: DAC channel1 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr12r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr12r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr12r1`]
module"]
pub type DAC_DHR12R1 = crate::Reg<dac_dhr12r1::DAC_DHR12R1rs>;
#[doc = "DAC channel1 12-bit right-aligned data holding register"]
pub mod dac_dhr12r1;
#[doc = "DAC_DHR12L1 (rw) register accessor: DAC channel1 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr12l1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr12l1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr12l1`]
module"]
pub type DAC_DHR12L1 = crate::Reg<dac_dhr12l1::DAC_DHR12L1rs>;
#[doc = "DAC channel1 12-bit left aligned data holding register"]
pub mod dac_dhr12l1;
#[doc = "DAC_DHR8R1 (rw) register accessor: DAC channel1 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr8r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr8r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr8r1`]
module"]
pub type DAC_DHR8R1 = crate::Reg<dac_dhr8r1::DAC_DHR8R1rs>;
#[doc = "DAC channel1 8-bit right aligned data holding register"]
pub mod dac_dhr8r1;
#[doc = "DAC_DHR12R2 (rw) register accessor: DAC channel2 12-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr12r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr12r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr12r2`]
module"]
pub type DAC_DHR12R2 = crate::Reg<dac_dhr12r2::DAC_DHR12R2rs>;
#[doc = "DAC channel2 12-bit right aligned data holding register"]
pub mod dac_dhr12r2;
#[doc = "DAC_DHR12L2 (rw) register accessor: DAC channel2 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr12l2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr12l2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr12l2`]
module"]
pub type DAC_DHR12L2 = crate::Reg<dac_dhr12l2::DAC_DHR12L2rs>;
#[doc = "DAC channel2 12-bit left aligned data holding register"]
pub mod dac_dhr12l2;
#[doc = "DAC_DHR8R2 (rw) register accessor: DAC channel2 8-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr8r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr8r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr8r2`]
module"]
pub type DAC_DHR8R2 = crate::Reg<dac_dhr8r2::DAC_DHR8R2rs>;
#[doc = "DAC channel2 8-bit right-aligned data holding register"]
pub mod dac_dhr8r2;
#[doc = "DAC_DHR12RD (rw) register accessor: Dual DAC 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr12rd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr12rd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr12rd`]
module"]
pub type DAC_DHR12RD = crate::Reg<dac_dhr12rd::DAC_DHR12RDrs>;
#[doc = "Dual DAC 12-bit right-aligned data holding register"]
pub mod dac_dhr12rd;
#[doc = "DAC_DHR12LD (rw) register accessor: DUAL DAC 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr12ld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr12ld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr12ld`]
module"]
pub type DAC_DHR12LD = crate::Reg<dac_dhr12ld::DAC_DHR12LDrs>;
#[doc = "DUAL DAC 12-bit left aligned data holding register"]
pub mod dac_dhr12ld;
#[doc = "DAC_DHR8RD (rw) register accessor: DUAL DAC 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr8rd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr8rd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr8rd`]
module"]
pub type DAC_DHR8RD = crate::Reg<dac_dhr8rd::DAC_DHR8RDrs>;
#[doc = "DUAL DAC 8-bit right aligned data holding register"]
pub mod dac_dhr8rd;
#[doc = "DAC_DOR1 (r) register accessor: DAC channel1 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dor1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dor1`]
module"]
pub type DAC_DOR1 = crate::Reg<dac_dor1::DAC_DOR1rs>;
#[doc = "DAC channel1 data output register"]
pub mod dac_dor1;
#[doc = "DAC_DOR2 (r) register accessor: DAC channel2 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dor2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dor2`]
module"]
pub type DAC_DOR2 = crate::Reg<dac_dor2::DAC_DOR2rs>;
#[doc = "DAC channel2 data output register"]
pub mod dac_dor2;
#[doc = "DAC_SR (rw) register accessor: DAC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_sr`]
module"]
pub type DAC_SR = crate::Reg<dac_sr::DAC_SRrs>;
#[doc = "DAC status register"]
pub mod dac_sr;
#[doc = "DAC_CCR (rw) register accessor: DAC calibration control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_ccr`]
module"]
pub type DAC_CCR = crate::Reg<dac_ccr::DAC_CCRrs>;
#[doc = "DAC calibration control register"]
pub mod dac_ccr;
#[doc = "DAC_MCR (rw) register accessor: DAC mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_mcr`]
module"]
pub type DAC_MCR = crate::Reg<dac_mcr::DAC_MCRrs>;
#[doc = "DAC mode control register"]
pub mod dac_mcr;
#[doc = "DAC_SHSR1 (rw) register accessor: DAC Sample and Hold sample time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_shsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_shsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_shsr1`]
module"]
pub type DAC_SHSR1 = crate::Reg<dac_shsr1::DAC_SHSR1rs>;
#[doc = "DAC Sample and Hold sample time register 1"]
pub mod dac_shsr1;
#[doc = "DAC_SHSR2 (rw) register accessor: DAC channel2 sample and hold sample time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_shsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_shsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_shsr2`]
module"]
pub type DAC_SHSR2 = crate::Reg<dac_shsr2::DAC_SHSR2rs>;
#[doc = "DAC channel2 sample and hold sample time register"]
pub mod dac_shsr2;
#[doc = "DAC_SHHR (rw) register accessor: DAC Sample and Hold hold time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_shhr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_shhr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_shhr`]
module"]
pub type DAC_SHHR = crate::Reg<dac_shhr::DAC_SHHRrs>;
#[doc = "DAC Sample and Hold hold time register"]
pub mod dac_shhr;
#[doc = "DAC_SHRR (rw) register accessor: DAC Sample and Hold refresh time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_shrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_shrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_shrr`]
module"]
pub type DAC_SHRR = crate::Reg<dac_shrr::DAC_SHRRrs>;
#[doc = "DAC Sample and Hold refresh time register"]
pub mod dac_shrr;
#[doc = "DAC_AUTOCR (rw) register accessor: Autonomous mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_autocr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_autocr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_autocr`]
module"]
pub type DAC_AUTOCR = crate::Reg<dac_autocr::DAC_AUTOCRrs>;
#[doc = "Autonomous mode control register"]
pub mod dac_autocr;
