#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    _reserved2: [u8; 0x04],
    fltcr: FLTCR,
    atcr1: ATCR1,
    atseedr: ATSEEDR,
    ator: ATOR,
    _reserved6: [u8; 0x04],
    smcr: SMCR,
    _reserved7: [u8; 0x08],
    ier: IER,
    sr: SR,
    misr: MISR,
    smisr: SMISR,
    scr: SCR,
    countr: COUNTR,
    _reserved13: [u8; 0x0c],
    cfgr: CFGR,
    _reserved14: [u8; 0xac],
    bkpr: [BKPR; 32],
    _reserved15: [u8; 0x026c],
    hwcfgr2: HWCFGR2,
    hwcfgr1: HWCFGR1,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x0c - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    #[inline(always)]
    pub const fn fltcr(&self) -> &FLTCR {
        &self.fltcr
    }
    ///0x10 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    #[inline(always)]
    pub const fn atcr1(&self) -> &ATCR1 {
        &self.atcr1
    }
    ///0x14 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    #[inline(always)]
    pub const fn atseedr(&self) -> &ATSEEDR {
        &self.atseedr
    }
    ///0x18 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    #[inline(always)]
    pub const fn ator(&self) -> &ATOR {
        &self.ator
    }
    ///0x20 - This register can be written only when the APB access is secure.
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
    }
    ///0x2c - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x30 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
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
    ///0x100..0x180 - TAMP backup %s register
    #[inline(always)]
    pub const fn bkpr(&self, n: usize) -> &BKPR {
        &self.bkpr[n]
    }
    ///Iterator for array of:
    ///0x100..0x180 - TAMP backup %s register
    #[inline(always)]
    pub fn bkpr_iter(&self) -> impl Iterator<Item = &BKPR> {
        self.bkpr.iter()
    }
    ///0x100 - TAMP backup 0 register
    #[inline(always)]
    pub const fn bkp0r(&self) -> &BKPR {
        self.bkpr(0)
    }
    ///0x104 - TAMP backup 1 register
    #[inline(always)]
    pub const fn bkp1r(&self) -> &BKPR {
        self.bkpr(1)
    }
    ///0x108 - TAMP backup 2 register
    #[inline(always)]
    pub const fn bkp2r(&self) -> &BKPR {
        self.bkpr(2)
    }
    ///0x10c - TAMP backup 3 register
    #[inline(always)]
    pub const fn bkp3r(&self) -> &BKPR {
        self.bkpr(3)
    }
    ///0x110 - TAMP backup 4 register
    #[inline(always)]
    pub const fn bkp4r(&self) -> &BKPR {
        self.bkpr(4)
    }
    ///0x114 - TAMP backup 5 register
    #[inline(always)]
    pub const fn bkp5r(&self) -> &BKPR {
        self.bkpr(5)
    }
    ///0x118 - TAMP backup 6 register
    #[inline(always)]
    pub const fn bkp6r(&self) -> &BKPR {
        self.bkpr(6)
    }
    ///0x11c - TAMP backup 7 register
    #[inline(always)]
    pub const fn bkp7r(&self) -> &BKPR {
        self.bkpr(7)
    }
    ///0x120 - TAMP backup 8 register
    #[inline(always)]
    pub const fn bkp8r(&self) -> &BKPR {
        self.bkpr(8)
    }
    ///0x124 - TAMP backup 9 register
    #[inline(always)]
    pub const fn bkp9r(&self) -> &BKPR {
        self.bkpr(9)
    }
    ///0x128 - TAMP backup 10 register
    #[inline(always)]
    pub const fn bkp10r(&self) -> &BKPR {
        self.bkpr(10)
    }
    ///0x12c - TAMP backup 11 register
    #[inline(always)]
    pub const fn bkp11r(&self) -> &BKPR {
        self.bkpr(11)
    }
    ///0x130 - TAMP backup 12 register
    #[inline(always)]
    pub const fn bkp12r(&self) -> &BKPR {
        self.bkpr(12)
    }
    ///0x134 - TAMP backup 13 register
    #[inline(always)]
    pub const fn bkp13r(&self) -> &BKPR {
        self.bkpr(13)
    }
    ///0x138 - TAMP backup 14 register
    #[inline(always)]
    pub const fn bkp14r(&self) -> &BKPR {
        self.bkpr(14)
    }
    ///0x13c - TAMP backup 15 register
    #[inline(always)]
    pub const fn bkp15r(&self) -> &BKPR {
        self.bkpr(15)
    }
    ///0x140 - TAMP backup 16 register
    #[inline(always)]
    pub const fn bkp16r(&self) -> &BKPR {
        self.bkpr(16)
    }
    ///0x144 - TAMP backup 17 register
    #[inline(always)]
    pub const fn bkp17r(&self) -> &BKPR {
        self.bkpr(17)
    }
    ///0x148 - TAMP backup 18 register
    #[inline(always)]
    pub const fn bkp18r(&self) -> &BKPR {
        self.bkpr(18)
    }
    ///0x14c - TAMP backup 19 register
    #[inline(always)]
    pub const fn bkp19r(&self) -> &BKPR {
        self.bkpr(19)
    }
    ///0x150 - TAMP backup 20 register
    #[inline(always)]
    pub const fn bkp20r(&self) -> &BKPR {
        self.bkpr(20)
    }
    ///0x154 - TAMP backup 21 register
    #[inline(always)]
    pub const fn bkp21r(&self) -> &BKPR {
        self.bkpr(21)
    }
    ///0x158 - TAMP backup 22 register
    #[inline(always)]
    pub const fn bkp22r(&self) -> &BKPR {
        self.bkpr(22)
    }
    ///0x15c - TAMP backup 23 register
    #[inline(always)]
    pub const fn bkp23r(&self) -> &BKPR {
        self.bkpr(23)
    }
    ///0x160 - TAMP backup 24 register
    #[inline(always)]
    pub const fn bkp24r(&self) -> &BKPR {
        self.bkpr(24)
    }
    ///0x164 - TAMP backup 25 register
    #[inline(always)]
    pub const fn bkp25r(&self) -> &BKPR {
        self.bkpr(25)
    }
    ///0x168 - TAMP backup 26 register
    #[inline(always)]
    pub const fn bkp26r(&self) -> &BKPR {
        self.bkpr(26)
    }
    ///0x16c - TAMP backup 27 register
    #[inline(always)]
    pub const fn bkp27r(&self) -> &BKPR {
        self.bkpr(27)
    }
    ///0x170 - TAMP backup 28 register
    #[inline(always)]
    pub const fn bkp28r(&self) -> &BKPR {
        self.bkpr(28)
    }
    ///0x174 - TAMP backup 29 register
    #[inline(always)]
    pub const fn bkp29r(&self) -> &BKPR {
        self.bkpr(29)
    }
    ///0x178 - TAMP backup 30 register
    #[inline(always)]
    pub const fn bkp30r(&self) -> &BKPR {
        self.bkpr(30)
    }
    ///0x17c - TAMP backup 31 register
    #[inline(always)]
    pub const fn bkp31r(&self) -> &BKPR {
        self.bkpr(31)
    }
    ///0x3ec - TAMP hardware configuration register 2
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    ///0x3f0 - TAMP hardware configuration register 1
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
    ///0x3f4 - TAMP version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - TAMP identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - TAMP size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**CR1 (rw) register accessor: This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod cr1;
