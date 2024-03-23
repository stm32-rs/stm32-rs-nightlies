#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    swtrgr: SWTRGR,
    dhr12r1: DHR12R1,
    dhr12l1: DHR12L1,
    dhr8r1: DHR8R1,
    dhr12r2: DHR12R2,
    dhr12l2: DHR12L2,
    dhr8r2: DHR8R2,
    dhr12rd: DHR12RD,
    dhr12ld: DHR12LD,
    dhr8rd: DHR8RD,
    dor1: DOR1,
    dor2: DOR2,
    sr: SR,
    ccr: CCR,
    mcr: MCR,
    shsr1: SHSR1,
    shsr2: SHSR2,
    shhr: SHHR,
    shrr: SHRR,
    _reserved20: [u8; 0x03a0],
    ip_hwcfgr0: IP_HWCFGR0,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - DAC control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - DAC software trigger register"]
    #[inline(always)]
    pub const fn swtrgr(&self) -> &SWTRGR {
        &self.swtrgr
    }
    #[doc = "0x08 - DAC channel1 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12r1(&self) -> &DHR12R1 {
        &self.dhr12r1
    }
    #[doc = "0x0c - DAC channel1 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12l1(&self) -> &DHR12L1 {
        &self.dhr12l1
    }
    #[doc = "0x10 - DAC channel1 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dhr8r1(&self) -> &DHR8R1 {
        &self.dhr8r1
    }
    #[doc = "0x14 - DAC channel2 12-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12r2(&self) -> &DHR12R2 {
        &self.dhr12r2
    }
    #[doc = "0x18 - DAC channel2 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12l2(&self) -> &DHR12L2 {
        &self.dhr12l2
    }
    #[doc = "0x1c - DAC channel2 8-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dhr8r2(&self) -> &DHR8R2 {
        &self.dhr8r2
    }
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12rd(&self) -> &DHR12RD {
        &self.dhr12rd
    }
    #[doc = "0x24 - DUAL DAC 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12ld(&self) -> &DHR12LD {
        &self.dhr12ld
    }
    #[doc = "0x28 - DUAL DAC 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dhr8rd(&self) -> &DHR8RD {
        &self.dhr8rd
    }
    #[doc = "0x2c - DAC channel1 data output register"]
    #[inline(always)]
    pub const fn dor1(&self) -> &DOR1 {
        &self.dor1
    }
    #[doc = "0x30 - DAC channel2 data output register"]
    #[inline(always)]
    pub const fn dor2(&self) -> &DOR2 {
        &self.dor2
    }
    #[doc = "0x34 - DAC status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x38 - DAC calibration control register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0x3c - DAC mode control register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    #[doc = "0x40 - DAC Sample and Hold sample time register 1"]
    #[inline(always)]
    pub const fn shsr1(&self) -> &SHSR1 {
        &self.shsr1
    }
    #[doc = "0x44 - DAC Sample and Hold sample time register 2"]
    #[inline(always)]
    pub const fn shsr2(&self) -> &SHSR2 {
        &self.shsr2
    }
    #[doc = "0x48 - DAC Sample and Hold hold time register"]
    #[inline(always)]
    pub const fn shhr(&self) -> &SHHR {
        &self.shhr
    }
    #[doc = "0x4c - DAC Sample and Hold refresh time register"]
    #[inline(always)]
    pub const fn shrr(&self) -> &SHRR {
        &self.shrr
    }
    #[doc = "0x3f0 - DAC IP Hardware Configuration Register"]
    #[inline(always)]
    pub const fn ip_hwcfgr0(&self) -> &IP_HWCFGR0 {
        &self.ip_hwcfgr0
    }
    #[doc = "0x3f4 - EXTI IP Version register"]
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    #[doc = "0x3f8 - EXTI Identification register"]
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    #[doc = "0x3fc - EXTI Size ID register"]
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
#[doc = "CR (rw) register accessor: DAC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "DAC control register"]
pub mod cr;
#[doc = "SWTRGR (w) register accessor: DAC software trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrgr`]
module"]
pub type SWTRGR = crate::Reg<swtrgr::SWTRGRrs>;
#[doc = "DAC software trigger register"]
pub mod swtrgr;
#[doc = "DHR12R1 (rw) register accessor: DAC channel1 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr12r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr12r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12r1`]
module"]
pub type DHR12R1 = crate::Reg<dhr12r1::DHR12R1rs>;
#[doc = "DAC channel1 12-bit right-aligned data holding register"]
pub mod dhr12r1;
#[doc = "DHR12L1 (rw) register accessor: DAC channel1 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr12l1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr12l1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12l1`]
module"]
pub type DHR12L1 = crate::Reg<dhr12l1::DHR12L1rs>;
#[doc = "DAC channel1 12-bit left aligned data holding register"]
pub mod dhr12l1;
#[doc = "DHR8R1 (rw) register accessor: DAC channel1 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr8r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr8r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8r1`]
module"]
pub type DHR8R1 = crate::Reg<dhr8r1::DHR8R1rs>;
#[doc = "DAC channel1 8-bit right aligned data holding register"]
pub mod dhr8r1;
#[doc = "DHR12R2 (rw) register accessor: DAC channel2 12-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr12r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr12r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12r2`]
module"]
pub type DHR12R2 = crate::Reg<dhr12r2::DHR12R2rs>;
#[doc = "DAC channel2 12-bit right aligned data holding register"]
pub mod dhr12r2;
#[doc = "DHR12L2 (rw) register accessor: DAC channel2 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr12l2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr12l2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12l2`]
module"]
pub type DHR12L2 = crate::Reg<dhr12l2::DHR12L2rs>;
#[doc = "DAC channel2 12-bit left aligned data holding register"]
pub mod dhr12l2;
#[doc = "DHR8R2 (rw) register accessor: DAC channel2 8-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr8r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr8r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8r2`]
module"]
pub type DHR8R2 = crate::Reg<dhr8r2::DHR8R2rs>;
#[doc = "DAC channel2 8-bit right-aligned data holding register"]
pub mod dhr8r2;
#[doc = "DHR12RD (rw) register accessor: Dual DAC 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr12rd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr12rd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12rd`]
module"]
pub type DHR12RD = crate::Reg<dhr12rd::DHR12RDrs>;
#[doc = "Dual DAC 12-bit right-aligned data holding register"]
pub mod dhr12rd;
#[doc = "DHR12LD (rw) register accessor: DUAL DAC 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr12ld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr12ld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12ld`]
module"]
pub type DHR12LD = crate::Reg<dhr12ld::DHR12LDrs>;
#[doc = "DUAL DAC 12-bit left aligned data holding register"]
pub mod dhr12ld;
#[doc = "DHR8RD (rw) register accessor: DUAL DAC 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr8rd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr8rd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8rd`]
module"]
pub type DHR8RD = crate::Reg<dhr8rd::DHR8RDrs>;
#[doc = "DUAL DAC 8-bit right aligned data holding register"]
pub mod dhr8rd;
#[doc = "DOR1 (r) register accessor: DAC channel1 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dor1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor1`]
module"]
pub type DOR1 = crate::Reg<dor1::DOR1rs>;
#[doc = "DAC channel1 data output register"]
pub mod dor1;
#[doc = "DOR2 (r) register accessor: DAC channel2 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dor2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor2`]
module"]
pub type DOR2 = crate::Reg<dor2::DOR2rs>;
#[doc = "DAC channel2 data output register"]
pub mod dor2;
#[doc = "SR (rw) register accessor: DAC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "DAC status register"]
pub mod sr;
#[doc = "CCR (rw) register accessor: DAC calibration control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "DAC calibration control register"]
pub mod ccr;
#[doc = "MCR (rw) register accessor: DAC mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
pub type MCR = crate::Reg<mcr::MCRrs>;
#[doc = "DAC mode control register"]
pub mod mcr;
#[doc = "SHSR1 (rw) register accessor: DAC Sample and Hold sample time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shsr1`]
module"]
pub type SHSR1 = crate::Reg<shsr1::SHSR1rs>;
#[doc = "DAC Sample and Hold sample time register 1"]
pub mod shsr1;
#[doc = "SHSR2 (rw) register accessor: DAC Sample and Hold sample time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shsr2`]
module"]
pub type SHSR2 = crate::Reg<shsr2::SHSR2rs>;
#[doc = "DAC Sample and Hold sample time register 2"]
pub mod shsr2;
#[doc = "SHHR (rw) register accessor: DAC Sample and Hold hold time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shhr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shhr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shhr`]
module"]
pub type SHHR = crate::Reg<shhr::SHHRrs>;
#[doc = "DAC Sample and Hold hold time register"]
pub mod shhr;
#[doc = "SHRR (rw) register accessor: DAC Sample and Hold refresh time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrr`]
module"]
pub type SHRR = crate::Reg<shrr::SHRRrs>;
#[doc = "DAC Sample and Hold refresh time register"]
pub mod shrr;
#[doc = "IP_HWCFGR0 (rw) register accessor: DAC IP Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ip_hwcfgr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ip_hwcfgr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ip_hwcfgr0`]
module"]
pub type IP_HWCFGR0 = crate::Reg<ip_hwcfgr0::IP_HWCFGR0rs>;
#[doc = "DAC IP Hardware Configuration Register"]
pub mod ip_hwcfgr0;
#[doc = "VERR (r) register accessor: EXTI IP Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verr`]
module"]
pub type VERR = crate::Reg<verr::VERRrs>;
#[doc = "EXTI IP Version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: EXTI Identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipidr`]
module"]
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
#[doc = "EXTI Identification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: EXTI Size ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidr`]
module"]
pub type SIDR = crate::Reg<sidr::SIDRrs>;
#[doc = "EXTI Size ID register"]
pub mod sidr;
