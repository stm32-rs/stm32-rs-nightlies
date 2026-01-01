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
    _reserved15: [u8; 0x04],
    pdcre: PDCRE,
    pucrf: PUCRF,
    pdcrf: PDCRF,
    pucrg: PUCRG,
    pdcrg: PDCRG,
    pucrh: PUCRH,
    pdcrh: PDCRH,
    pucri: PUCRI,
    pdcri: PDCRI,
    _reserved24: [u8; 0x18],
    cr5: CR5,
}
impl RegisterBlock {
    ///0x00 - Power control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - Power control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - Power control register 3
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    ///0x0c - Power control register 4
    #[inline(always)]
    pub const fn cr4(&self) -> &CR4 {
        &self.cr4
    }
    ///0x10 - Power status register 1
    #[inline(always)]
    pub const fn sr1(&self) -> &SR1 {
        &self.sr1
    }
    ///0x14 - Power status register 2
    #[inline(always)]
    pub const fn sr2(&self) -> &SR2 {
        &self.sr2
    }
    ///0x18 - Power status clear register
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
    }
    ///0x20 - Power Port A pull-up control register
    #[inline(always)]
    pub const fn pucra(&self) -> &PUCRA {
        &self.pucra
    }
    ///0x24 - Power Port A pull-down control register
    #[inline(always)]
    pub const fn pdcra(&self) -> &PDCRA {
        &self.pdcra
    }
    ///0x28 - Power Port B pull-up control register
    #[inline(always)]
    pub const fn pucrb(&self) -> &PUCRB {
        &self.pucrb
    }
    ///0x2c - Power Port B pull-down control register
    #[inline(always)]
    pub const fn pdcrb(&self) -> &PDCRB {
        &self.pdcrb
    }
    ///0x30 - Power Port C pull-up control register
    #[inline(always)]
    pub const fn pucrc(&self) -> &PUCRC {
        &self.pucrc
    }
    ///0x34 - Power Port C pull-down control register
    #[inline(always)]
    pub const fn pdcrc(&self) -> &PDCRC {
        &self.pdcrc
    }
    ///0x38 - Power Port D pull-up control register
    #[inline(always)]
    pub const fn pucrd(&self) -> &PUCRD {
        &self.pucrd
    }
    ///0x3c - Power Port D pull-down control register
    #[inline(always)]
    pub const fn pdcrd(&self) -> &PDCRD {
        &self.pdcrd
    }
    ///0x44 - Power Port E pull-down control register
    #[inline(always)]
    pub const fn pdcre(&self) -> &PDCRE {
        &self.pdcre
    }
    ///0x48 - Power Port F pull-up control register
    #[inline(always)]
    pub const fn pucrf(&self) -> &PUCRF {
        &self.pucrf
    }
    ///0x4c - Power Port F pull-down control register
    #[inline(always)]
    pub const fn pdcrf(&self) -> &PDCRF {
        &self.pdcrf
    }
    ///0x50 - Power Port G pull-up control register
    #[inline(always)]
    pub const fn pucrg(&self) -> &PUCRG {
        &self.pucrg
    }
    ///0x54 - Power Port G pull-down control register
    #[inline(always)]
    pub const fn pdcrg(&self) -> &PDCRG {
        &self.pdcrg
    }
    ///0x58 - Power Port H pull-up control register
    #[inline(always)]
    pub const fn pucrh(&self) -> &PUCRH {
        &self.pucrh
    }
    ///0x5c - Power Port H pull-down control register
    #[inline(always)]
    pub const fn pdcrh(&self) -> &PDCRH {
        &self.pdcrh
    }
    ///0x60 - Power Port I pull-up control register
    #[inline(always)]
    pub const fn pucri(&self) -> &PUCRI {
        &self.pucri
    }
    ///0x64 - Power Port I pull-down control register
    #[inline(always)]
    pub const fn pdcri(&self) -> &PDCRI {
        &self.pdcri
    }
    ///0x80 - PWR control register
    #[inline(always)]
    pub const fn cr5(&self) -> &CR5 {
        &self.cr5
    }
}
/**CR1 (rw) register accessor: Power control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///Power control register 1
pub mod cr1;
/**CR2 (rw) register accessor: Power control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///Power control register 2
pub mod cr2;
/**CR3 (rw) register accessor: Power control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:CR3)

For information about available fields see [`mod@cr3`] module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///Power control register 3
pub mod cr3;
/**CR4 (rw) register accessor: Power control register 4

You can [`read`](crate::Reg::read) this register and get [`cr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:CR4)

For information about available fields see [`mod@cr4`] module*/
pub type CR4 = crate::Reg<cr4::CR4rs>;
///Power control register 4
pub mod cr4;
/**SR1 (r) register accessor: Power status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:SR1)

For information about available fields see [`mod@sr1`] module*/
pub type SR1 = crate::Reg<sr1::SR1rs>;
///Power status register 1
pub mod sr1;
/**SR2 (r) register accessor: Power status register 2

You can [`read`](crate::Reg::read) this register and get [`sr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:SR2)

For information about available fields see [`mod@sr2`] module*/
pub type SR2 = crate::Reg<sr2::SR2rs>;
///Power status register 2
pub mod sr2;
/**SCR (w) register accessor: Power status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:SCR)

For information about available fields see [`mod@scr`] module*/
pub type SCR = crate::Reg<scr::SCRrs>;
///Power status clear register
pub mod scr;
/**PUCRA (rw) register accessor: Power Port A pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PUCRA)

For information about available fields see [`mod@pucra`] module*/
pub type PUCRA = crate::Reg<pucra::PUCRArs>;
///Power Port A pull-up control register
pub mod pucra;
/**PDCRA (rw) register accessor: Power Port A pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PDCRA)

For information about available fields see [`mod@pdcra`] module*/
pub type PDCRA = crate::Reg<pdcra::PDCRArs>;
///Power Port A pull-down control register
pub mod pdcra;
/**PUCRB (rw) register accessor: Power Port B pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PUCRB)

For information about available fields see [`mod@pucrb`] module*/
pub type PUCRB = crate::Reg<pucrb::PUCRBrs>;
///Power Port B pull-up control register
pub mod pucrb;
/**PDCRB (rw) register accessor: Power Port B pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PDCRB)

For information about available fields see [`mod@pdcrb`] module*/
pub type PDCRB = crate::Reg<pdcrb::PDCRBrs>;
///Power Port B pull-down control register
pub mod pdcrb;
/**PUCRC (rw) register accessor: Power Port C pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PUCRC)

For information about available fields see [`mod@pucrc`] module*/
pub type PUCRC = crate::Reg<pucrc::PUCRCrs>;
///Power Port C pull-up control register
pub mod pucrc;
/**PDCRC (rw) register accessor: Power Port C pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PDCRC)

For information about available fields see [`mod@pdcrc`] module*/
pub type PDCRC = crate::Reg<pdcrc::PDCRCrs>;
///Power Port C pull-down control register
pub mod pdcrc;
/**PUCRD (rw) register accessor: Power Port D pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PUCRD)

For information about available fields see [`mod@pucrd`] module*/
pub type PUCRD = crate::Reg<pucrd::PUCRDrs>;
///Power Port D pull-up control register
pub mod pucrd;
/**PDCRD (rw) register accessor: Power Port D pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PDCRD)

For information about available fields see [`mod@pdcrd`] module*/
pub type PDCRD = crate::Reg<pdcrd::PDCRDrs>;
///Power Port D pull-down control register
pub mod pdcrd;
/**PDCRE (rw) register accessor: Power Port E pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PDCRE)

For information about available fields see [`mod@pdcre`] module*/
pub type PDCRE = crate::Reg<pdcre::PDCRErs>;
///Power Port E pull-down control register
pub mod pdcre;
/**PUCRF (rw) register accessor: Power Port F pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PUCRF)

For information about available fields see [`mod@pucrf`] module*/
pub type PUCRF = crate::Reg<pucrf::PUCRFrs>;
///Power Port F pull-up control register
pub mod pucrf;
/**PDCRF (rw) register accessor: Power Port F pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PDCRF)

For information about available fields see [`mod@pdcrf`] module*/
pub type PDCRF = crate::Reg<pdcrf::PDCRFrs>;
///Power Port F pull-down control register
pub mod pdcrf;
/**PUCRG (rw) register accessor: Power Port G pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PUCRG)

For information about available fields see [`mod@pucrg`] module*/
pub type PUCRG = crate::Reg<pucrg::PUCRGrs>;
///Power Port G pull-up control register
pub mod pucrg;
/**PDCRG (rw) register accessor: Power Port G pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PDCRG)

For information about available fields see [`mod@pdcrg`] module*/
pub type PDCRG = crate::Reg<pdcrg::PDCRGrs>;
///Power Port G pull-down control register
pub mod pdcrg;
/**PUCRH (rw) register accessor: Power Port H pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PUCRH)

For information about available fields see [`mod@pucrh`] module*/
pub type PUCRH = crate::Reg<pucrh::PUCRHrs>;
///Power Port H pull-up control register
pub mod pucrh;
/**PDCRH (rw) register accessor: Power Port H pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PDCRH)

For information about available fields see [`mod@pdcrh`] module*/
pub type PDCRH = crate::Reg<pdcrh::PDCRHrs>;
///Power Port H pull-down control register
pub mod pdcrh;
/**PUCRI (rw) register accessor: Power Port I pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucri::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucri::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PUCRI)

For information about available fields see [`mod@pucri`] module*/
pub type PUCRI = crate::Reg<pucri::PUCRIrs>;
///Power Port I pull-up control register
pub mod pucri;
/**PDCRI (rw) register accessor: Power Port I pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcri::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcri::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:PDCRI)

For information about available fields see [`mod@pdcri`] module*/
pub type PDCRI = crate::Reg<pdcri::PDCRIrs>;
///Power Port I pull-down control register
pub mod pdcri;
/**CR5 (rw) register accessor: PWR control register

You can [`read`](crate::Reg::read) this register and get [`cr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:CR5)

For information about available fields see [`mod@cr5`] module*/
pub type CR5 = crate::Reg<cr5::CR5rs>;
///PWR control register
pub mod cr5;