/**CR2 (rw) register accessor: This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod cr2;
/**FLTCR (rw) register accessor: This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`fltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:FLTCR)

For information about available fields see [`mod@fltcr`] module*/
pub type FLTCR = crate::Reg<fltcr::FLTCRrs>;
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod fltcr;
/**ATCR1 (rw) register accessor: This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`atcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:ATCR1)

For information about available fields see [`mod@atcr1`] module*/
pub type ATCR1 = crate::Reg<atcr1::ATCR1rs>;
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod atcr1;
/**ATSEEDR (w) register accessor: This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atseedr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:ATSEEDR)

For information about available fields see [`mod@atseedr`] module*/
pub type ATSEEDR = crate::Reg<atseedr::ATSEEDRrs>;
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod atseedr;
/**ATOR (r) register accessor: This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`ator::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:ATOR)

For information about available fields see [`mod@ator`] module*/
pub type ATOR = crate::Reg<ator::ATORrs>;
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod ator;
/**SMCR (rw) register accessor: This register can be written only when the APB access is secure.

You can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:SMCR)

For information about available fields see [`mod@smcr`] module*/
pub type SMCR = crate::Reg<smcr::SMCRrs>;
///This register can be written only when the APB access is secure.
pub mod smcr;
/**IER (rw) register accessor: This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod ier;
/**SR (r) register accessor: This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod sr;
/**MISR (r) register accessor: TAMP non-secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:MISR)

For information about available fields see [`mod@misr`] module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///TAMP non-secure masked interrupt status register
pub mod misr;
/**SMISR (r) register accessor: TAMP secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`smisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:SMISR)

For information about available fields see [`mod@smisr`] module*/
pub type SMISR = crate::Reg<smisr::SMISRrs>;
///TAMP secure masked interrupt status register
pub mod smisr;
/**SCR (w) register accessor: TAMP status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:SCR)

For information about available fields see [`mod@scr`] module*/
pub type SCR = crate::Reg<scr::SCRrs>;
///TAMP status clear register
pub mod scr;
/**COUNTR (r) register accessor: TAMP monotonic counter register

You can [`read`](crate::Reg::read) this register and get [`countr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:COUNTR)

For information about available fields see [`mod@countr`] module*/
pub type COUNTR = crate::Reg<countr::COUNTRrs>;
///TAMP monotonic counter register
pub mod countr;
/**CFGR (rw) register accessor: TAMP configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///TAMP configuration register
pub mod cfgr;
/**BKPR (rw) register accessor: TAMP backup %s register

You can [`read`](crate::Reg::read) this register and get [`bkpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:BKP[0]R)

For information about available fields see [`mod@bkpr`] module*/
pub type BKPR = crate::Reg<bkpr::BKPRrs>;
///TAMP backup %s register
pub mod bkpr;
/**HWCFGR2 (r) register accessor: TAMP hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:HWCFGR2)

For information about available fields see [`mod@hwcfgr2`] module*/
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
///TAMP hardware configuration register 2
pub mod hwcfgr2;
/**HWCFGR1 (r) register accessor: TAMP hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:HWCFGR1)

For information about available fields see [`mod@hwcfgr1`] module*/
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
///TAMP hardware configuration register 1
pub mod hwcfgr1;
/**VERR (r) register accessor: TAMP version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///TAMP version register
pub mod verr;
/**IPIDR (r) register accessor: TAMP identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///TAMP identification register
pub mod ipidr;
/**SIDR (r) register accessor: TAMP size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///TAMP size identification register
pub mod sidr;
