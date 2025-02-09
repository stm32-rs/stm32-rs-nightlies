#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
    cr4: CR4,
    sr1: SR1,
    sr2: SR2,
    scr: SCR,
    _reserved7: [u8; 0x04],
    pucra: PUCRA,
    pdcra: PDCRA,
    pucrb: PUCRB,
    pdcrb: PDCRB,
    pucrc: PUCRC,
    pdcrc: PDCRC,
    pucrd: PUCRD,
    pdcrd: PDCRD,
    _reserved15: [u8; 0x08],
    pucrf: PUCRF,
    pdcrf: PDCRF,
    _reserved17: [u8; 0x20],
    bkp0r: BKP0R,
    bkp1r: BKP1R,
    bkp2r: BKP2R,
    bkp3r: BKP3R,
}
impl RegisterBlock {
    ///0x00 - PWR control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - PWR control register 1
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
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
    ///0x70 - PWR backup 0 register
    #[inline(always)]
    pub const fn bkp0r(&self) -> &BKP0R {
        &self.bkp0r
    }
    ///0x74 - PWR backup 1 register
    #[inline(always)]
    pub const fn bkp1r(&self) -> &BKP1R {
        &self.bkp1r
    }
    ///0x78 - PWR backup 2 register
    #[inline(always)]
    pub const fn bkp2r(&self) -> &BKP2R {
        &self.bkp2r
    }
    ///0x7c - PWR backup 3 register
    #[inline(always)]
    pub const fn bkp3r(&self) -> &BKP3R {
        &self.bkp3r
    }
}
/**CR1 (rw) register accessor: PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:CR1)

For information about available fields see [`mod@cr1`]
module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///PWR control register 1
pub mod cr1;
/**CR2 (rw) register accessor: PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:CR2)

For information about available fields see [`mod@cr2`]
module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///PWR control register 1
pub mod cr2;
/**CR3 (rw) register accessor: PWR control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:CR3)

For information about available fields see [`mod@cr3`]
module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///PWR control register 3
pub mod cr3;
/**CR4 (rw) register accessor: PWR control register 4

You can [`read`](crate::Reg::read) this register and get [`cr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:CR4)

For information about available fields see [`mod@cr4`]
module*/
pub type CR4 = crate::Reg<cr4::CR4rs>;
///PWR control register 4
pub mod cr4;
/**SR1 (r) register accessor: PWR status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:SR1)

For information about available fields see [`mod@sr1`]
module*/
pub type SR1 = crate::Reg<sr1::SR1rs>;
///PWR status register 1
pub mod sr1;
/**SR2 (r) register accessor: PWR status register 2

You can [`read`](crate::Reg::read) this register and get [`sr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:SR2)

For information about available fields see [`mod@sr2`]
module*/
pub type SR2 = crate::Reg<sr2::SR2rs>;
///PWR status register 2
pub mod sr2;
/**SCR (w) register accessor: PWR status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:SCR)

For information about available fields see [`mod@scr`]
module*/
pub type SCR = crate::Reg<scr::SCRrs>;
///PWR status clear register
pub mod scr;
/**PUCRA (rw) register accessor: PWR Port A pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:PUCRA)

For information about available fields see [`mod@pucra`]
module*/
pub type PUCRA = crate::Reg<pucra::PUCRArs>;
///PWR Port A pull-up control register
pub mod pucra;
/**PDCRA (rw) register accessor: PWR Port A pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:PDCRA)

For information about available fields see [`mod@pdcra`]
module*/
pub type PDCRA = crate::Reg<pdcra::PDCRArs>;
///PWR Port A pull-down control register
pub mod pdcra;
/**PUCRB (rw) register accessor: PWR Port B pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:PUCRB)

For information about available fields see [`mod@pucrb`]
module*/
pub type PUCRB = crate::Reg<pucrb::PUCRBrs>;
///PWR Port B pull-up control register
pub mod pucrb;
/**PDCRB (rw) register accessor: PWR Port B pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:PDCRB)

For information about available fields see [`mod@pdcrb`]
module*/
pub type PDCRB = crate::Reg<pdcrb::PDCRBrs>;
///PWR Port B pull-down control register
pub mod pdcrb;
/**PUCRC (rw) register accessor: PWR Port C pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:PUCRC)

For information about available fields see [`mod@pucrc`]
module*/
pub type PUCRC = crate::Reg<pucrc::PUCRCrs>;
///PWR Port C pull-up control register
pub mod pucrc;
/**PDCRC (rw) register accessor: PWR Port C pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:PDCRC)

For information about available fields see [`mod@pdcrc`]
module*/
pub type PDCRC = crate::Reg<pdcrc::PDCRCrs>;
///PWR Port C pull-down control register
pub mod pdcrc;
/**PUCRD (rw) register accessor: PWR Port D pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:PUCRD)

For information about available fields see [`mod@pucrd`]
module*/
pub type PUCRD = crate::Reg<pucrd::PUCRDrs>;
///PWR Port D pull-up control register
pub mod pucrd;
/**PDCRD (rw) register accessor: PWR Port D pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:PDCRD)

For information about available fields see [`mod@pdcrd`]
module*/
pub type PDCRD = crate::Reg<pdcrd::PDCRDrs>;
///PWR Port D pull-down control register
pub mod pdcrd;
/**PUCRF (rw) register accessor: PWR Port F pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:PUCRF)

For information about available fields see [`mod@pucrf`]
module*/
pub type PUCRF = crate::Reg<pucrf::PUCRFrs>;
///PWR Port F pull-up control register
pub mod pucrf;
/**PDCRF (rw) register accessor: PWR Port F pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:PDCRF)

For information about available fields see [`mod@pdcrf`]
module*/
pub type PDCRF = crate::Reg<pdcrf::PDCRFrs>;
///PWR Port F pull-down control register
pub mod pdcrf;
/**BKP0R (rw) register accessor: PWR backup 0 register

You can [`read`](crate::Reg::read) this register and get [`bkp0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:BKP0R)

For information about available fields see [`mod@bkp0r`]
module*/
pub type BKP0R = crate::Reg<bkp0r::BKP0Rrs>;
///PWR backup 0 register
pub mod bkp0r;
/**BKP1R (rw) register accessor: PWR backup 1 register

You can [`read`](crate::Reg::read) this register and get [`bkp1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:BKP1R)

For information about available fields see [`mod@bkp1r`]
module*/
pub type BKP1R = crate::Reg<bkp1r::BKP1Rrs>;
///PWR backup 1 register
pub mod bkp1r;
/**BKP2R (rw) register accessor: PWR backup 2 register

You can [`read`](crate::Reg::read) this register and get [`bkp2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:BKP2R)

For information about available fields see [`mod@bkp2r`]
module*/
pub type BKP2R = crate::Reg<bkp2r::BKP2Rrs>;
///PWR backup 2 register
pub mod bkp2r;
/**BKP3R (rw) register accessor: PWR backup 3 register

You can [`read`](crate::Reg::read) this register and get [`bkp3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:BKP3R)

For information about available fields see [`mod@bkp3r`]
module*/
pub type BKP3R = crate::Reg<bkp3r::BKP3Rrs>;
///PWR backup 3 register
pub mod bkp3r;
