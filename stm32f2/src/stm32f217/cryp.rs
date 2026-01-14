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
}
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#CRYP:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**SR (r) register accessor: status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#CRYP:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///status register
pub mod sr;
/**DIN (rw) register accessor: data input register

You can [`read`](crate::Reg::read) this register and get [`din::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#CRYP:DIN)

For information about available fields see [`mod@din`] module*/
pub type DIN = crate::Reg<din::DINrs>;
///data input register
pub mod din;
/**DOUT (r) register accessor: data output register

You can [`read`](crate::Reg::read) this register and get [`dout::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#CRYP:DOUT)

For information about available fields see [`mod@dout`] module*/
pub type DOUT = crate::Reg<dout::DOUTrs>;
///data output register
pub mod dout;
/**DMACR (rw) register accessor: DMA control register

You can [`read`](crate::Reg::read) this register and get [`dmacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#CRYP:DMACR)

For information about available fields see [`mod@dmacr`] module*/
pub type DMACR = crate::Reg<dmacr::DMACRrs>;
///DMA control register
pub mod dmacr;
/**IMSCR (rw) register accessor: interrupt mask set/clear register

You can [`read`](crate::Reg::read) this register and get [`imscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#CRYP:IMSCR)

For information about available fields see [`mod@imscr`] module*/
pub type IMSCR = crate::Reg<imscr::IMSCRrs>;
///interrupt mask set/clear register
pub mod imscr;
/**RISR (r) register accessor: raw interrupt status register

You can [`read`](crate::Reg::read) this register and get [`risr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#CRYP:RISR)

For information about available fields see [`mod@risr`] module*/
pub type RISR = crate::Reg<risr::RISRrs>;
///raw interrupt status register
pub mod risr;
/**MISR (r) register accessor: masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#CRYP:MISR)

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
