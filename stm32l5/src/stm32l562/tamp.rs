#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
    fltcr: FLTCR,
    atcr1: ATCR1,
    atseedr: ATSEEDR,
    ator: ATOR,
    atcr2: ATCR2,
    smcr: SMCR,
    privcr: PRIVCR,
    _reserved10: [u8; 0x04],
    ier: IER,
    sr: SR,
    misr: MISR,
    smisr: SMISR,
    scr: SCR,
    countr: COUNTR,
    _reserved16: [u8; 0x0c],
    cfgr: CFGR,
    _reserved17: [u8; 0xac],
    bkpr: [BKPR; 32],
}
impl RegisterBlock {
    #[doc = "0x00 - control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x08 - control register 3"]
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    #[doc = "0x0c - TAMP filter control register"]
    #[inline(always)]
    pub const fn fltcr(&self) -> &FLTCR {
        &self.fltcr
    }
    #[doc = "0x10 - TAMP active tamper control register 1"]
    #[inline(always)]
    pub const fn atcr1(&self) -> &ATCR1 {
        &self.atcr1
    }
    #[doc = "0x14 - TAMP active tamper seed register"]
    #[inline(always)]
    pub const fn atseedr(&self) -> &ATSEEDR {
        &self.atseedr
    }
    #[doc = "0x18 - TAMP active tamper output register"]
    #[inline(always)]
    pub const fn ator(&self) -> &ATOR {
        &self.ator
    }
    #[doc = "0x1c - TAMP active tamper control register 2"]
    #[inline(always)]
    pub const fn atcr2(&self) -> &ATCR2 {
        &self.atcr2
    }
    #[doc = "0x20 - TAMP secure mode register"]
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
    }
    #[doc = "0x24 - TAMP privilege mode control register"]
    #[inline(always)]
    pub const fn privcr(&self) -> &PRIVCR {
        &self.privcr
    }
    #[doc = "0x2c - TAMP interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x30 - TAMP status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x34 - TAMP masked interrupt status register"]
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    #[doc = "0x38 - TAMP secure masked interrupt status register"]
    #[inline(always)]
    pub const fn smisr(&self) -> &SMISR {
        &self.smisr
    }
    #[doc = "0x3c - TAMP status clear register"]
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
    }
    #[doc = "0x40 - TAMP monotonic counter register"]
    #[inline(always)]
    pub const fn countr(&self) -> &COUNTR {
        &self.countr
    }
    #[doc = "0x50 - TAMP configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x100..0x180 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkpr(&self, n: usize) -> &BKPR {
        &self.bkpr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x180 - TAMP backup register"]
    #[inline(always)]
    pub fn bkpr_iter(&self) -> impl Iterator<Item = &BKPR> {
        self.bkpr.iter()
    }
    #[doc = "0x100 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp0r(&self) -> &BKPR {
        self.bkpr(0)
    }
    #[doc = "0x104 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp1r(&self) -> &BKPR {
        self.bkpr(1)
    }
    #[doc = "0x108 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp2r(&self) -> &BKPR {
        self.bkpr(2)
    }
    #[doc = "0x10c - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp3r(&self) -> &BKPR {
        self.bkpr(3)
    }
    #[doc = "0x110 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp4r(&self) -> &BKPR {
        self.bkpr(4)
    }
    #[doc = "0x114 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp5r(&self) -> &BKPR {
        self.bkpr(5)
    }
    #[doc = "0x118 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp6r(&self) -> &BKPR {
        self.bkpr(6)
    }
    #[doc = "0x11c - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp7r(&self) -> &BKPR {
        self.bkpr(7)
    }
    #[doc = "0x120 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp8r(&self) -> &BKPR {
        self.bkpr(8)
    }
    #[doc = "0x124 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp9r(&self) -> &BKPR {
        self.bkpr(9)
    }
    #[doc = "0x128 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp10r(&self) -> &BKPR {
        self.bkpr(10)
    }
    #[doc = "0x12c - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp11r(&self) -> &BKPR {
        self.bkpr(11)
    }
    #[doc = "0x130 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp12r(&self) -> &BKPR {
        self.bkpr(12)
    }
    #[doc = "0x134 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp13r(&self) -> &BKPR {
        self.bkpr(13)
    }
    #[doc = "0x138 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp14r(&self) -> &BKPR {
        self.bkpr(14)
    }
    #[doc = "0x13c - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp15r(&self) -> &BKPR {
        self.bkpr(15)
    }
    #[doc = "0x140 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp16r(&self) -> &BKPR {
        self.bkpr(16)
    }
    #[doc = "0x144 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp17r(&self) -> &BKPR {
        self.bkpr(17)
    }
    #[doc = "0x148 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp18r(&self) -> &BKPR {
        self.bkpr(18)
    }
    #[doc = "0x14c - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp19r(&self) -> &BKPR {
        self.bkpr(19)
    }
    #[doc = "0x150 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp20r(&self) -> &BKPR {
        self.bkpr(20)
    }
    #[doc = "0x154 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp21r(&self) -> &BKPR {
        self.bkpr(21)
    }
    #[doc = "0x158 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp22r(&self) -> &BKPR {
        self.bkpr(22)
    }
    #[doc = "0x15c - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp23r(&self) -> &BKPR {
        self.bkpr(23)
    }
    #[doc = "0x160 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp24r(&self) -> &BKPR {
        self.bkpr(24)
    }
    #[doc = "0x164 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp25r(&self) -> &BKPR {
        self.bkpr(25)
    }
    #[doc = "0x168 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp26r(&self) -> &BKPR {
        self.bkpr(26)
    }
    #[doc = "0x16c - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp27r(&self) -> &BKPR {
        self.bkpr(27)
    }
    #[doc = "0x170 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp28r(&self) -> &BKPR {
        self.bkpr(28)
    }
    #[doc = "0x174 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp29r(&self) -> &BKPR {
        self.bkpr(29)
    }
    #[doc = "0x178 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp30r(&self) -> &BKPR {
        self.bkpr(30)
    }
    #[doc = "0x17c - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp31r(&self) -> &BKPR {
        self.bkpr(31)
    }
}
#[doc = "CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`]
module"]
pub type CR3 = crate::Reg<cr3::CR3rs>;
#[doc = "control register 3"]
pub mod cr3;
#[doc = "FLTCR (rw) register accessor: TAMP filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltcr`]
module"]
pub type FLTCR = crate::Reg<fltcr::FLTCRrs>;
#[doc = "TAMP filter control register"]
pub mod fltcr;
#[doc = "ATCR1 (rw) register accessor: TAMP active tamper control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atcr1`]
module"]
pub type ATCR1 = crate::Reg<atcr1::ATCR1rs>;
#[doc = "TAMP active tamper control register 1"]
pub mod atcr1;
#[doc = "ATSEEDR (w) register accessor: TAMP active tamper seed register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atseedr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atseedr`]
module"]
pub type ATSEEDR = crate::Reg<atseedr::ATSEEDRrs>;
#[doc = "TAMP active tamper seed register"]
pub mod atseedr;
#[doc = "ATOR (r) register accessor: TAMP active tamper output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ator::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ator`]
module"]
pub type ATOR = crate::Reg<ator::ATORrs>;
#[doc = "TAMP active tamper output register"]
pub mod ator;
#[doc = "ATCR2 (rw) register accessor: TAMP active tamper control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atcr2`]
module"]
pub type ATCR2 = crate::Reg<atcr2::ATCR2rs>;
#[doc = "TAMP active tamper control register 2"]
pub mod atcr2;
#[doc = "SMCR (rw) register accessor: TAMP secure mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcr`]
module"]
pub type SMCR = crate::Reg<smcr::SMCRrs>;
#[doc = "TAMP secure mode register"]
pub mod smcr;
#[doc = "PRIVCR (rw) register accessor: TAMP privilege mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcr`]
module"]
pub type PRIVCR = crate::Reg<privcr::PRIVCRrs>;
#[doc = "TAMP privilege mode control register"]
pub mod privcr;
#[doc = "IER (rw) register accessor: TAMP interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "TAMP interrupt enable register"]
pub mod ier;
#[doc = "SR (r) register accessor: TAMP status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "TAMP status register"]
pub mod sr;
#[doc = "MISR (r) register accessor: TAMP masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misr`]
module"]
pub type MISR = crate::Reg<misr::MISRrs>;
#[doc = "TAMP masked interrupt status register"]
pub mod misr;
#[doc = "SMISR (r) register accessor: TAMP secure masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smisr`]
module"]
pub type SMISR = crate::Reg<smisr::SMISRrs>;
#[doc = "TAMP secure masked interrupt status register"]
pub mod smisr;
#[doc = "SCR (w) register accessor: TAMP status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
pub type SCR = crate::Reg<scr::SCRrs>;
#[doc = "TAMP status clear register"]
pub mod scr;
#[doc = "COUNTR (r) register accessor: TAMP monotonic counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`countr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@countr`]
module"]
pub type COUNTR = crate::Reg<countr::COUNTRrs>;
#[doc = "TAMP monotonic counter register"]
pub mod countr;
#[doc = "CFGR (rw) register accessor: TAMP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
#[doc = "TAMP configuration register"]
pub mod cfgr;
#[doc = "BKPR (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkpr`]
module"]
pub type BKPR = crate::Reg<bkpr::BKPRrs>;
#[doc = "TAMP backup register"]
pub mod bkpr;
