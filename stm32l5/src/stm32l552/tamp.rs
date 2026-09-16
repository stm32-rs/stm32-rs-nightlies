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
    smcr: SMCR,
    privcr: PRIVCR,
    _reserved10: [u8; 0x04],
    ier: IER,
    sr: SR,
    misr: MISR,
    smisr: SMISR,
    scr: SCR,
    countr: COUNTR,
    _reserved16: [u8; 0x0c],
    cfgr: CFGR,
    _reserved17: [u8; 0xac],
    bkpr: [BKPR; 32],
}
impl RegisterBlock {
    ///0x00 - control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - control register 3
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
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
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
    ///0x34 - TAMP masked interrupt status register
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
    ///0x40 - TAMP monotonic counter register
    #[inline(always)]
    pub const fn countr(&self) -> &COUNTR {
        &self.countr
    }
    ///0x50 - TAMP configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x100..0x180 - TAMP backup register
    #[inline(always)]
    pub const fn bkpr(&self, n: usize) -> &BKPR {
        &self.bkpr[n]
    }
    ///Iterator for array of:
    ///0x100..0x180 - TAMP backup register
    #[inline(always)]
    pub fn bkpr_iter(&self) -> impl Iterator<Item = &BKPR> {
        self.bkpr.iter()
    }
    ///0x100 - TAMP backup register
    #[inline(always)]
    pub const fn bkp0r(&self) -> &BKPR {
        self.bkpr(0)
    }
    ///0x104 - TAMP backup register
    #[inline(always)]
    pub const fn bkp1r(&self) -> &BKPR {
        self.bkpr(1)
    }
    ///0x108 - TAMP backup register
    #[inline(always)]
    pub const fn bkp2r(&self) -> &BKPR {
        self.bkpr(2)
    }
    ///0x10c - TAMP backup register
    #[inline(always)]
    pub const fn bkp3r(&self) -> &BKPR {
        self.bkpr(3)
    }
    ///0x110 - TAMP backup register
    #[inline(always)]
    pub const fn bkp4r(&self) -> &BKPR {
        self.bkpr(4)
    }
    ///0x114 - TAMP backup register
    #[inline(always)]
    pub const fn bkp5r(&self) -> &BKPR {
        self.bkpr(5)
    }
    ///0x118 - TAMP backup register
    #[inline(always)]
    pub const fn bkp6r(&self) -> &BKPR {
        self.bkpr(6)
    }
    ///0x11c - TAMP backup register
    #[inline(always)]
    pub const fn bkp7r(&self) -> &BKPR {
        self.bkpr(7)
    }
    ///0x120 - TAMP backup register
    #[inline(always)]
    pub const fn bkp8r(&self) -> &BKPR {
        self.bkpr(8)
    }
    ///0x124 - TAMP backup register
    #[inline(always)]
    pub const fn bkp9r(&self) -> &BKPR {
        self.bkpr(9)
    }
    ///0x128 - TAMP backup register
    #[inline(always)]
    pub const fn bkp10r(&self) -> &BKPR {
        self.bkpr(10)
    }
    ///0x12c - TAMP backup register
    #[inline(always)]
    pub const fn bkp11r(&self) -> &BKPR {
        self.bkpr(11)
    }
    ///0x130 - TAMP backup register
    #[inline(always)]
    pub const fn bkp12r(&self) -> &BKPR {
        self.bkpr(12)
    }
    ///0x134 - TAMP backup register
    #[inline(always)]
    pub const fn bkp13r(&self) -> &BKPR {
        self.bkpr(13)
    }
    ///0x138 - TAMP backup register
    #[inline(always)]
    pub const fn bkp14r(&self) -> &BKPR {
        self.bkpr(14)
    }
    ///0x13c - TAMP backup register
    #[inline(always)]
    pub const fn bkp15r(&self) -> &BKPR {
        self.bkpr(15)
    }
    ///0x140 - TAMP backup register
    #[inline(always)]
    pub const fn bkp16r(&self) -> &BKPR {
        self.bkpr(16)
    }
    ///0x144 - TAMP backup register
    #[inline(always)]
    pub const fn bkp17r(&self) -> &BKPR {
        self.bkpr(17)
    }
    ///0x148 - TAMP backup register
    #[inline(always)]
    pub const fn bkp18r(&self) -> &BKPR {
        self.bkpr(18)
    }
    ///0x14c - TAMP backup register
    #[inline(always)]
    pub const fn bkp19r(&self) -> &BKPR {
        self.bkpr(19)
    }
    ///0x150 - TAMP backup register
    #[inline(always)]
    pub const fn bkp20r(&self) -> &BKPR {
        self.bkpr(20)
    }
    ///0x154 - TAMP backup register
    #[inline(always)]
    pub const fn bkp21r(&self) -> &BKPR {
        self.bkpr(21)
    }
    ///0x158 - TAMP backup register
    #[inline(always)]
    pub const fn bkp22r(&self) -> &BKPR {
        self.bkpr(22)
    }
    ///0x15c - TAMP backup register
    #[inline(always)]
    pub const fn bkp23r(&self) -> &BKPR {
        self.bkpr(23)
    }
    ///0x160 - TAMP backup register
    #[inline(always)]
    pub const fn bkp24r(&self) -> &BKPR {
        self.bkpr(24)
    }
    ///0x164 - TAMP backup register
    #[inline(always)]
    pub const fn bkp25r(&self) -> &BKPR {
        self.bkpr(25)
    }
    ///0x168 - TAMP backup register
    #[inline(always)]
    pub const fn bkp26r(&self) -> &BKPR {
        self.bkpr(26)
    }
    ///0x16c - TAMP backup register
    #[inline(always)]
    pub const fn bkp27r(&self) -> &BKPR {
        self.bkpr(27)
    }
    ///0x170 - TAMP backup register
    #[inline(always)]
    pub const fn bkp28r(&self) -> &BKPR {
        self.bkpr(28)
    }
    ///0x174 - TAMP backup register
    #[inline(always)]
    pub const fn bkp29r(&self) -> &BKPR {
        self.bkpr(29)
    }
    ///0x178 - TAMP backup register
    #[inline(always)]
    pub const fn bkp30r(&self) -> &BKPR {
        self.bkpr(30)
    }
    ///0x17c - TAMP backup register
    #[inline(always)]
    pub const fn bkp31r(&self) -> &BKPR {
        self.bkpr(31)
    }
}
/**CR1 (rw) register accessor: control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///control register 1
pub mod cr1;
/**CR2 (rw) register accessor: control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///control register 2
pub mod cr2;
/**CR3 (rw) register accessor: control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:CR3)

For information about available fields see [`mod@cr3`] module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///control register 3
pub mod cr3;
/**FLTCR (rw) register accessor: TAMP filter control register

You can [`read`](crate::Reg::read) this register and get [`fltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:FLTCR)

For information about available fields see [`mod@fltcr`] module*/
pub type FLTCR = crate::Reg<fltcr::FLTCRrs>;
///TAMP filter control register
pub mod fltcr;
/**ATCR1 (rw) register accessor: TAMP active tamper control register 1

You can [`read`](crate::Reg::read) this register and get [`atcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:ATCR1)

For information about available fields see [`mod@atcr1`] module*/
pub type ATCR1 = crate::Reg<atcr1::ATCR1rs>;
///TAMP active tamper control register 1
pub mod atcr1;
/**ATSEEDR (w) register accessor: TAMP active tamper seed register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atseedr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:ATSEEDR)

For information about available fields see [`mod@atseedr`] module*/
pub type ATSEEDR = crate::Reg<atseedr::ATSEEDRrs>;
///TAMP active tamper seed register
pub mod atseedr;
/**ATOR (r) register accessor: TAMP active tamper output register

You can [`read`](crate::Reg::read) this register and get [`ator::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:ATOR)

For information about available fields see [`mod@ator`] module*/
pub type ATOR = crate::Reg<ator::ATORrs>;
///TAMP active tamper output register
pub mod ator;
/**ATCR2 (rw) register accessor: TAMP active tamper control register 2

You can [`read`](crate::Reg::read) this register and get [`atcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:ATCR2)

For information about available fields see [`mod@atcr2`] module*/
pub type ATCR2 = crate::Reg<atcr2::ATCR2rs>;
///TAMP active tamper control register 2
pub mod atcr2;
/**SMCR (rw) register accessor: TAMP secure mode register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:SMCR)

For information about available fields see [`mod@smcr`] module*/
pub type SMCR = crate::Reg<smcr::SMCRrs>;
///TAMP secure mode register
pub mod smcr;
/**PRIVCR (rw) register accessor: TAMP privilege mode control register

You can [`read`](crate::Reg::read) this register and get [`privcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:PRIVCR)

For information about available fields see [`mod@privcr`] module*/
pub type PRIVCR = crate::Reg<privcr::PRIVCRrs>;
///TAMP privilege mode control register
pub mod privcr;
/**IER (rw) register accessor: TAMP interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///TAMP interrupt enable register
pub mod ier;
/**SR (r) register accessor: TAMP status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///TAMP status register
pub mod sr;
/**MISR (r) register accessor: TAMP masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:MISR)

For information about available fields see [`mod@misr`] module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///TAMP masked interrupt status register
pub mod misr;
/**SMISR (r) register accessor: TAMP secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`smisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:SMISR)

For information about available fields see [`mod@smisr`] module*/
pub type SMISR = crate::Reg<smisr::SMISRrs>;
///TAMP secure masked interrupt status register
pub mod smisr;
/**SCR (w) register accessor: TAMP status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:SCR)

For information about available fields see [`mod@scr`] module*/
pub type SCR = crate::Reg<scr::SCRrs>;
///TAMP status clear register
pub mod scr;
/**COUNTR (r) register accessor: TAMP monotonic counter register

You can [`read`](crate::Reg::read) this register and get [`countr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:COUNTR)

For information about available fields see [`mod@countr`] module*/
pub type COUNTR = crate::Reg<countr::COUNTRrs>;
///TAMP monotonic counter register
pub mod countr;
/**CFGR (rw) register accessor: TAMP configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///TAMP configuration register
pub mod cfgr;
/**BKPR (rw) register accessor: TAMP backup register

You can [`read`](crate::Reg::read) this register and get [`bkpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:BKP[0]R)

For information about available fields see [`mod@bkpr`] module*/
pub type BKPR = crate::Reg<bkpr::BKPRrs>;
///TAMP backup register
pub mod bkpr;
