#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x04],
    cr3: CR3,
    cr4: CR4,
    sr1: SR1,
    sr2: SR2,
    scr: SCR,
    _reserved6: [u8; 0x04],
    pucra: PUCRA,
    pdcra: PDCRA,
    pucrb: PUCRB,
    pdcrb: PDCRB,
    pucrc: PUCRC,
    pdcrc: PDCRC,
    pucrd: PUCRD,
    pdcrd: PDCRD,
    _reserved14: [u8; 0x08],
    pucrf: PUCRF,
    pdcrf: PDCRF,
}
impl RegisterBlock {
    ///0x00 - PWR control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x08 - PWR control register 3
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    ///0x0c - PWR control register 4
    #[inline(always)]
    pub const fn cr4(&self) -> &CR4 {
        &self.cr4
    }
    ///0x10 - PWR status register 1
    #[inline(always)]
    pub const fn sr1(&self) -> &SR1 {
        &self.sr1
    }
    ///0x14 - PWR status register 2
    #[inline(always)]
    pub const fn sr2(&self) -> &SR2 {
        &self.sr2
    }
    ///0x18 - PWR status clear register
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
    }
    ///0x20 - PWR Port A pull-up control register
    #[inline(always)]
    pub const fn pucra(&self) -> &PUCRA {
        &self.pucra
    }
    ///0x24 - PWR Port A pull-down control register
    #[inline(always)]
    pub const fn pdcra(&self) -> &PDCRA {
        &self.pdcra
    }
    ///0x28 - PWR Port B pull-up control register
    #[inline(always)]
    pub const fn pucrb(&self) -> &PUCRB {
        &self.pucrb
    }
    ///0x2c - PWR Port B pull-down control register
    #[inline(always)]
    pub const fn pdcrb(&self) -> &PDCRB {
        &self.pdcrb
    }
    ///0x30 - PWR Port C pull-up control register
    #[inline(always)]
    pub const fn pucrc(&self) -> &PUCRC {
        &self.pucrc
    }
    ///0x34 - PWR Port C pull-down control register
    #[inline(always)]
    pub const fn pdcrc(&self) -> &PDCRC {
        &self.pdcrc
    }
    ///0x38 - PWR Port D pull-up control register
    #[inline(always)]
    pub const fn pucrd(&self) -> &PUCRD {
        &self.pucrd
    }
    ///0x3c - PWR Port D pull-down control register
    #[inline(always)]
    pub const fn pdcrd(&self) -> &PDCRD {
        &self.pdcrd
    }
    ///0x48 - PWR Port F pull-up control register
    #[inline(always)]
    pub const fn pucrf(&self) -> &PUCRF {
        &self.pucrf
    }
    ///0x4c - PWR Port F pull-down control register
    #[inline(always)]
    pub const fn pdcrf(&self) -> &PDCRF {
        &self.pdcrf
    }
}
/**CR1 (rw) register accessor: PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:CR1)

For information about available fields see [`mod@cr1`]
module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///PWR control register 1
pub mod cr1;
/**CR3 (rw) register accessor: PWR control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:CR3)

For information about available fields see [`mod@cr3`]
module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///PWR control register 3
pub mod cr3;
/**CR4 (rw) register accessor: PWR control register 4

You can [`read`](crate::Reg::read) this register and get [`cr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:CR4)

For information about available fields see [`mod@cr4`]
module*/
pub type CR4 = crate::Reg<cr4::CR4rs>;
///PWR control register 4
pub mod cr4;
/**SR1 (r) register accessor: PWR status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:SR1)

For information about available fields see [`mod@sr1`]
module*/
pub type SR1 = crate::Reg<sr1::SR1rs>;
///PWR status register 1
pub mod sr1;
/**SR2 (r) register accessor: PWR status register 2

You can [`read`](crate::Reg::read) this register and get [`sr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:SR2)

For information about available fields see [`mod@sr2`]
module*/
pub type SR2 = crate::Reg<sr2::SR2rs>;
///PWR status register 2
pub mod sr2;
/**SCR (w) register accessor: PWR status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:SCR)

For information about available fields see [`mod@scr`]
module*/
pub type SCR = crate::Reg<scr::SCRrs>;
///PWR status clear register
pub mod scr;
/**PUCRA (rw) register accessor: PWR Port A pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:PUCRA)

For information about available fields see [`mod@pucra`]
module*/
pub type PUCRA = crate::Reg<pucra::PUCRArs>;
///PWR Port A pull-up control register
pub mod pucra;
/**PDCRA (rw) register accessor: PWR Port A pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:PDCRA)

For information about available fields see [`mod@pdcra`]
module*/
pub type PDCRA = crate::Reg<pdcra::PDCRArs>;
///PWR Port A pull-down control register
pub mod pdcra;
/**PUCRB (rw) register accessor: PWR Port B pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:PUCRB)

For information about available fields see [`mod@pucrb`]
module*/
pub type PUCRB = crate::Reg<pucrb::PUCRBrs>;
///PWR Port B pull-up control register
pub mod pucrb;
/**PDCRB (rw) register accessor: PWR Port B pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:PDCRB)

For information about available fields see [`mod@pdcrb`]
module*/
pub type PDCRB = crate::Reg<pdcrb::PDCRBrs>;
///PWR Port B pull-down control register
pub mod pdcrb;
/**PUCRC (rw) register accessor: PWR Port C pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:PUCRC)

For information about available fields see [`mod@pucrc`]
module*/
pub type PUCRC = crate::Reg<pucrc::PUCRCrs>;
///PWR Port C pull-up control register
pub mod pucrc;
/**PDCRC (rw) register accessor: PWR Port C pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:PDCRC)

For information about available fields see [`mod@pdcrc`]
module*/
pub type PDCRC = crate::Reg<pdcrc::PDCRCrs>;
///PWR Port C pull-down control register
pub mod pdcrc;
/**PUCRD (rw) register accessor: PWR Port D pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:PUCRD)

For information about available fields see [`mod@pucrd`]
module*/
pub type PUCRD = crate::Reg<pucrd::PUCRDrs>;
///PWR Port D pull-up control register
pub mod pucrd;
/**PDCRD (rw) register accessor: PWR Port D pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:PDCRD)

For information about available fields see [`mod@pdcrd`]
module*/
pub type PDCRD = crate::Reg<pdcrd::PDCRDrs>;
///PWR Port D pull-down control register
pub mod pdcrd;
/**PUCRF (rw) register accessor: PWR Port F pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:PUCRF)

For information about available fields see [`mod@pucrf`]
module*/
pub type PUCRF = crate::Reg<pucrf::PUCRFrs>;
///PWR Port F pull-up control register
pub mod pucrf;
/**PDCRF (rw) register accessor: PWR Port F pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:PDCRF)

For information about available fields see [`mod@pdcrf`]
module*/
pub type PDCRF = crate::Reg<pdcrf::PDCRFrs>;
///PWR Port F pull-down control register
pub mod pdcrf;
