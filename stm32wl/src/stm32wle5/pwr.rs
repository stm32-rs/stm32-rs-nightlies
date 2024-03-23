#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
    cr4: CR4,
    sr1: SR1,
    sr2: SR2,
    scr: SCR,
    cr5: CR5,
    pucra: PUCRA,
    pdcra: PDCRA,
    pucrb: PUCRB,
    pdcrb: PDCRB,
    pucrc: PUCRC,
    pdcrc: PDCRC,
    _reserved14: [u8; 0x20],
    pucrh: PUCRH,
    pdcrh: PDCRH,
    _reserved16: [u8; 0x28],
    extscr: EXTSCR,
    _reserved17: [u8; 0x04],
    subghzspicr: SUBGHZSPICR,
}
impl RegisterBlock {
    #[doc = "0x00 - Power control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - Power control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x08 - Power control register 3"]
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    #[doc = "0x0c - Power control register 4"]
    #[inline(always)]
    pub const fn cr4(&self) -> &CR4 {
        &self.cr4
    }
    #[doc = "0x10 - Power status register 1"]
    #[inline(always)]
    pub const fn sr1(&self) -> &SR1 {
        &self.sr1
    }
    #[doc = "0x14 - Power status register 2"]
    #[inline(always)]
    pub const fn sr2(&self) -> &SR2 {
        &self.sr2
    }
    #[doc = "0x18 - Power status clear register"]
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
    }
    #[doc = "0x1c - Power control register 5"]
    #[inline(always)]
    pub const fn cr5(&self) -> &CR5 {
        &self.cr5
    }
    #[doc = "0x20 - Power Port A pull-up control register"]
    #[inline(always)]
    pub const fn pucra(&self) -> &PUCRA {
        &self.pucra
    }
    #[doc = "0x24 - Power Port A pull-down control register"]
    #[inline(always)]
    pub const fn pdcra(&self) -> &PDCRA {
        &self.pdcra
    }
    #[doc = "0x28 - Power Port B pull-up control register"]
    #[inline(always)]
    pub const fn pucrb(&self) -> &PUCRB {
        &self.pucrb
    }
    #[doc = "0x2c - Power Port B pull-down control register"]
    #[inline(always)]
    pub const fn pdcrb(&self) -> &PDCRB {
        &self.pdcrb
    }
    #[doc = "0x30 - Power Port C pull-up control register"]
    #[inline(always)]
    pub const fn pucrc(&self) -> &PUCRC {
        &self.pucrc
    }
    #[doc = "0x34 - Power Port C pull-down control register"]
    #[inline(always)]
    pub const fn pdcrc(&self) -> &PDCRC {
        &self.pdcrc
    }
    #[doc = "0x58 - Power Port H pull-up control register"]
    #[inline(always)]
    pub const fn pucrh(&self) -> &PUCRH {
        &self.pucrh
    }
    #[doc = "0x5c - Power Port H pull-down control register"]
    #[inline(always)]
    pub const fn pdcrh(&self) -> &PDCRH {
        &self.pdcrh
    }
    #[doc = "0x88 - Power extended status and status clear register"]
    #[inline(always)]
    pub const fn extscr(&self) -> &EXTSCR {
        &self.extscr
    }
    #[doc = "0x90 - Power SPI3 control register"]
    #[inline(always)]
    pub const fn subghzspicr(&self) -> &SUBGHZSPICR {
        &self.subghzspicr
    }
}
#[doc = "CR1 (rw) register accessor: Power control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "Power control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Power control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "Power control register 2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: Power control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`]
module"]
pub type CR3 = crate::Reg<cr3::CR3rs>;
#[doc = "Power control register 3"]
pub mod cr3;
#[doc = "CR4 (rw) register accessor: Power control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr4`]
module"]
pub type CR4 = crate::Reg<cr4::CR4rs>;
#[doc = "Power control register 4"]
pub mod cr4;
#[doc = "SR1 (r) register accessor: Power status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`]
module"]
pub type SR1 = crate::Reg<sr1::SR1rs>;
#[doc = "Power status register 1"]
pub mod sr1;
#[doc = "SR2 (r) register accessor: Power status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr2`]
module"]
pub type SR2 = crate::Reg<sr2::SR2rs>;
#[doc = "Power status register 2"]
pub mod sr2;
#[doc = "SCR (w) register accessor: Power status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
pub type SCR = crate::Reg<scr::SCRrs>;
#[doc = "Power status clear register"]
pub mod scr;
#[doc = "CR5 (rw) register accessor: Power control register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr5`]
module"]
pub type CR5 = crate::Reg<cr5::CR5rs>;
#[doc = "Power control register 5"]
pub mod cr5;
#[doc = "PUCRA (rw) register accessor: Power Port A pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucra`]
module"]
pub type PUCRA = crate::Reg<pucra::PUCRArs>;
#[doc = "Power Port A pull-up control register"]
pub mod pucra;
#[doc = "PDCRA (rw) register accessor: Power Port A pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcra`]
module"]
pub type PDCRA = crate::Reg<pdcra::PDCRArs>;
#[doc = "Power Port A pull-down control register"]
pub mod pdcra;
#[doc = "PUCRB (rw) register accessor: Power Port B pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucrb`]
module"]
pub type PUCRB = crate::Reg<pucrb::PUCRBrs>;
#[doc = "Power Port B pull-up control register"]
pub mod pucrb;
#[doc = "PDCRB (rw) register accessor: Power Port B pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcrb`]
module"]
pub type PDCRB = crate::Reg<pdcrb::PDCRBrs>;
#[doc = "Power Port B pull-down control register"]
pub mod pdcrb;
#[doc = "PUCRC (rw) register accessor: Power Port C pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucrc`]
module"]
pub type PUCRC = crate::Reg<pucrc::PUCRCrs>;
#[doc = "Power Port C pull-up control register"]
pub mod pucrc;
#[doc = "PDCRC (rw) register accessor: Power Port C pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcrc`]
module"]
pub type PDCRC = crate::Reg<pdcrc::PDCRCrs>;
#[doc = "Power Port C pull-down control register"]
pub mod pdcrc;
#[doc = "PUCRH (rw) register accessor: Power Port H pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucrh`]
module"]
pub type PUCRH = crate::Reg<pucrh::PUCRHrs>;
#[doc = "Power Port H pull-up control register"]
pub mod pucrh;
#[doc = "PDCRH (rw) register accessor: Power Port H pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcrh`]
module"]
pub type PDCRH = crate::Reg<pdcrh::PDCRHrs>;
#[doc = "Power Port H pull-down control register"]
pub mod pdcrh;
#[doc = "EXTSCR (rw) register accessor: Power extended status and status clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extscr`]
module"]
pub type EXTSCR = crate::Reg<extscr::EXTSCRrs>;
#[doc = "Power extended status and status clear register"]
pub mod extscr;
#[doc = "SUBGHZSPICR (rw) register accessor: Power SPI3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subghzspicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subghzspicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subghzspicr`]
module"]
pub type SUBGHZSPICR = crate::Reg<subghzspicr::SUBGHZSPICRrs>;
#[doc = "Power SPI3 control register"]
pub mod subghzspicr;
