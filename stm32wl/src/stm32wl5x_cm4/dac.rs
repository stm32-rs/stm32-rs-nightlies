#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    swtrgr: SWTRGR,
    dhr12r1: DHR12R1,
    dhr12l1: DHR12L1,
    dhr8r1: DHR8R1,
    _reserved5: [u8; 0x0c],
    dhr12rd: DHR12RD,
    dhr12ld: DHR12LD,
    dhr8rd: DHR8RD,
    dor1: DOR1,
    _reserved9: [u8; 0x04],
    sr: SR,
    ccr: CCR,
    mcr: MCR,
    shsr1: SHSR1,
    _reserved13: [u8; 0x04],
    shhr: SHHR,
    shrr: SHRR,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - software trigger register"]
    #[inline(always)]
    pub const fn swtrgr(&self) -> &SWTRGR {
        &self.swtrgr
    }
    #[doc = "0x08 - channel1 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12r1(&self) -> &DHR12R1 {
        &self.dhr12r1
    }
    #[doc = "0x0c - channel1 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12l1(&self) -> &DHR12L1 {
        &self.dhr12l1
    }
    #[doc = "0x10 - channel1 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dhr8r1(&self) -> &DHR8R1 {
        &self.dhr8r1
    }
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12rd(&self) -> &DHR12RD {
        &self.dhr12rd
    }
    #[doc = "0x24 - Dual DAC 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12ld(&self) -> &DHR12LD {
        &self.dhr12ld
    }
    #[doc = "0x28 - Dual DAC 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dhr8rd(&self) -> &DHR8RD {
        &self.dhr8rd
    }
    #[doc = "0x2c - DAC channel1 data output register"]
    #[inline(always)]
    pub const fn dor1(&self) -> &DOR1 {
        &self.dor1
    }
    #[doc = "0x34 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x38 - calibration control register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0x3c - mode control register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    #[doc = "0x40 - Sample and Hold sample time register 1"]
    #[inline(always)]
    pub const fn shsr1(&self) -> &SHSR1 {
        &self.shsr1
    }
    #[doc = "0x48 - Sample and Hold hold time register"]
    #[inline(always)]
    pub const fn shhr(&self) -> &SHHR {
        &self.shhr
    }
    #[doc = "0x4c - Sample and Hold refresh time register"]
    #[inline(always)]
    pub const fn shrr(&self) -> &SHRR {
        &self.shrr
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "control register"]
pub mod cr;
#[doc = "SWTRGR (w) register accessor: software trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrgr`]
module"]
pub type SWTRGR = crate::Reg<swtrgr::SWTRGRrs>;
#[doc = "software trigger register"]
pub mod swtrgr;
#[doc = "DHR12R1 (rw) register accessor: channel1 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr12r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr12r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12r1`]
module"]
pub type DHR12R1 = crate::Reg<dhr12r1::DHR12R1rs>;
#[doc = "channel1 12-bit right-aligned data holding register"]
pub mod dhr12r1;
#[doc = "DHR12L1 (rw) register accessor: channel1 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr12l1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr12l1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12l1`]
module"]
pub type DHR12L1 = crate::Reg<dhr12l1::DHR12L1rs>;
#[doc = "channel1 12-bit left aligned data holding register"]
pub mod dhr12l1;
#[doc = "DHR8R1 (rw) register accessor: channel1 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr8r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr8r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8r1`]
module"]
pub type DHR8R1 = crate::Reg<dhr8r1::DHR8R1rs>;
#[doc = "channel1 8-bit right aligned data holding register"]
pub mod dhr8r1;
#[doc = "DHR12RD (rw) register accessor: Dual DAC 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr12rd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr12rd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12rd`]
module"]
pub type DHR12RD = crate::Reg<dhr12rd::DHR12RDrs>;
#[doc = "Dual DAC 12-bit right-aligned data holding register"]
pub mod dhr12rd;
#[doc = "DHR12LD (rw) register accessor: Dual DAC 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr12ld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr12ld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12ld`]
module"]
pub type DHR12LD = crate::Reg<dhr12ld::DHR12LDrs>;
#[doc = "Dual DAC 12-bit left aligned data holding register"]
pub mod dhr12ld;
#[doc = "DHR8RD (rw) register accessor: Dual DAC 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr8rd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr8rd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8rd`]
module"]
pub type DHR8RD = crate::Reg<dhr8rd::DHR8RDrs>;
#[doc = "Dual DAC 8-bit right aligned data holding register"]
pub mod dhr8rd;
#[doc = "DOR1 (r) register accessor: DAC channel1 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dor1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor1`]
module"]
pub type DOR1 = crate::Reg<dor1::DOR1rs>;
#[doc = "DAC channel1 data output register"]
pub mod dor1;
#[doc = "SR (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "status register"]
pub mod sr;
#[doc = "CCR (rw) register accessor: calibration control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "calibration control register"]
pub mod ccr;
#[doc = "MCR (rw) register accessor: mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
pub type MCR = crate::Reg<mcr::MCRrs>;
#[doc = "mode control register"]
pub mod mcr;
#[doc = "SHSR1 (rw) register accessor: Sample and Hold sample time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shsr1`]
module"]
pub type SHSR1 = crate::Reg<shsr1::SHSR1rs>;
#[doc = "Sample and Hold sample time register 1"]
pub mod shsr1;
#[doc = "SHHR (rw) register accessor: Sample and Hold hold time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shhr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shhr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shhr`]
module"]
pub type SHHR = crate::Reg<shhr::SHHRrs>;
#[doc = "Sample and Hold hold time register"]
pub mod shhr;
#[doc = "SHRR (rw) register accessor: Sample and Hold refresh time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrr`]
module"]
pub type SHRR = crate::Reg<shrr::SHRRrs>;
#[doc = "Sample and Hold refresh time register"]
pub mod shrr;
