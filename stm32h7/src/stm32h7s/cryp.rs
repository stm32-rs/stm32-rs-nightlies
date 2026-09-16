#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    dinr: DINR,
    doutr: DOUTR,
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
    ///0x00 - CRYP control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - CRYP status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - CRYP data input register
    #[inline(always)]
    pub const fn dinr(&self) -> &DINR {
        &self.dinr
    }
    ///0x0c - CRYP data output register
    #[inline(always)]
    pub const fn doutr(&self) -> &DOUTR {
        &self.doutr
    }
    ///0x10 - CRYP DMA control register
    #[inline(always)]
    pub const fn dmacr(&self) -> &DMACR {
        &self.dmacr
    }
    ///0x14 - CRYP interrupt mask set/clear register
    #[inline(always)]
    pub const fn imscr(&self) -> &IMSCR {
        &self.imscr
    }
    ///0x18 - CRYP raw interrupt status register
    #[inline(always)]
    pub const fn risr(&self) -> &RISR {
        &self.risr
    }
    ///0x1c - CRYP masked interrupt status register
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
    ///0x50..0x70 - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccmr(&self, n: usize) -> &CSGCMCCMR {
        &self.csgcmccmr[n]
    }
    ///Iterator for array of:
    ///0x50..0x70 - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub fn csgcmccmr_iter(&self) -> impl Iterator<Item = &CSGCMCCMR> {
        self.csgcmccmr.iter()
    }
    ///0x50 - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccm0r(&self) -> &CSGCMCCMR {
        self.csgcmccmr(0)
    }
    ///0x54 - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccm1r(&self) -> &CSGCMCCMR {
        self.csgcmccmr(1)
    }
    ///0x58 - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccm2r(&self) -> &CSGCMCCMR {
        self.csgcmccmr(2)
    }
    ///0x5c - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccm3r(&self) -> &CSGCMCCMR {
        self.csgcmccmr(3)
    }
    ///0x60 - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccm4r(&self) -> &CSGCMCCMR {
        self.csgcmccmr(4)
    }
    ///0x64 - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccm5r(&self) -> &CSGCMCCMR {
        self.csgcmccmr(5)
    }
    ///0x68 - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccm6r(&self) -> &CSGCMCCMR {
        self.csgcmccmr(6)
    }
    ///0x6c - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccm7r(&self) -> &CSGCMCCMR {
        self.csgcmccmr(7)
    }
    ///0x70..0x90 - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcmr(&self, n: usize) -> &CSGCMR {
        &self.csgcmr[n]
    }
    ///Iterator for array of:
    ///0x70..0x90 - CRYP context swap GCM registers
    #[inline(always)]
    pub fn csgcmr_iter(&self) -> impl Iterator<Item = &CSGCMR> {
        self.csgcmr.iter()
    }
    ///0x70 - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcm0r(&self) -> &CSGCMR {
        self.csgcmr(0)
    }
    ///0x74 - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcm1r(&self) -> &CSGCMR {
        self.csgcmr(1)
    }
    ///0x78 - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcm2r(&self) -> &CSGCMR {
        self.csgcmr(2)
    }
    ///0x7c - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcm3r(&self) -> &CSGCMR {
        self.csgcmr(3)
    }
    ///0x80 - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcm4r(&self) -> &CSGCMR {
        self.csgcmr(4)
    }
    ///0x84 - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcm5r(&self) -> &CSGCMR {
        self.csgcmr(5)
    }
    ///0x88 - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcm6r(&self) -> &CSGCMR {
        self.csgcmr(6)
    }
    ///0x8c - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcm7r(&self) -> &CSGCMR {
        self.csgcmr(7)
    }
}
/**CR (rw) register accessor: CRYP control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///CRYP control register
pub mod cr;
/**SR (r) register accessor: CRYP status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///CRYP status register
pub mod sr;
/**DINR (rw) register accessor: CRYP data input register

You can [`read`](crate::Reg::read) this register and get [`dinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:DINR)

For information about available fields see [`mod@dinr`] module*/
pub type DINR = crate::Reg<dinr::DINRrs>;
///CRYP data input register
pub mod dinr;
/**DOUTR (r) register accessor: CRYP data output register

You can [`read`](crate::Reg::read) this register and get [`doutr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:DOUTR)

For information about available fields see [`mod@doutr`] module*/
pub type DOUTR = crate::Reg<doutr::DOUTRrs>;
///CRYP data output register
pub mod doutr;
/**DMACR (rw) register accessor: CRYP DMA control register

You can [`read`](crate::Reg::read) this register and get [`dmacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:DMACR)

For information about available fields see [`mod@dmacr`] module*/
pub type DMACR = crate::Reg<dmacr::DMACRrs>;
///CRYP DMA control register
pub mod dmacr;
/**IMSCR (rw) register accessor: CRYP interrupt mask set/clear register

You can [`read`](crate::Reg::read) this register and get [`imscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:IMSCR)

For information about available fields see [`mod@imscr`] module*/
pub type IMSCR = crate::Reg<imscr::IMSCRrs>;
///CRYP interrupt mask set/clear register
pub mod imscr;
/**RISR (r) register accessor: CRYP raw interrupt status register

You can [`read`](crate::Reg::read) this register and get [`risr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:RISR)

For information about available fields see [`mod@risr`] module*/
pub type RISR = crate::Reg<risr::RISRrs>;
///CRYP raw interrupt status register
pub mod risr;
/**MISR (r) register accessor: CRYP masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:MISR)

For information about available fields see [`mod@misr`] module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///CRYP masked interrupt status register
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
/**CSGCMCCMR (rw) register accessor: CRYP context swap GCM-CCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcmccmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:CSGCMCCM[0]R)

For information about available fields see [`mod@csgcmccmr`] module*/
pub type CSGCMCCMR = crate::Reg<csgcmccmr::CSGCMCCMRrs>;
///CRYP context swap GCM-CCM registers
pub mod csgcmccmr;
/**CSGCMR (rw) register accessor: CRYP context swap GCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:CSGCM[0]R)

For information about available fields see [`mod@csgcmr`] module*/
pub type CSGCMR = crate::Reg<csgcmr::CSGCMRrs>;
///CRYP context swap GCM registers
pub mod csgcmr;
