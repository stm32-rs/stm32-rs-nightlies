#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tamp_cr1: TAMP_CR1,
    tamp_cr2: TAMP_CR2,
    tamp_cr3: TAMP_CR3,
    tamp_fltcr: TAMP_FLTCR,
    tamp_atcr1: TAMP_ATCR1,
    tamp_atseedr: TAMP_ATSEEDR,
    tamp_ator: TAMP_ATOR,
    tamp_atcr2: TAMP_ATCR2,
    tamp_seccfgr: TAMP_SECCFGR,
    tamp_privcr: TAMP_PRIVCR,
    _reserved10: [u8; 0x04],
    tamp_ier: TAMP_IER,
    tamp_sr: TAMP_SR,
    tamp_misr: TAMP_MISR,
    tamp_smisr: TAMP_SMISR,
    tamp_scr: TAMP_SCR,
    tamp_count1r: TAMP_COUNT1R,
    _reserved16: [u8; 0x10],
    tamp_ercfgr: TAMP_ERCFGR,
    _reserved17: [u8; 0xa8],
    tamp_bkp0r: TAMP_BKP0R,
    tamp_bkp1r: TAMP_BKP1R,
    tamp_bkp2r: TAMP_BKP2R,
    tamp_bkp3r: TAMP_BKP3R,
    tamp_bkp4r: TAMP_BKP4R,
    tamp_bkp5r: TAMP_BKP5R,
    tamp_bkp6r: TAMP_BKP6R,
    tamp_bkp7r: TAMP_BKP7R,
    tamp_bkp8r: TAMP_BKP8R,
    tamp_bkp9r: TAMP_BKP9R,
    tamp_bkp10r: TAMP_BKP10R,
    tamp_bkp11r: TAMP_BKP11R,
    tamp_bkp12r: TAMP_BKP12R,
    tamp_bkp13r: TAMP_BKP13R,
    tamp_bkp14r: TAMP_BKP14R,
    tamp_bkp15r: TAMP_BKP15R,
    tamp_bkp16r: TAMP_BKP16R,
    tamp_bkp17r: TAMP_BKP17R,
    tamp_bkp18r: TAMP_BKP18R,
    tamp_bkp19r: TAMP_BKP19R,
    tamp_bkp20r: TAMP_BKP20R,
    tamp_bkp21r: TAMP_BKP21R,
    tamp_bkp22r: TAMP_BKP22R,
    tamp_bkp23r: TAMP_BKP23R,
    tamp_bkp24r: TAMP_BKP24R,
    tamp_bkp25r: TAMP_BKP25R,
    tamp_bkp26r: TAMP_BKP26R,
    tamp_bkp27r: TAMP_BKP27R,
    tamp_bkp28r: TAMP_BKP28R,
    tamp_bkp29r: TAMP_BKP29R,
    tamp_bkp30r: TAMP_BKP30R,
    tamp_bkp31r: TAMP_BKP31R,
}
impl RegisterBlock {
    ///0x00 - TAMP control register 1
    #[inline(always)]
    pub const fn tamp_cr1(&self) -> &TAMP_CR1 {
        &self.tamp_cr1
    }
    ///0x04 - TAMP control register 2
    #[inline(always)]
    pub const fn tamp_cr2(&self) -> &TAMP_CR2 {
        &self.tamp_cr2
    }
    ///0x08 - TAMP control register 3
    #[inline(always)]
    pub const fn tamp_cr3(&self) -> &TAMP_CR3 {
        &self.tamp_cr3
    }
    ///0x0c - TAMP filter control register
    #[inline(always)]
    pub const fn tamp_fltcr(&self) -> &TAMP_FLTCR {
        &self.tamp_fltcr
    }
    ///0x10 - TAMP active tamper control register 1
    #[inline(always)]
    pub const fn tamp_atcr1(&self) -> &TAMP_ATCR1 {
        &self.tamp_atcr1
    }
    ///0x14 - TAMP active tamper seed register
    #[inline(always)]
    pub const fn tamp_atseedr(&self) -> &TAMP_ATSEEDR {
        &self.tamp_atseedr
    }
    ///0x18 - TAMP active tamper output register
    #[inline(always)]
    pub const fn tamp_ator(&self) -> &TAMP_ATOR {
        &self.tamp_ator
    }
    ///0x1c - TAMP active tamper control register 2
    #[inline(always)]
    pub const fn tamp_atcr2(&self) -> &TAMP_ATCR2 {
        &self.tamp_atcr2
    }
    ///0x20 - TAMP secure mode register
    #[inline(always)]
    pub const fn tamp_seccfgr(&self) -> &TAMP_SECCFGR {
        &self.tamp_seccfgr
    }
    ///0x24 - TAMP privilege mode control register
    #[inline(always)]
    pub const fn tamp_privcr(&self) -> &TAMP_PRIVCR {
        &self.tamp_privcr
    }
    ///0x2c - TAMP interrupt enable register
    #[inline(always)]
    pub const fn tamp_ier(&self) -> &TAMP_IER {
        &self.tamp_ier
    }
    ///0x30 - TAMP status register
    #[inline(always)]
    pub const fn tamp_sr(&self) -> &TAMP_SR {
        &self.tamp_sr
    }
    ///0x34 - TAMP non-secure masked interrupt status register
    #[inline(always)]
    pub const fn tamp_misr(&self) -> &TAMP_MISR {
        &self.tamp_misr
    }
    ///0x38 - TAMP secure masked interrupt status register
    #[inline(always)]
    pub const fn tamp_smisr(&self) -> &TAMP_SMISR {
        &self.tamp_smisr
    }
    ///0x3c - TAMP status clear register
    #[inline(always)]
    pub const fn tamp_scr(&self) -> &TAMP_SCR {
        &self.tamp_scr
    }
    ///0x40 - TAMP monotonic counter 1 register
    #[inline(always)]
    pub const fn tamp_count1r(&self) -> &TAMP_COUNT1R {
        &self.tamp_count1r
    }
    ///0x54 - TAMP erase configuration register
    #[inline(always)]
    pub const fn tamp_ercfgr(&self) -> &TAMP_ERCFGR {
        &self.tamp_ercfgr
    }
    ///0x100 - TAMP backup 0 register
    #[inline(always)]
    pub const fn tamp_bkp0r(&self) -> &TAMP_BKP0R {
        &self.tamp_bkp0r
    }
    ///0x104 - TAMP backup 1 register
    #[inline(always)]
    pub const fn tamp_bkp1r(&self) -> &TAMP_BKP1R {
        &self.tamp_bkp1r
    }
    ///0x108 - TAMP backup 2 register
    #[inline(always)]
    pub const fn tamp_bkp2r(&self) -> &TAMP_BKP2R {
        &self.tamp_bkp2r
    }
    ///0x10c - TAMP backup 3 register
    #[inline(always)]
    pub const fn tamp_bkp3r(&self) -> &TAMP_BKP3R {
        &self.tamp_bkp3r
    }
    ///0x110 - TAMP backup 4 register
    #[inline(always)]
    pub const fn tamp_bkp4r(&self) -> &TAMP_BKP4R {
        &self.tamp_bkp4r
    }
    ///0x114 - TAMP backup 5 register
    #[inline(always)]
    pub const fn tamp_bkp5r(&self) -> &TAMP_BKP5R {
        &self.tamp_bkp5r
    }
    ///0x118 - TAMP backup 6 register
    #[inline(always)]
    pub const fn tamp_bkp6r(&self) -> &TAMP_BKP6R {
        &self.tamp_bkp6r
    }
    ///0x11c - TAMP backup 7 register
    #[inline(always)]
    pub const fn tamp_bkp7r(&self) -> &TAMP_BKP7R {
        &self.tamp_bkp7r
    }
    ///0x120 - TAMP backup 8 register
    #[inline(always)]
    pub const fn tamp_bkp8r(&self) -> &TAMP_BKP8R {
        &self.tamp_bkp8r
    }
    ///0x124 - TAMP backup 9 register
    #[inline(always)]
    pub const fn tamp_bkp9r(&self) -> &TAMP_BKP9R {
        &self.tamp_bkp9r
    }
    ///0x128 - TAMP backup 10 register
    #[inline(always)]
    pub const fn tamp_bkp10r(&self) -> &TAMP_BKP10R {
        &self.tamp_bkp10r
    }
    ///0x12c - TAMP backup 11 register
    #[inline(always)]
    pub const fn tamp_bkp11r(&self) -> &TAMP_BKP11R {
        &self.tamp_bkp11r
    }
    ///0x130 - TAMP backup 12 register
    #[inline(always)]
    pub const fn tamp_bkp12r(&self) -> &TAMP_BKP12R {
        &self.tamp_bkp12r
    }
    ///0x134 - TAMP backup 13 register
    #[inline(always)]
    pub const fn tamp_bkp13r(&self) -> &TAMP_BKP13R {
        &self.tamp_bkp13r
    }
    ///0x138 - TAMP backup 14 register
    #[inline(always)]
    pub const fn tamp_bkp14r(&self) -> &TAMP_BKP14R {
        &self.tamp_bkp14r
    }
    ///0x13c - TAMP backup 15 register
    #[inline(always)]
    pub const fn tamp_bkp15r(&self) -> &TAMP_BKP15R {
        &self.tamp_bkp15r
    }
    ///0x140 - TAMP backup 16 register
    #[inline(always)]
    pub const fn tamp_bkp16r(&self) -> &TAMP_BKP16R {
        &self.tamp_bkp16r
    }
    ///0x144 - TAMP backup 17 register
    #[inline(always)]
    pub const fn tamp_bkp17r(&self) -> &TAMP_BKP17R {
        &self.tamp_bkp17r
    }
    ///0x148 - TAMP backup 18 register
    #[inline(always)]
    pub const fn tamp_bkp18r(&self) -> &TAMP_BKP18R {
        &self.tamp_bkp18r
    }
    ///0x14c - TAMP backup 19 register
    #[inline(always)]
    pub const fn tamp_bkp19r(&self) -> &TAMP_BKP19R {
        &self.tamp_bkp19r
    }
    ///0x150 - TAMP backup 20 register
    #[inline(always)]
    pub const fn tamp_bkp20r(&self) -> &TAMP_BKP20R {
        &self.tamp_bkp20r
    }
    ///0x154 - TAMP backup 21 register
    #[inline(always)]
    pub const fn tamp_bkp21r(&self) -> &TAMP_BKP21R {
        &self.tamp_bkp21r
    }
    ///0x158 - TAMP backup 22 register
    #[inline(always)]
    pub const fn tamp_bkp22r(&self) -> &TAMP_BKP22R {
        &self.tamp_bkp22r
    }
    ///0x15c - TAMP backup 23 register
    #[inline(always)]
    pub const fn tamp_bkp23r(&self) -> &TAMP_BKP23R {
        &self.tamp_bkp23r
    }
    ///0x160 - TAMP backup 24 register
    #[inline(always)]
    pub const fn tamp_bkp24r(&self) -> &TAMP_BKP24R {
        &self.tamp_bkp24r
    }
    ///0x164 - TAMP backup 25 register
    #[inline(always)]
    pub const fn tamp_bkp25r(&self) -> &TAMP_BKP25R {
        &self.tamp_bkp25r
    }
    ///0x168 - TAMP backup 26 register
    #[inline(always)]
    pub const fn tamp_bkp26r(&self) -> &TAMP_BKP26R {
        &self.tamp_bkp26r
    }
    ///0x16c - TAMP backup 27 register
    #[inline(always)]
    pub const fn tamp_bkp27r(&self) -> &TAMP_BKP27R {
        &self.tamp_bkp27r
    }
    ///0x170 - TAMP backup 28 register
    #[inline(always)]
    pub const fn tamp_bkp28r(&self) -> &TAMP_BKP28R {
        &self.tamp_bkp28r
    }
    ///0x174 - TAMP backup 29 register
    #[inline(always)]
    pub const fn tamp_bkp29r(&self) -> &TAMP_BKP29R {
        &self.tamp_bkp29r
    }
    ///0x178 - TAMP backup 30 register
    #[inline(always)]
    pub const fn tamp_bkp30r(&self) -> &TAMP_BKP30R {
        &self.tamp_bkp30r
    }
    ///0x17c - TAMP backup 31 register
    #[inline(always)]
    pub const fn tamp_bkp31r(&self) -> &TAMP_BKP31R {
        &self.tamp_bkp31r
    }
}
/**TAMP_CR1 (rw) register accessor: TAMP control register 1

You can [`read`](crate::Reg::read) this register and get [`tamp_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_CR1)

For information about available fields see [`mod@tamp_cr1`]
module*/
pub type TAMP_CR1 = crate::Reg<tamp_cr1::TAMP_CR1rs>;
///TAMP control register 1
pub mod tamp_cr1;
/**TAMP_CR2 (rw) register accessor: TAMP control register 2

You can [`read`](crate::Reg::read) this register and get [`tamp_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_CR2)

For information about available fields see [`mod@tamp_cr2`]
module*/
pub type TAMP_CR2 = crate::Reg<tamp_cr2::TAMP_CR2rs>;
///TAMP control register 2
pub mod tamp_cr2;
/**TAMP_CR3 (rw) register accessor: TAMP control register 3

You can [`read`](crate::Reg::read) this register and get [`tamp_cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_CR3)

For information about available fields see [`mod@tamp_cr3`]
module*/
pub type TAMP_CR3 = crate::Reg<tamp_cr3::TAMP_CR3rs>;
///TAMP control register 3
pub mod tamp_cr3;
/**TAMP_FLTCR (rw) register accessor: TAMP filter control register

You can [`read`](crate::Reg::read) this register and get [`tamp_fltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_fltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_FLTCR)

For information about available fields see [`mod@tamp_fltcr`]
module*/
pub type TAMP_FLTCR = crate::Reg<tamp_fltcr::TAMP_FLTCRrs>;
///TAMP filter control register
pub mod tamp_fltcr;
/**TAMP_ATCR1 (rw) register accessor: TAMP active tamper control register 1

You can [`read`](crate::Reg::read) this register and get [`tamp_atcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_atcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_ATCR1)

For information about available fields see [`mod@tamp_atcr1`]
module*/
pub type TAMP_ATCR1 = crate::Reg<tamp_atcr1::TAMP_ATCR1rs>;
///TAMP active tamper control register 1
pub mod tamp_atcr1;
/**TAMP_ATSEEDR (w) register accessor: TAMP active tamper seed register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_atseedr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_ATSEEDR)

For information about available fields see [`mod@tamp_atseedr`]
module*/
pub type TAMP_ATSEEDR = crate::Reg<tamp_atseedr::TAMP_ATSEEDRrs>;
///TAMP active tamper seed register
pub mod tamp_atseedr;
/**TAMP_ATOR (r) register accessor: TAMP active tamper output register

You can [`read`](crate::Reg::read) this register and get [`tamp_ator::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_ATOR)

For information about available fields see [`mod@tamp_ator`]
module*/
pub type TAMP_ATOR = crate::Reg<tamp_ator::TAMP_ATORrs>;
///TAMP active tamper output register
pub mod tamp_ator;
/**TAMP_ATCR2 (rw) register accessor: TAMP active tamper control register 2

You can [`read`](crate::Reg::read) this register and get [`tamp_atcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_atcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_ATCR2)

For information about available fields see [`mod@tamp_atcr2`]
module*/
pub type TAMP_ATCR2 = crate::Reg<tamp_atcr2::TAMP_ATCR2rs>;
///TAMP active tamper control register 2
pub mod tamp_atcr2;
/**TAMP_SECCFGR (rw) register accessor: TAMP secure mode register

You can [`read`](crate::Reg::read) this register and get [`tamp_seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_SECCFGR)

For information about available fields see [`mod@tamp_seccfgr`]
module*/
pub type TAMP_SECCFGR = crate::Reg<tamp_seccfgr::TAMP_SECCFGRrs>;
///TAMP secure mode register
pub mod tamp_seccfgr;
/**TAMP_PRIVCR (rw) register accessor: TAMP privilege mode control register

You can [`read`](crate::Reg::read) this register and get [`tamp_privcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_privcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_PRIVCR)

For information about available fields see [`mod@tamp_privcr`]
module*/
pub type TAMP_PRIVCR = crate::Reg<tamp_privcr::TAMP_PRIVCRrs>;
///TAMP privilege mode control register
pub mod tamp_privcr;
/**TAMP_IER (rw) register accessor: TAMP interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tamp_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_IER)

For information about available fields see [`mod@tamp_ier`]
module*/
pub type TAMP_IER = crate::Reg<tamp_ier::TAMP_IERrs>;
///TAMP interrupt enable register
pub mod tamp_ier;
/**TAMP_SR (r) register accessor: TAMP status register

You can [`read`](crate::Reg::read) this register and get [`tamp_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_SR)

For information about available fields see [`mod@tamp_sr`]
module*/
pub type TAMP_SR = crate::Reg<tamp_sr::TAMP_SRrs>;
///TAMP status register
pub mod tamp_sr;
/**TAMP_MISR (r) register accessor: TAMP non-secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`tamp_misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_MISR)

For information about available fields see [`mod@tamp_misr`]
module*/
pub type TAMP_MISR = crate::Reg<tamp_misr::TAMP_MISRrs>;
///TAMP non-secure masked interrupt status register
pub mod tamp_misr;
/**TAMP_SMISR (r) register accessor: TAMP secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`tamp_smisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_SMISR)

For information about available fields see [`mod@tamp_smisr`]
module*/
pub type TAMP_SMISR = crate::Reg<tamp_smisr::TAMP_SMISRrs>;
///TAMP secure masked interrupt status register
pub mod tamp_smisr;
/**TAMP_SCR (w) register accessor: TAMP status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_SCR)

For information about available fields see [`mod@tamp_scr`]
module*/
pub type TAMP_SCR = crate::Reg<tamp_scr::TAMP_SCRrs>;
///TAMP status clear register
pub mod tamp_scr;
/**TAMP_COUNT1R (r) register accessor: TAMP monotonic counter 1 register

You can [`read`](crate::Reg::read) this register and get [`tamp_count1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_COUNT1R)

For information about available fields see [`mod@tamp_count1r`]
module*/
pub type TAMP_COUNT1R = crate::Reg<tamp_count1r::TAMP_COUNT1Rrs>;
///TAMP monotonic counter 1 register
pub mod tamp_count1r;
/**TAMP_ERCFGR (rw) register accessor: TAMP erase configuration register

You can [`read`](crate::Reg::read) this register and get [`tamp_ercfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_ercfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_ERCFGR)

For information about available fields see [`mod@tamp_ercfgr`]
module*/
pub type TAMP_ERCFGR = crate::Reg<tamp_ercfgr::TAMP_ERCFGRrs>;
///TAMP erase configuration register
pub mod tamp_ercfgr;
/**TAMP_BKP0R (rw) register accessor: TAMP backup 0 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP0R)

For information about available fields see [`mod@tamp_bkp0r`]
module*/
pub type TAMP_BKP0R = crate::Reg<tamp_bkp0r::TAMP_BKP0Rrs>;
///TAMP backup 0 register
pub mod tamp_bkp0r;
/**TAMP_BKP1R (rw) register accessor: TAMP backup 1 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP1R)

For information about available fields see [`mod@tamp_bkp1r`]
module*/
pub type TAMP_BKP1R = crate::Reg<tamp_bkp1r::TAMP_BKP1Rrs>;
///TAMP backup 1 register
pub mod tamp_bkp1r;
/**TAMP_BKP2R (rw) register accessor: TAMP backup 2 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP2R)

For information about available fields see [`mod@tamp_bkp2r`]
module*/
pub type TAMP_BKP2R = crate::Reg<tamp_bkp2r::TAMP_BKP2Rrs>;
///TAMP backup 2 register
pub mod tamp_bkp2r;
/**TAMP_BKP3R (rw) register accessor: TAMP backup 3 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP3R)

For information about available fields see [`mod@tamp_bkp3r`]
module*/
pub type TAMP_BKP3R = crate::Reg<tamp_bkp3r::TAMP_BKP3Rrs>;
///TAMP backup 3 register
pub mod tamp_bkp3r;
/**TAMP_BKP4R (rw) register accessor: TAMP backup 4 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP4R)

For information about available fields see [`mod@tamp_bkp4r`]
module*/
pub type TAMP_BKP4R = crate::Reg<tamp_bkp4r::TAMP_BKP4Rrs>;
///TAMP backup 4 register
pub mod tamp_bkp4r;
/**TAMP_BKP5R (rw) register accessor: TAMP backup 5 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP5R)

For information about available fields see [`mod@tamp_bkp5r`]
module*/
pub type TAMP_BKP5R = crate::Reg<tamp_bkp5r::TAMP_BKP5Rrs>;
///TAMP backup 5 register
pub mod tamp_bkp5r;
/**TAMP_BKP6R (rw) register accessor: TAMP backup 6 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP6R)

For information about available fields see [`mod@tamp_bkp6r`]
module*/
pub type TAMP_BKP6R = crate::Reg<tamp_bkp6r::TAMP_BKP6Rrs>;
///TAMP backup 6 register
pub mod tamp_bkp6r;
/**TAMP_BKP7R (rw) register accessor: TAMP backup 7 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP7R)

For information about available fields see [`mod@tamp_bkp7r`]
module*/
pub type TAMP_BKP7R = crate::Reg<tamp_bkp7r::TAMP_BKP7Rrs>;
///TAMP backup 7 register
pub mod tamp_bkp7r;
/**TAMP_BKP8R (rw) register accessor: TAMP backup 8 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp8r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp8r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP8R)

For information about available fields see [`mod@tamp_bkp8r`]
module*/
pub type TAMP_BKP8R = crate::Reg<tamp_bkp8r::TAMP_BKP8Rrs>;
///TAMP backup 8 register
pub mod tamp_bkp8r;
/**TAMP_BKP9R (rw) register accessor: TAMP backup 9 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp9r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp9r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP9R)

For information about available fields see [`mod@tamp_bkp9r`]
module*/
pub type TAMP_BKP9R = crate::Reg<tamp_bkp9r::TAMP_BKP9Rrs>;
///TAMP backup 9 register
pub mod tamp_bkp9r;
/**TAMP_BKP10R (rw) register accessor: TAMP backup 10 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp10r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp10r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP10R)

For information about available fields see [`mod@tamp_bkp10r`]
module*/
pub type TAMP_BKP10R = crate::Reg<tamp_bkp10r::TAMP_BKP10Rrs>;
///TAMP backup 10 register
pub mod tamp_bkp10r;
/**TAMP_BKP11R (rw) register accessor: TAMP backup 11 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp11r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp11r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP11R)

For information about available fields see [`mod@tamp_bkp11r`]
module*/
pub type TAMP_BKP11R = crate::Reg<tamp_bkp11r::TAMP_BKP11Rrs>;
///TAMP backup 11 register
pub mod tamp_bkp11r;
/**TAMP_BKP12R (rw) register accessor: TAMP backup 12 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp12r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp12r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP12R)

For information about available fields see [`mod@tamp_bkp12r`]
module*/
pub type TAMP_BKP12R = crate::Reg<tamp_bkp12r::TAMP_BKP12Rrs>;
///TAMP backup 12 register
pub mod tamp_bkp12r;
/**TAMP_BKP13R (rw) register accessor: TAMP backup 13 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp13r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp13r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP13R)

For information about available fields see [`mod@tamp_bkp13r`]
module*/
pub type TAMP_BKP13R = crate::Reg<tamp_bkp13r::TAMP_BKP13Rrs>;
///TAMP backup 13 register
pub mod tamp_bkp13r;
/**TAMP_BKP14R (rw) register accessor: TAMP backup 14 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp14r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp14r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP14R)

For information about available fields see [`mod@tamp_bkp14r`]
module*/
pub type TAMP_BKP14R = crate::Reg<tamp_bkp14r::TAMP_BKP14Rrs>;
///TAMP backup 14 register
pub mod tamp_bkp14r;
/**TAMP_BKP15R (rw) register accessor: TAMP backup 15 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp15r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp15r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP15R)

For information about available fields see [`mod@tamp_bkp15r`]
module*/
pub type TAMP_BKP15R = crate::Reg<tamp_bkp15r::TAMP_BKP15Rrs>;
///TAMP backup 15 register
pub mod tamp_bkp15r;
/**TAMP_BKP16R (rw) register accessor: TAMP backup 16 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp16r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp16r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP16R)

For information about available fields see [`mod@tamp_bkp16r`]
module*/
pub type TAMP_BKP16R = crate::Reg<tamp_bkp16r::TAMP_BKP16Rrs>;
///TAMP backup 16 register
pub mod tamp_bkp16r;
/**TAMP_BKP17R (rw) register accessor: TAMP backup 17 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp17r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp17r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP17R)

For information about available fields see [`mod@tamp_bkp17r`]
module*/
pub type TAMP_BKP17R = crate::Reg<tamp_bkp17r::TAMP_BKP17Rrs>;
///TAMP backup 17 register
pub mod tamp_bkp17r;
/**TAMP_BKP18R (rw) register accessor: TAMP backup 18 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp18r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp18r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP18R)

For information about available fields see [`mod@tamp_bkp18r`]
module*/
pub type TAMP_BKP18R = crate::Reg<tamp_bkp18r::TAMP_BKP18Rrs>;
///TAMP backup 18 register
pub mod tamp_bkp18r;
/**TAMP_BKP19R (rw) register accessor: TAMP backup 19 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp19r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp19r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP19R)

For information about available fields see [`mod@tamp_bkp19r`]
module*/
pub type TAMP_BKP19R = crate::Reg<tamp_bkp19r::TAMP_BKP19Rrs>;
///TAMP backup 19 register
pub mod tamp_bkp19r;
/**TAMP_BKP20R (rw) register accessor: TAMP backup 20 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp20r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp20r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP20R)

For information about available fields see [`mod@tamp_bkp20r`]
module*/
pub type TAMP_BKP20R = crate::Reg<tamp_bkp20r::TAMP_BKP20Rrs>;
///TAMP backup 20 register
pub mod tamp_bkp20r;
/**TAMP_BKP21R (rw) register accessor: TAMP backup 21 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp21r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp21r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP21R)

For information about available fields see [`mod@tamp_bkp21r`]
module*/
pub type TAMP_BKP21R = crate::Reg<tamp_bkp21r::TAMP_BKP21Rrs>;
///TAMP backup 21 register
pub mod tamp_bkp21r;
/**TAMP_BKP22R (rw) register accessor: TAMP backup 22 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp22r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp22r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP22R)

For information about available fields see [`mod@tamp_bkp22r`]
module*/
pub type TAMP_BKP22R = crate::Reg<tamp_bkp22r::TAMP_BKP22Rrs>;
///TAMP backup 22 register
pub mod tamp_bkp22r;
/**TAMP_BKP23R (rw) register accessor: TAMP backup 23 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp23r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp23r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP23R)

For information about available fields see [`mod@tamp_bkp23r`]
module*/
pub type TAMP_BKP23R = crate::Reg<tamp_bkp23r::TAMP_BKP23Rrs>;
///TAMP backup 23 register
pub mod tamp_bkp23r;
/**TAMP_BKP24R (rw) register accessor: TAMP backup 24 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp24r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp24r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP24R)

For information about available fields see [`mod@tamp_bkp24r`]
module*/
pub type TAMP_BKP24R = crate::Reg<tamp_bkp24r::TAMP_BKP24Rrs>;
///TAMP backup 24 register
pub mod tamp_bkp24r;
/**TAMP_BKP25R (rw) register accessor: TAMP backup 25 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp25r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp25r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP25R)

For information about available fields see [`mod@tamp_bkp25r`]
module*/
pub type TAMP_BKP25R = crate::Reg<tamp_bkp25r::TAMP_BKP25Rrs>;
///TAMP backup 25 register
pub mod tamp_bkp25r;
/**TAMP_BKP26R (rw) register accessor: TAMP backup 26 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp26r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp26r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP26R)

For information about available fields see [`mod@tamp_bkp26r`]
module*/
pub type TAMP_BKP26R = crate::Reg<tamp_bkp26r::TAMP_BKP26Rrs>;
///TAMP backup 26 register
pub mod tamp_bkp26r;
/**TAMP_BKP27R (rw) register accessor: TAMP backup 27 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp27r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp27r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP27R)

For information about available fields see [`mod@tamp_bkp27r`]
module*/
pub type TAMP_BKP27R = crate::Reg<tamp_bkp27r::TAMP_BKP27Rrs>;
///TAMP backup 27 register
pub mod tamp_bkp27r;
/**TAMP_BKP28R (rw) register accessor: TAMP backup 28 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp28r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp28r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP28R)

For information about available fields see [`mod@tamp_bkp28r`]
module*/
pub type TAMP_BKP28R = crate::Reg<tamp_bkp28r::TAMP_BKP28Rrs>;
///TAMP backup 28 register
pub mod tamp_bkp28r;
/**TAMP_BKP29R (rw) register accessor: TAMP backup 29 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp29r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp29r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP29R)

For information about available fields see [`mod@tamp_bkp29r`]
module*/
pub type TAMP_BKP29R = crate::Reg<tamp_bkp29r::TAMP_BKP29Rrs>;
///TAMP backup 29 register
pub mod tamp_bkp29r;
/**TAMP_BKP30R (rw) register accessor: TAMP backup 30 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp30r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp30r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP30R)

For information about available fields see [`mod@tamp_bkp30r`]
module*/
pub type TAMP_BKP30R = crate::Reg<tamp_bkp30r::TAMP_BKP30Rrs>;
///TAMP backup 30 register
pub mod tamp_bkp30r;
/**TAMP_BKP31R (rw) register accessor: TAMP backup 31 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp31r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp31r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:TAMP_BKP31R)

For information about available fields see [`mod@tamp_bkp31r`]
module*/
pub type TAMP_BKP31R = crate::Reg<tamp_bkp31r::TAMP_BKP31Rrs>;
///TAMP backup 31 register
pub mod tamp_bkp31r;
