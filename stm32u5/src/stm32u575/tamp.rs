#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
    fltcr: FLTCR,
    atcr1: ATCR1,
    atseedr: ATSEEDR,
    ator: ATOR,
    atcr2: ATCR2,
    seccfgr: SECCFGR,
    privcr: PRIVCR,
    _reserved10: [u8; 0x04],
    ier: IER,
    sr: SR,
    misr: MISR,
    smisr: SMISR,
    scr: SCR,
    count1r: COUNT1R,
    _reserved16: [u8; 0x10],
    ercfgr: ERCFGR,
    _reserved17: [u8; 0xa8],
    bkp0r: BKP0R,
    bkp1r: BKP1R,
    bkp2r: BKP2R,
    bkp3r: BKP3R,
    bkp4r: BKP4R,
    bkp5r: BKP5R,
    bkp6r: BKP6R,
    bkp7r: BKP7R,
    bkp8r: BKP8R,
    bkp9r: BKP9R,
    bkp10r: BKP10R,
    bkp11r: BKP11R,
    bkp12r: BKP12R,
    bkp13r: BKP13R,
    bkp14r: BKP14R,
    bkp15r: BKP15R,
    bkp16r: BKP16R,
    bkp17r: BKP17R,
    bkp18r: BKP18R,
    bkp19r: BKP19R,
    bkp20r: BKP20R,
    bkp21r: BKP21R,
    bkp22r: BKP22R,
    bkp23r: BKP23R,
    bkp24r: BKP24R,
    bkp25r: BKP25R,
    bkp26r: BKP26R,
    bkp27r: BKP27R,
    bkp28r: BKP28R,
    bkp29r: BKP29R,
    bkp30r: BKP30R,
    bkp31r: BKP31R,
}
impl RegisterBlock {
    ///0x00 - TAMP control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - TAMP control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - TAMP control register 3
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    ///0x0c - TAMP filter control register
    #[inline(always)]
    pub const fn fltcr(&self) -> &FLTCR {
        &self.fltcr
    }
    ///0x10 - TAMP active tamper control register 1
    #[inline(always)]
    pub const fn atcr1(&self) -> &ATCR1 {
        &self.atcr1
    }
    ///0x14 - TAMP active tamper seed register
    #[inline(always)]
    pub const fn atseedr(&self) -> &ATSEEDR {
        &self.atseedr
    }
    ///0x18 - TAMP active tamper output register
    #[inline(always)]
    pub const fn ator(&self) -> &ATOR {
        &self.ator
    }
    ///0x1c - TAMP active tamper control register 2
    #[inline(always)]
    pub const fn atcr2(&self) -> &ATCR2 {
        &self.atcr2
    }
    ///0x20 - TAMP secure mode register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x24 - TAMP privilege mode control register
    #[inline(always)]
    pub const fn privcr(&self) -> &PRIVCR {
        &self.privcr
    }
    ///0x2c - TAMP interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x30 - TAMP status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x34 - TAMP non-secure masked interrupt status register
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    ///0x38 - TAMP secure masked interrupt status register
    #[inline(always)]
    pub const fn smisr(&self) -> &SMISR {
        &self.smisr
    }
    ///0x3c - TAMP status clear register
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
    }
    ///0x40 - TAMP monotonic counter 1 register
    #[inline(always)]
    pub const fn count1r(&self) -> &COUNT1R {
        &self.count1r
    }
    ///0x54 - TAMP erase configuration register
    #[inline(always)]
    pub const fn ercfgr(&self) -> &ERCFGR {
        &self.ercfgr
    }
    ///0x100 - TAMP backup 0 register
    #[inline(always)]
    pub const fn bkp0r(&self) -> &BKP0R {
        &self.bkp0r
    }
    ///0x104 - TAMP backup 1 register
    #[inline(always)]
    pub const fn bkp1r(&self) -> &BKP1R {
        &self.bkp1r
    }
    ///0x108 - TAMP backup 2 register
    #[inline(always)]
    pub const fn bkp2r(&self) -> &BKP2R {
        &self.bkp2r
    }
    ///0x10c - TAMP backup 3 register
    #[inline(always)]
    pub const fn bkp3r(&self) -> &BKP3R {
        &self.bkp3r
    }
    ///0x110 - TAMP backup 4 register
    #[inline(always)]
    pub const fn bkp4r(&self) -> &BKP4R {
        &self.bkp4r
    }
    ///0x114 - TAMP backup 5 register
    #[inline(always)]
    pub const fn bkp5r(&self) -> &BKP5R {
        &self.bkp5r
    }
    ///0x118 - TAMP backup 6 register
    #[inline(always)]
    pub const fn bkp6r(&self) -> &BKP6R {
        &self.bkp6r
    }
    ///0x11c - TAMP backup 7 register
    #[inline(always)]
    pub const fn bkp7r(&self) -> &BKP7R {
        &self.bkp7r
    }
    ///0x120 - TAMP backup 8 register
    #[inline(always)]
    pub const fn bkp8r(&self) -> &BKP8R {
        &self.bkp8r
    }
    ///0x124 - TAMP backup 9 register
    #[inline(always)]
    pub const fn bkp9r(&self) -> &BKP9R {
        &self.bkp9r
    }
    ///0x128 - TAMP backup 10 register
    #[inline(always)]
    pub const fn bkp10r(&self) -> &BKP10R {
        &self.bkp10r
    }
    ///0x12c - TAMP backup 11 register
    #[inline(always)]
    pub const fn bkp11r(&self) -> &BKP11R {
        &self.bkp11r
    }
    ///0x130 - TAMP backup 12 register
    #[inline(always)]
    pub const fn bkp12r(&self) -> &BKP12R {
        &self.bkp12r
    }
    ///0x134 - TAMP backup 13 register
    #[inline(always)]
    pub const fn bkp13r(&self) -> &BKP13R {
        &self.bkp13r
    }
    ///0x138 - TAMP backup 14 register
    #[inline(always)]
    pub const fn bkp14r(&self) -> &BKP14R {
        &self.bkp14r
    }
    ///0x13c - TAMP backup 15 register
    #[inline(always)]
    pub const fn bkp15r(&self) -> &BKP15R {
        &self.bkp15r
    }
    ///0x140 - TAMP backup 16 register
    #[inline(always)]
    pub const fn bkp16r(&self) -> &BKP16R {
        &self.bkp16r
    }
    ///0x144 - TAMP backup 17 register
    #[inline(always)]
    pub const fn bkp17r(&self) -> &BKP17R {
        &self.bkp17r
    }
    ///0x148 - TAMP backup 18 register
    #[inline(always)]
    pub const fn bkp18r(&self) -> &BKP18R {
        &self.bkp18r
    }
    ///0x14c - TAMP backup 19 register
    #[inline(always)]
    pub const fn bkp19r(&self) -> &BKP19R {
        &self.bkp19r
    }
    ///0x150 - TAMP backup 20 register
    #[inline(always)]
    pub const fn bkp20r(&self) -> &BKP20R {
        &self.bkp20r
    }
    ///0x154 - TAMP backup 21 register
    #[inline(always)]
    pub const fn bkp21r(&self) -> &BKP21R {
        &self.bkp21r
    }
    ///0x158 - TAMP backup 22 register
    #[inline(always)]
    pub const fn bkp22r(&self) -> &BKP22R {
        &self.bkp22r
    }
    ///0x15c - TAMP backup 23 register
    #[inline(always)]
    pub const fn bkp23r(&self) -> &BKP23R {
        &self.bkp23r
    }
    ///0x160 - TAMP backup 24 register
    #[inline(always)]
    pub const fn bkp24r(&self) -> &BKP24R {
        &self.bkp24r
    }
    ///0x164 - TAMP backup 25 register
    #[inline(always)]
    pub const fn bkp25r(&self) -> &BKP25R {
        &self.bkp25r
    }
    ///0x168 - TAMP backup 26 register
    #[inline(always)]
    pub const fn bkp26r(&self) -> &BKP26R {
        &self.bkp26r
    }
    ///0x16c - TAMP backup 27 register
    #[inline(always)]
    pub const fn bkp27r(&self) -> &BKP27R {
        &self.bkp27r
    }
    ///0x170 - TAMP backup 28 register
    #[inline(always)]
    pub const fn bkp28r(&self) -> &BKP28R {
        &self.bkp28r
    }
    ///0x174 - TAMP backup 29 register
    #[inline(always)]
    pub const fn bkp29r(&self) -> &BKP29R {
        &self.bkp29r
    }
    ///0x178 - TAMP backup 30 register
    #[inline(always)]
    pub const fn bkp30r(&self) -> &BKP30R {
        &self.bkp30r
    }
    ///0x17c - TAMP backup 31 register
    #[inline(always)]
    pub const fn bkp31r(&self) -> &BKP31R {
        &self.bkp31r
    }
}
/**CR1 (rw) register accessor: TAMP control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///TAMP control register 1
pub mod cr1;
/**CR2 (rw) register accessor: TAMP control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///TAMP control register 2
pub mod cr2;
/**CR3 (rw) register accessor: TAMP control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:CR3)

For information about available fields see [`mod@cr3`] module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///TAMP control register 3
pub mod cr3;
/**FLTCR (rw) register accessor: TAMP filter control register

You can [`read`](crate::Reg::read) this register and get [`fltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:FLTCR)

For information about available fields see [`mod@fltcr`] module*/
pub type FLTCR = crate::Reg<fltcr::FLTCRrs>;
///TAMP filter control register
pub mod fltcr;
/**ATCR1 (rw) register accessor: TAMP active tamper control register 1

You can [`read`](crate::Reg::read) this register and get [`atcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:ATCR1)

For information about available fields see [`mod@atcr1`] module*/
pub type ATCR1 = crate::Reg<atcr1::ATCR1rs>;
///TAMP active tamper control register 1
pub mod atcr1;
/**ATSEEDR (w) register accessor: TAMP active tamper seed register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atseedr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:ATSEEDR)

For information about available fields see [`mod@atseedr`] module*/
pub type ATSEEDR = crate::Reg<atseedr::ATSEEDRrs>;
///TAMP active tamper seed register
pub mod atseedr;
/**ATOR (r) register accessor: TAMP active tamper output register

You can [`read`](crate::Reg::read) this register and get [`ator::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:ATOR)

For information about available fields see [`mod@ator`] module*/
pub type ATOR = crate::Reg<ator::ATORrs>;
///TAMP active tamper output register
pub mod ator;
/**ATCR2 (rw) register accessor: TAMP active tamper control register 2

You can [`read`](crate::Reg::read) this register and get [`atcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:ATCR2)

For information about available fields see [`mod@atcr2`] module*/
pub type ATCR2 = crate::Reg<atcr2::ATCR2rs>;
///TAMP active tamper control register 2
pub mod atcr2;
/**SECCFGR (rw) register accessor: TAMP secure mode register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///TAMP secure mode register
pub mod seccfgr;
/**PRIVCR (rw) register accessor: TAMP privilege mode control register

You can [`read`](crate::Reg::read) this register and get [`privcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:PRIVCR)

For information about available fields see [`mod@privcr`] module*/
pub type PRIVCR = crate::Reg<privcr::PRIVCRrs>;
///TAMP privilege mode control register
pub mod privcr;
/**IER (rw) register accessor: TAMP interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///TAMP interrupt enable register
pub mod ier;
/**SR (r) register accessor: TAMP status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///TAMP status register
pub mod sr;
/**MISR (r) register accessor: TAMP non-secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:MISR)

For information about available fields see [`mod@misr`] module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///TAMP non-secure masked interrupt status register
pub mod misr;
/**SMISR (r) register accessor: TAMP secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`smisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:SMISR)

For information about available fields see [`mod@smisr`] module*/
pub type SMISR = crate::Reg<smisr::SMISRrs>;
///TAMP secure masked interrupt status register
pub mod smisr;
/**SCR (w) register accessor: TAMP status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:SCR)

For information about available fields see [`mod@scr`] module*/
pub type SCR = crate::Reg<scr::SCRrs>;
///TAMP status clear register
pub mod scr;
/**COUNT1R (r) register accessor: TAMP monotonic counter 1 register

You can [`read`](crate::Reg::read) this register and get [`count1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:COUNT1R)

For information about available fields see [`mod@count1r`] module*/
pub type COUNT1R = crate::Reg<count1r::COUNT1Rrs>;
///TAMP monotonic counter 1 register
pub mod count1r;
/**ERCFGR (rw) register accessor: TAMP erase configuration register

You can [`read`](crate::Reg::read) this register and get [`ercfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ercfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:ERCFGR)

For information about available fields see [`mod@ercfgr`] module*/
pub type ERCFGR = crate::Reg<ercfgr::ERCFGRrs>;
///TAMP erase configuration register
pub mod ercfgr;
/**BKP0R (rw) register accessor: TAMP backup 0 register

You can [`read`](crate::Reg::read) this register and get [`bkp0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP0R)

For information about available fields see [`mod@bkp0r`] module*/
pub type BKP0R = crate::Reg<bkp0r::BKP0Rrs>;
///TAMP backup 0 register
pub mod bkp0r;
/**BKP1R (rw) register accessor: TAMP backup 1 register

You can [`read`](crate::Reg::read) this register and get [`bkp1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP1R)

For information about available fields see [`mod@bkp1r`] module*/
pub type BKP1R = crate::Reg<bkp1r::BKP1Rrs>;
///TAMP backup 1 register
pub mod bkp1r;
/**BKP2R (rw) register accessor: TAMP backup 2 register

You can [`read`](crate::Reg::read) this register and get [`bkp2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP2R)

For information about available fields see [`mod@bkp2r`] module*/
pub type BKP2R = crate::Reg<bkp2r::BKP2Rrs>;
///TAMP backup 2 register
pub mod bkp2r;
/**BKP3R (rw) register accessor: TAMP backup 3 register

You can [`read`](crate::Reg::read) this register and get [`bkp3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP3R)

For information about available fields see [`mod@bkp3r`] module*/
pub type BKP3R = crate::Reg<bkp3r::BKP3Rrs>;
///TAMP backup 3 register
pub mod bkp3r;
/**BKP4R (rw) register accessor: TAMP backup 4 register

You can [`read`](crate::Reg::read) this register and get [`bkp4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP4R)

For information about available fields see [`mod@bkp4r`] module*/
pub type BKP4R = crate::Reg<bkp4r::BKP4Rrs>;
///TAMP backup 4 register
pub mod bkp4r;
/**BKP5R (rw) register accessor: TAMP backup 5 register

You can [`read`](crate::Reg::read) this register and get [`bkp5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP5R)

For information about available fields see [`mod@bkp5r`] module*/
pub type BKP5R = crate::Reg<bkp5r::BKP5Rrs>;
///TAMP backup 5 register
pub mod bkp5r;
/**BKP6R (rw) register accessor: TAMP backup 6 register

You can [`read`](crate::Reg::read) this register and get [`bkp6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP6R)

For information about available fields see [`mod@bkp6r`] module*/
pub type BKP6R = crate::Reg<bkp6r::BKP6Rrs>;
///TAMP backup 6 register
pub mod bkp6r;
/**BKP7R (rw) register accessor: TAMP backup 7 register

You can [`read`](crate::Reg::read) this register and get [`bkp7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP7R)

For information about available fields see [`mod@bkp7r`] module*/
pub type BKP7R = crate::Reg<bkp7r::BKP7Rrs>;
///TAMP backup 7 register
pub mod bkp7r;
/**BKP8R (rw) register accessor: TAMP backup 8 register

You can [`read`](crate::Reg::read) this register and get [`bkp8r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp8r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP8R)

For information about available fields see [`mod@bkp8r`] module*/
pub type BKP8R = crate::Reg<bkp8r::BKP8Rrs>;
///TAMP backup 8 register
pub mod bkp8r;
/**BKP9R (rw) register accessor: TAMP backup 9 register

You can [`read`](crate::Reg::read) this register and get [`bkp9r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp9r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP9R)

For information about available fields see [`mod@bkp9r`] module*/
pub type BKP9R = crate::Reg<bkp9r::BKP9Rrs>;
///TAMP backup 9 register
pub mod bkp9r;
/**BKP10R (rw) register accessor: TAMP backup 10 register

You can [`read`](crate::Reg::read) this register and get [`bkp10r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp10r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP10R)

For information about available fields see [`mod@bkp10r`] module*/
pub type BKP10R = crate::Reg<bkp10r::BKP10Rrs>;
///TAMP backup 10 register
pub mod bkp10r;
/**BKP11R (rw) register accessor: TAMP backup 11 register

You can [`read`](crate::Reg::read) this register and get [`bkp11r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp11r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP11R)

For information about available fields see [`mod@bkp11r`] module*/
pub type BKP11R = crate::Reg<bkp11r::BKP11Rrs>;
///TAMP backup 11 register
pub mod bkp11r;
/**BKP12R (rw) register accessor: TAMP backup 12 register

You can [`read`](crate::Reg::read) this register and get [`bkp12r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp12r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP12R)

For information about available fields see [`mod@bkp12r`] module*/
pub type BKP12R = crate::Reg<bkp12r::BKP12Rrs>;
///TAMP backup 12 register
pub mod bkp12r;
/**BKP13R (rw) register accessor: TAMP backup 13 register

You can [`read`](crate::Reg::read) this register and get [`bkp13r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp13r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP13R)

For information about available fields see [`mod@bkp13r`] module*/
pub type BKP13R = crate::Reg<bkp13r::BKP13Rrs>;
///TAMP backup 13 register
pub mod bkp13r;
/**BKP14R (rw) register accessor: TAMP backup 14 register

You can [`read`](crate::Reg::read) this register and get [`bkp14r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp14r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP14R)

For information about available fields see [`mod@bkp14r`] module*/
pub type BKP14R = crate::Reg<bkp14r::BKP14Rrs>;
///TAMP backup 14 register
pub mod bkp14r;
/**BKP15R (rw) register accessor: TAMP backup 15 register

You can [`read`](crate::Reg::read) this register and get [`bkp15r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp15r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP15R)

For information about available fields see [`mod@bkp15r`] module*/
pub type BKP15R = crate::Reg<bkp15r::BKP15Rrs>;
///TAMP backup 15 register
pub mod bkp15r;
/**BKP16R (rw) register accessor: TAMP backup 16 register

You can [`read`](crate::Reg::read) this register and get [`bkp16r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp16r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP16R)

For information about available fields see [`mod@bkp16r`] module*/
pub type BKP16R = crate::Reg<bkp16r::BKP16Rrs>;
///TAMP backup 16 register
pub mod bkp16r;
/**BKP17R (rw) register accessor: TAMP backup 17 register

You can [`read`](crate::Reg::read) this register and get [`bkp17r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp17r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP17R)

For information about available fields see [`mod@bkp17r`] module*/
pub type BKP17R = crate::Reg<bkp17r::BKP17Rrs>;
///TAMP backup 17 register
pub mod bkp17r;
/**BKP18R (rw) register accessor: TAMP backup 18 register

You can [`read`](crate::Reg::read) this register and get [`bkp18r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp18r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP18R)

For information about available fields see [`mod@bkp18r`] module*/
pub type BKP18R = crate::Reg<bkp18r::BKP18Rrs>;
///TAMP backup 18 register
pub mod bkp18r;
/**BKP19R (rw) register accessor: TAMP backup 19 register

You can [`read`](crate::Reg::read) this register and get [`bkp19r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp19r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP19R)

For information about available fields see [`mod@bkp19r`] module*/
pub type BKP19R = crate::Reg<bkp19r::BKP19Rrs>;
///TAMP backup 19 register
pub mod bkp19r;
/**BKP20R (rw) register accessor: TAMP backup 20 register

You can [`read`](crate::Reg::read) this register and get [`bkp20r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp20r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP20R)

For information about available fields see [`mod@bkp20r`] module*/
pub type BKP20R = crate::Reg<bkp20r::BKP20Rrs>;
///TAMP backup 20 register
pub mod bkp20r;
/**BKP21R (rw) register accessor: TAMP backup 21 register

You can [`read`](crate::Reg::read) this register and get [`bkp21r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp21r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP21R)

For information about available fields see [`mod@bkp21r`] module*/
pub type BKP21R = crate::Reg<bkp21r::BKP21Rrs>;
///TAMP backup 21 register
pub mod bkp21r;
/**BKP22R (rw) register accessor: TAMP backup 22 register

You can [`read`](crate::Reg::read) this register and get [`bkp22r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp22r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP22R)

For information about available fields see [`mod@bkp22r`] module*/
pub type BKP22R = crate::Reg<bkp22r::BKP22Rrs>;
///TAMP backup 22 register
pub mod bkp22r;
/**BKP23R (rw) register accessor: TAMP backup 23 register

You can [`read`](crate::Reg::read) this register and get [`bkp23r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp23r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP23R)

For information about available fields see [`mod@bkp23r`] module*/
pub type BKP23R = crate::Reg<bkp23r::BKP23Rrs>;
///TAMP backup 23 register
pub mod bkp23r;
/**BKP24R (rw) register accessor: TAMP backup 24 register

You can [`read`](crate::Reg::read) this register and get [`bkp24r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp24r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP24R)

For information about available fields see [`mod@bkp24r`] module*/
pub type BKP24R = crate::Reg<bkp24r::BKP24Rrs>;
///TAMP backup 24 register
pub mod bkp24r;
/**BKP25R (rw) register accessor: TAMP backup 25 register

You can [`read`](crate::Reg::read) this register and get [`bkp25r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp25r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP25R)

For information about available fields see [`mod@bkp25r`] module*/
pub type BKP25R = crate::Reg<bkp25r::BKP25Rrs>;
///TAMP backup 25 register
pub mod bkp25r;
/**BKP26R (rw) register accessor: TAMP backup 26 register

You can [`read`](crate::Reg::read) this register and get [`bkp26r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp26r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP26R)

For information about available fields see [`mod@bkp26r`] module*/
pub type BKP26R = crate::Reg<bkp26r::BKP26Rrs>;
///TAMP backup 26 register
pub mod bkp26r;
/**BKP27R (rw) register accessor: TAMP backup 27 register

You can [`read`](crate::Reg::read) this register and get [`bkp27r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp27r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP27R)

For information about available fields see [`mod@bkp27r`] module*/
pub type BKP27R = crate::Reg<bkp27r::BKP27Rrs>;
///TAMP backup 27 register
pub mod bkp27r;
/**BKP28R (rw) register accessor: TAMP backup 28 register

You can [`read`](crate::Reg::read) this register and get [`bkp28r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp28r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP28R)

For information about available fields see [`mod@bkp28r`] module*/
pub type BKP28R = crate::Reg<bkp28r::BKP28Rrs>;
///TAMP backup 28 register
pub mod bkp28r;
/**BKP29R (rw) register accessor: TAMP backup 29 register

You can [`read`](crate::Reg::read) this register and get [`bkp29r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp29r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP29R)

For information about available fields see [`mod@bkp29r`] module*/
pub type BKP29R = crate::Reg<bkp29r::BKP29Rrs>;
///TAMP backup 29 register
pub mod bkp29r;
/**BKP30R (rw) register accessor: TAMP backup 30 register

You can [`read`](crate::Reg::read) this register and get [`bkp30r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp30r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP30R)

For information about available fields see [`mod@bkp30r`] module*/
pub type BKP30R = crate::Reg<bkp30r::BKP30Rrs>;
///TAMP backup 30 register
pub mod bkp30r;
/**BKP31R (rw) register accessor: TAMP backup 31 register

You can [`read`](crate::Reg::read) this register and get [`bkp31r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp31r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:BKP31R)

For information about available fields see [`mod@bkp31r`] module*/
pub type BKP31R = crate::Reg<bkp31r::BKP31Rrs>;
///TAMP backup 31 register
pub mod bkp31r;
