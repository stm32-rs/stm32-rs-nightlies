#[repr(C)]
#[derive(Debug)]
///Register block
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
    csgcmccmr: [CSGCMCCMR; 8],
    csgcmr: [CSGCMR; 8],
}
impl RegisterBlock {
    ///0x00 - control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - data input register
    #[inline(always)]
    pub const fn din(&self) -> &DIN {
        &self.din
    }
    ///0x0c - data output register
    #[inline(always)]
    pub const fn dout(&self) -> &DOUT {
        &self.dout
    }
    ///0x10 - DMA control register
    #[inline(always)]
    pub const fn dmacr(&self) -> &DMACR {
        &self.dmacr
    }
    ///0x14 - interrupt mask set/clear register
    #[inline(always)]
    pub const fn imscr(&self) -> &IMSCR {
        &self.imscr
    }
    ///0x18 - raw interrupt status register
    #[inline(always)]
    pub const fn risr(&self) -> &RISR {
        &self.risr
    }
    ///0x1c - masked interrupt status register
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    ///0x20..0x40 - Cluster KEY%s, containing K?LR, K?RR
    #[inline(always)]
    pub const fn key(&self, n: usize) -> &KEY {
        &self.key[n]
    }
    ///Iterator for array of:
    ///0x20..0x40 - Cluster KEY%s, containing K?LR, K?RR
    #[inline(always)]
    pub fn key_iter(&self) -> impl Iterator<Item = &KEY> {
        self.key.iter()
    }
    ///0x40..0x50 - Cluster INIT%s, containing IV?LR, IV?RR
    #[inline(always)]
    pub const fn init(&self, n: usize) -> &INIT {
        &self.init[n]
    }
    ///Iterator for array of:
    ///0x40..0x50 - Cluster INIT%s, containing IV?LR, IV?RR
    #[inline(always)]
    pub fn init_iter(&self) -> impl Iterator<Item = &INIT> {
        self.init.iter()
    }
    ///0x50..0x70 - context swap register
    #[inline(always)]
    pub const fn csgcmccmr(&self, n: usize) -> &CSGCMCCMR {
        &self.csgcmccmr[n]
    }
    ///Iterator for array of:
    ///0x50..0x70 - context swap register
    #[inline(always)]
    pub fn csgcmccmr_iter(&self) -> impl Iterator<Item = &CSGCMCCMR> {
        self.csgcmccmr.iter()
    }
    ///0x50 - context swap register
    #[inline(always)]
    pub const fn csgcmccm0r(&self) -> &CSGCMCCMR {
        self.csgcmccmr(0)
    }
    ///0x54 - context swap register
    #[inline(always)]
    pub const fn csgcmccm1r(&self) -> &CSGCMCCMR {
        self.csgcmccmr(1)
    }
    ///0x58 - context swap register
    #[inline(always)]
    pub const fn csgcmccm2r(&self) -> &CSGCMCCMR {
        self.csgcmccmr(2)
    }
    ///0x5c - context swap register
    #[inline(always)]
    pub const fn csgcmccm3r(&self) -> &CSGCMCCMR {
        self.csgcmccmr(3)
    }
    ///0x60 - context swap register
    #[inline(always)]
    pub const fn csgcmccm4r(&self) -> &CSGCMCCMR {
        self.csgcmccmr(4)
    }
    ///0x64 - context swap register
    #[inline(always)]
    pub const fn csgcmccm5r(&self) -> &CSGCMCCMR {
        self.csgcmccmr(5)
    }
    ///0x68 - context swap register
    #[inline(always)]
    pub const fn csgcmccm6r(&self) -> &CSGCMCCMR {
        self.csgcmccmr(6)
    }
    ///0x6c - context swap register
    #[inline(always)]
    pub const fn csgcmccm7r(&self) -> &CSGCMCCMR {
        self.csgcmccmr(7)
    }
    ///0x70..0x90 - context swap register
    #[inline(always)]
    pub const fn csgcmr(&self, n: usize) -> &CSGCMR {
        &self.csgcmr[n]
    }
    ///Iterator for array of:
    ///0x70..0x90 - context swap register
    #[inline(always)]
    pub fn csgcmr_iter(&self) -> impl Iterator<Item = &CSGCMR> {
        self.csgcmr.iter()
    }
    ///0x70 - context swap register
    #[inline(always)]
    pub const fn csgcm0r(&self) -> &CSGCMR {
        self.csgcmr(0)
    }
    ///0x74 - context swap register
    #[inline(always)]
    pub const fn csgcm1r(&self) -> &CSGCMR {
        self.csgcmr(1)
    }
    ///0x78 - context swap register
    #[inline(always)]
    pub const fn csgcm2r(&self) -> &CSGCMR {
        self.csgcmr(2)
    }
    ///0x7c - context swap register
    #[inline(always)]
    pub const fn csgcm3r(&self) -> &CSGCMR {
        self.csgcmr(3)
    }
    ///0x80 - context swap register
    #[inline(always)]
    pub const fn csgcm4r(&self) -> &CSGCMR {
        self.csgcmr(4)
    }
    ///0x84 - context swap register
    #[inline(always)]
    pub const fn csgcm5r(&self) -> &CSGCMR {
        self.csgcmr(5)
    }
    ///0x88 - context swap register
    #[inline(always)]
    pub const fn csgcm6r(&self) -> &CSGCMR {
        self.csgcmr(6)
    }
    ///0x8c - context swap register
    #[inline(always)]
    pub const fn csgcm7r(&self) -> &CSGCMR {
        self.csgcmr(7)
    }
}
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#CRYP:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**SR (r) register accessor: status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#CRYP:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///status register
pub mod sr;
/**DIN (rw) register accessor: data input register

You can [`read`](crate::Reg::read) this register and get [`din::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#CRYP:DIN)

For information about available fields see [`mod@din`] module*/
pub type DIN = crate::Reg<din::DINrs>;
///data input register
pub mod din;
/**DOUT (r) register accessor: data output register

You can [`read`](crate::Reg::read) this register and get [`dout::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#CRYP:DOUT)

For information about available fields see [`mod@dout`] module*/
pub type DOUT = crate::Reg<dout::DOUTrs>;
///data output register
pub mod dout;
/**DMACR (rw) register accessor: DMA control register

You can [`read`](crate::Reg::read) this register and get [`dmacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#CRYP:DMACR)

For information about available fields see [`mod@dmacr`] module*/
pub type DMACR = crate::Reg<dmacr::DMACRrs>;
///DMA control register
pub mod dmacr;
/**IMSCR (rw) register accessor: interrupt mask set/clear register

You can [`read`](crate::Reg::read) this register and get [`imscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#CRYP:IMSCR)

For information about available fields see [`mod@imscr`] module*/
pub type IMSCR = crate::Reg<imscr::IMSCRrs>;
///interrupt mask set/clear register
pub mod imscr;
/**RISR (r) register accessor: raw interrupt status register

You can [`read`](crate::Reg::read) this register and get [`risr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#CRYP:RISR)

For information about available fields see [`mod@risr`] module*/
pub type RISR = crate::Reg<risr::RISRrs>;
///raw interrupt status register
pub mod risr;
/**MISR (r) register accessor: masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#CRYP:MISR)

For information about available fields see [`mod@misr`] module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///masked interrupt status register
pub mod misr;
///Cluster KEY%s, containing K?LR, K?RR
pub use self::key::KEY;
///Cluster
///Cluster KEY%s, containing K?LR, K?RR
pub mod key;
///Cluster INIT%s, containing IV?LR, IV?RR
pub use self::init::INIT;
///Cluster
///Cluster INIT%s, containing IV?LR, IV?RR
pub mod init;
/**CSGCMCCMR (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#CRYP:CSGCMCCM[0]R)

For information about available fields see [`mod@csgcmccmr`] module*/
pub type CSGCMCCMR = crate::Reg<csgcmccmr::CSGCMCCMRrs>;
///context swap register
pub mod csgcmccmr;
/**CSGCMR (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#CRYP:CSGCM[0]R)

For information about available fields see [`mod@csgcmr`] module*/
pub type CSGCMR = crate::Reg<csgcmr::CSGCMRrs>;
///context swap register
pub mod csgcmr;
