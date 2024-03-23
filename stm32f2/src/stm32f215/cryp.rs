#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    din: DIN,
    dout: DOUT,
    dmacr: DMACR,
    imscr: IMSCR,
    risr: RISR,
    misr: MISR,
    key: [KEY; 4],
    init: [INIT; 2],
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x08 - data input register"]
    #[inline(always)]
    pub const fn din(&self) -> &DIN {
        &self.din
    }
    #[doc = "0x0c - data output register"]
    #[inline(always)]
    pub const fn dout(&self) -> &DOUT {
        &self.dout
    }
    #[doc = "0x10 - DMA control register"]
    #[inline(always)]
    pub const fn dmacr(&self) -> &DMACR {
        &self.dmacr
    }
    #[doc = "0x14 - interrupt mask set/clear register"]
    #[inline(always)]
    pub const fn imscr(&self) -> &IMSCR {
        &self.imscr
    }
    #[doc = "0x18 - raw interrupt status register"]
    #[inline(always)]
    pub const fn risr(&self) -> &RISR {
        &self.risr
    }
    #[doc = "0x1c - masked interrupt status register"]
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    #[doc = "0x20..0x40 - Cluster KEY%s, containing K?LR, K?RR"]
    #[inline(always)]
    pub const fn key(&self, n: usize) -> &KEY {
        &self.key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x40 - Cluster KEY%s, containing K?LR, K?RR"]
    #[inline(always)]
    pub fn key_iter(&self) -> impl Iterator<Item = &KEY> {
        self.key.iter()
    }
    #[doc = "0x40..0x50 - Cluster INIT%s, containing IV?LR, IV?RR"]
    #[inline(always)]
    pub const fn init(&self, n: usize) -> &INIT {
        &self.init[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x50 - Cluster INIT%s, containing IV?LR, IV?RR"]
    #[inline(always)]
    pub fn init_iter(&self) -> impl Iterator<Item = &INIT> {
        self.init.iter()
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "control register"]
pub mod cr;
#[doc = "SR (r) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "status register"]
pub mod sr;
#[doc = "DIN (rw) register accessor: data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din`]
module"]
pub type DIN = crate::Reg<din::DINrs>;
#[doc = "data input register"]
pub mod din;
#[doc = "DOUT (r) register accessor: data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout`]
module"]
pub type DOUT = crate::Reg<dout::DOUTrs>;
#[doc = "data output register"]
pub mod dout;
#[doc = "DMACR (rw) register accessor: DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacr`]
module"]
pub type DMACR = crate::Reg<dmacr::DMACRrs>;
#[doc = "DMA control register"]
pub mod dmacr;
#[doc = "IMSCR (rw) register accessor: interrupt mask set/clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imscr`]
module"]
pub type IMSCR = crate::Reg<imscr::IMSCRrs>;
#[doc = "interrupt mask set/clear register"]
pub mod imscr;
#[doc = "RISR (r) register accessor: raw interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`risr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@risr`]
module"]
pub type RISR = crate::Reg<risr::RISRrs>;
#[doc = "raw interrupt status register"]
pub mod risr;
#[doc = "MISR (r) register accessor: masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misr`]
module"]
pub type MISR = crate::Reg<misr::MISRrs>;
#[doc = "masked interrupt status register"]
pub mod misr;
#[doc = "Cluster KEY%s, containing K?LR, K?RR"]
pub use self::key::KEY;
#[doc = r"Cluster"]
#[doc = "Cluster KEY%s, containing K?LR, K?RR"]
pub mod key;
#[doc = "Cluster INIT%s, containing IV?LR, IV?RR"]
pub use self::init::INIT;
#[doc = r"Cluster"]
#[doc = "Cluster INIT%s, containing IV?LR, IV?RR"]
pub mod init;
