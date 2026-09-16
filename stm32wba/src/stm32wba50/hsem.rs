#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    r: [R; 16],
    _reserved1: [u8; 0x40],
    rlr: [RLR; 16],
    _reserved2: [u8; 0x40],
    ier: IER,
    icr: ICR,
    isr: ISR,
    misr: MISR,
    _reserved6: [u8; 0x70],
    sier: SIER,
    sicr: SICR,
    sisr: SISR,
    msisr: MSISR,
    _reserved10: [u8; 0x70],
    seccfgr: SECCFGR,
    _reserved11: [u8; 0x0c],
    privcfgr: PRIVCFGR,
    _reserved12: [u8; 0x1c],
    cr: CR,
    keyr: KEYR,
}
impl RegisterBlock {
    ///0x00..0x40 - HSEM register HSEM_R%s
    #[inline(always)]
    pub const fn r(&self, n: usize) -> &R {
        &self.r[n]
    }
    ///Iterator for array of:
    ///0x00..0x40 - HSEM register HSEM_R%s
    #[inline(always)]
    pub fn r_iter(&self) -> impl Iterator<Item = &R> {
        self.r.iter()
    }
    ///0x80..0xc0 - Semaphore %s read lock register
    #[inline(always)]
    pub const fn rlr(&self, n: usize) -> &RLR {
        &self.rlr[n]
    }
    ///Iterator for array of:
    ///0x80..0xc0 - Semaphore %s read lock register
    #[inline(always)]
    pub fn rlr_iter(&self) -> impl Iterator<Item = &RLR> {
        self.rlr.iter()
    }
    ///0x100 - HSEM non-secure interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x104 - HSEM non-secure interrupt clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x108 - HSEM non-secure interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x10c - HSEM non-secure interrupt status register
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    ///0x180 - HSEM secure interrupt enable register
    #[inline(always)]
    pub const fn sier(&self) -> &SIER {
        &self.sier
    }
    ///0x184 - HSEM secure interrupt clear register
    #[inline(always)]
    pub const fn sicr(&self) -> &SICR {
        &self.sicr
    }
    ///0x188 - HSEM secure interrupt status register
    #[inline(always)]
    pub const fn sisr(&self) -> &SISR {
        &self.sisr
    }
    ///0x18c - HSEM secure masked interrupt status register
    #[inline(always)]
    pub const fn msisr(&self) -> &MSISR {
        &self.msisr
    }
    ///0x200 - HSEM security configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x210 - HSEM privilege configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    ///0x230 - HSEM clear register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x234 - HSEM interrupt clear register
    #[inline(always)]
    pub const fn keyr(&self) -> &KEYR {
        &self.keyr
    }
}
/**R (rw) register accessor: HSEM register HSEM_R%s

You can [`read`](crate::Reg::read) this register and get [`r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HSEM:R[0])

For information about available fields see [`mod@r`] module*/
pub type R = crate::Reg<r::Rrs>;
///HSEM register HSEM_R%s
pub mod r;
/**RLR (r) register accessor: Semaphore %s read lock register

You can [`read`](crate::Reg::read) this register and get [`rlr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HSEM:RLR[0])

For information about available fields see [`mod@rlr`] module*/
pub type RLR = crate::Reg<rlr::RLRrs>;
///Semaphore %s read lock register
pub mod rlr;
/**IER (rw) register accessor: HSEM non-secure interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HSEM:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///HSEM non-secure interrupt enable register
pub mod ier;
/**ICR (rw) register accessor: HSEM non-secure interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HSEM:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///HSEM non-secure interrupt clear register
pub mod icr;
/**ISR (rw) register accessor: HSEM non-secure interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HSEM:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///HSEM non-secure interrupt status register
pub mod isr;
/**MISR (rw) register accessor: HSEM non-secure interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HSEM:MISR)

For information about available fields see [`mod@misr`] module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///HSEM non-secure interrupt status register
pub mod misr;
/**SIER (rw) register accessor: HSEM secure interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`sier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HSEM:SIER)

For information about available fields see [`mod@sier`] module*/
pub type SIER = crate::Reg<sier::SIERrs>;
///HSEM secure interrupt enable register
pub mod sier;
/**SICR (rw) register accessor: HSEM secure interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`sicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HSEM:SICR)

For information about available fields see [`mod@sicr`] module*/
pub type SICR = crate::Reg<sicr::SICRrs>;
///HSEM secure interrupt clear register
pub mod sicr;
/**SISR (r) register accessor: HSEM secure interrupt status register

You can [`read`](crate::Reg::read) this register and get [`sisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HSEM:SISR)

For information about available fields see [`mod@sisr`] module*/
pub type SISR = crate::Reg<sisr::SISRrs>;
///HSEM secure interrupt status register
pub mod sisr;
/**MSISR (r) register accessor: HSEM secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`msisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HSEM:MSISR)

For information about available fields see [`mod@msisr`] module*/
pub type MSISR = crate::Reg<msisr::MSISRrs>;
///HSEM secure masked interrupt status register
pub mod msisr;
/**SECCFGR (rw) register accessor: HSEM security configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HSEM:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///HSEM security configuration register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: HSEM privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HSEM:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///HSEM privilege configuration register
pub mod privcfgr;
/**CR (w) register accessor: HSEM clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HSEM:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///HSEM clear register
pub mod cr;
/**KEYR (rw) register accessor: HSEM interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`keyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HSEM:KEYR)

For information about available fields see [`mod@keyr`] module*/
pub type KEYR = crate::Reg<keyr::KEYRrs>;
///HSEM interrupt clear register
pub mod keyr;
