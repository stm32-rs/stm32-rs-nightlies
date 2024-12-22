#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tamp_cr1: TAMP_CR1,
    tamp_cr2: TAMP_CR2,
    tamp_cr3: TAMP_CR3,
    tamp_fltcr: TAMP_FLTCR,
    _reserved4: [u8; 0x1c],
    tamp_ier: TAMP_IER,
    tamp_sr: TAMP_SR,
    tamp_misr: TAMP_MISR,
    _reserved7: [u8; 0x04],
    tamp_scr: TAMP_SCR,
    _reserved8: [u8; 0xc0],
    tamp_bkp0r: TAMP_BKP0R,
    tamp_bkp1r: TAMP_BKP1R,
    tamp_bkp2r: TAMP_BKP2R,
    tamp_bkp3r: TAMP_BKP3R,
    tamp_bkp4r: TAMP_BKP4R,
    tamp_bkp5r: TAMP_BKP5R,
    tamp_bkp6r: TAMP_BKP6R,
    tamp_bkp7r: TAMP_BKP7R,
    tamp_bkp8r: TAMP_BKP8R,
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
    ///0x34 - TAMP masked interrupt status register
    #[inline(always)]
    pub const fn tamp_misr(&self) -> &TAMP_MISR {
        &self.tamp_misr
    }
    ///0x3c - TAMP status clear register
    #[inline(always)]
    pub const fn tamp_scr(&self) -> &TAMP_SCR {
        &self.tamp_scr
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
}
/**TAMP_CR1 (rw) register accessor: TAMP control register 1

You can [`read`](crate::Reg::read) this register and get [`tamp_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_CR1)

For information about available fields see [`mod@tamp_cr1`]
module*/
pub type TAMP_CR1 = crate::Reg<tamp_cr1::TAMP_CR1rs>;
///TAMP control register 1
pub mod tamp_cr1;
/**TAMP_CR2 (rw) register accessor: TAMP control register 2

You can [`read`](crate::Reg::read) this register and get [`tamp_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_CR2)

For information about available fields see [`mod@tamp_cr2`]
module*/
pub type TAMP_CR2 = crate::Reg<tamp_cr2::TAMP_CR2rs>;
///TAMP control register 2
pub mod tamp_cr2;
/**TAMP_CR3 (rw) register accessor: TAMP control register 3

You can [`read`](crate::Reg::read) this register and get [`tamp_cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_CR3)

For information about available fields see [`mod@tamp_cr3`]
module*/
pub type TAMP_CR3 = crate::Reg<tamp_cr3::TAMP_CR3rs>;
///TAMP control register 3
pub mod tamp_cr3;
/**TAMP_FLTCR (rw) register accessor: TAMP filter control register

You can [`read`](crate::Reg::read) this register and get [`tamp_fltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_fltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_FLTCR)

For information about available fields see [`mod@tamp_fltcr`]
module*/
pub type TAMP_FLTCR = crate::Reg<tamp_fltcr::TAMP_FLTCRrs>;
///TAMP filter control register
pub mod tamp_fltcr;
/**TAMP_IER (rw) register accessor: TAMP interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tamp_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_IER)

For information about available fields see [`mod@tamp_ier`]
module*/
pub type TAMP_IER = crate::Reg<tamp_ier::TAMP_IERrs>;
///TAMP interrupt enable register
pub mod tamp_ier;
/**TAMP_SR (r) register accessor: TAMP status register

You can [`read`](crate::Reg::read) this register and get [`tamp_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_SR)

For information about available fields see [`mod@tamp_sr`]
module*/
pub type TAMP_SR = crate::Reg<tamp_sr::TAMP_SRrs>;
///TAMP status register
pub mod tamp_sr;
/**TAMP_MISR (r) register accessor: TAMP masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`tamp_misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_MISR)

For information about available fields see [`mod@tamp_misr`]
module*/
pub type TAMP_MISR = crate::Reg<tamp_misr::TAMP_MISRrs>;
///TAMP masked interrupt status register
pub mod tamp_misr;
/**TAMP_SCR (w) register accessor: TAMP status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_SCR)

For information about available fields see [`mod@tamp_scr`]
module*/
pub type TAMP_SCR = crate::Reg<tamp_scr::TAMP_SCRrs>;
///TAMP status clear register
pub mod tamp_scr;
/**TAMP_BKP0R (rw) register accessor: TAMP backup 0 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_BKP0R)

For information about available fields see [`mod@tamp_bkp0r`]
module*/
pub type TAMP_BKP0R = crate::Reg<tamp_bkp0r::TAMP_BKP0Rrs>;
///TAMP backup 0 register
pub mod tamp_bkp0r;
/**TAMP_BKP1R (rw) register accessor: TAMP backup 1 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_BKP1R)

For information about available fields see [`mod@tamp_bkp1r`]
module*/
pub type TAMP_BKP1R = crate::Reg<tamp_bkp1r::TAMP_BKP1Rrs>;
///TAMP backup 1 register
pub mod tamp_bkp1r;
/**TAMP_BKP2R (rw) register accessor: TAMP backup 2 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_BKP2R)

For information about available fields see [`mod@tamp_bkp2r`]
module*/
pub type TAMP_BKP2R = crate::Reg<tamp_bkp2r::TAMP_BKP2Rrs>;
///TAMP backup 2 register
pub mod tamp_bkp2r;
/**TAMP_BKP3R (rw) register accessor: TAMP backup 3 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_BKP3R)

For information about available fields see [`mod@tamp_bkp3r`]
module*/
pub type TAMP_BKP3R = crate::Reg<tamp_bkp3r::TAMP_BKP3Rrs>;
///TAMP backup 3 register
pub mod tamp_bkp3r;
/**TAMP_BKP4R (rw) register accessor: TAMP backup 4 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_BKP4R)

For information about available fields see [`mod@tamp_bkp4r`]
module*/
pub type TAMP_BKP4R = crate::Reg<tamp_bkp4r::TAMP_BKP4Rrs>;
///TAMP backup 4 register
pub mod tamp_bkp4r;
/**TAMP_BKP5R (rw) register accessor: TAMP backup 5 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_BKP5R)

For information about available fields see [`mod@tamp_bkp5r`]
module*/
pub type TAMP_BKP5R = crate::Reg<tamp_bkp5r::TAMP_BKP5Rrs>;
///TAMP backup 5 register
pub mod tamp_bkp5r;
/**TAMP_BKP6R (rw) register accessor: TAMP backup 6 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_BKP6R)

For information about available fields see [`mod@tamp_bkp6r`]
module*/
pub type TAMP_BKP6R = crate::Reg<tamp_bkp6r::TAMP_BKP6Rrs>;
///TAMP backup 6 register
pub mod tamp_bkp6r;
/**TAMP_BKP7R (rw) register accessor: TAMP backup 7 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_BKP7R)

For information about available fields see [`mod@tamp_bkp7r`]
module*/
pub type TAMP_BKP7R = crate::Reg<tamp_bkp7r::TAMP_BKP7Rrs>;
///TAMP backup 7 register
pub mod tamp_bkp7r;
/**TAMP_BKP8R (rw) register accessor: TAMP backup 8 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp8r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp8r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TAMP:TAMP_BKP8R)

For information about available fields see [`mod@tamp_bkp8r`]
module*/
pub type TAMP_BKP8R = crate::Reg<tamp_bkp8r::TAMP_BKP8Rrs>;
///TAMP backup 8 register
pub mod tamp_bkp8r;
