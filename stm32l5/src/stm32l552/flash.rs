#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    acr: ACR,
    pdkeyr: PDKEYR,
    nskeyr: NSKEYR,
    seckeyr: SECKEYR,
    optkeyr: OPTKEYR,
    lvekeyr: LVEKEYR,
    _reserved6: [u8; 0x08],
    nssr: NSSR,
    secsr: SECSR,
    nscr: NSCR,
    seccr: SECCR,
    eccr: ECCR,
    _reserved11: [u8; 0x0c],
    optr: OPTR,
    nsbootadd0r: NSBOOTADD0R,
    nsbootadd1r: NSBOOTADD1R,
    secbootadd0r: SECBOOTADD0R,
    secwm1r1: SECWM1R1,
    secwm1r2: SECWM1R2,
    wrp1ar: WRP1AR,
    wrp1br: WRP1BR,
    secwm2r1: SECWM2R1,
    secwm2r2: SECWM2R2,
    wrp2ar: WRP2AR,
    wrp2br: WRP2BR,
    _reserved23: [u8; 0x10],
    secbb1r: [SECBB1R; 4],
    _reserved24: [u8; 0x10],
    secbb2r: [SECBB2R; 4],
    _reserved25: [u8; 0x10],
    sechdpcr: SECHDPCR,
    privcfgr: PRIVCFGR,
}
impl RegisterBlock {
    ///0x00 - Access control register
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    ///0x04 - Power down key register
    #[inline(always)]
    pub const fn pdkeyr(&self) -> &PDKEYR {
        &self.pdkeyr
    }
    ///0x08 - Flash non-secure key register
    #[inline(always)]
    pub const fn nskeyr(&self) -> &NSKEYR {
        &self.nskeyr
    }
    ///0x0c - Flash secure key register
    #[inline(always)]
    pub const fn seckeyr(&self) -> &SECKEYR {
        &self.seckeyr
    }
    ///0x10 - Flash option key register
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    ///0x14 - Flash low voltage key register
    #[inline(always)]
    pub const fn lvekeyr(&self) -> &LVEKEYR {
        &self.lvekeyr
    }
    ///0x20 - Flash status register
    #[inline(always)]
    pub const fn nssr(&self) -> &NSSR {
        &self.nssr
    }
    ///0x24 - Flash status register
    #[inline(always)]
    pub const fn secsr(&self) -> &SECSR {
        &self.secsr
    }
    ///0x28 - Flash non-secure control register
    #[inline(always)]
    pub const fn nscr(&self) -> &NSCR {
        &self.nscr
    }
    ///0x2c - Flash secure control register
    #[inline(always)]
    pub const fn seccr(&self) -> &SECCR {
        &self.seccr
    }
    ///0x30 - Flash ECC register
    #[inline(always)]
    pub const fn eccr(&self) -> &ECCR {
        &self.eccr
    }
    ///0x40 - Flash option register
    #[inline(always)]
    pub const fn optr(&self) -> &OPTR {
        &self.optr
    }
    ///0x44 - Flash non-secure boot address 0 register
    #[inline(always)]
    pub const fn nsbootadd0r(&self) -> &NSBOOTADD0R {
        &self.nsbootadd0r
    }
    ///0x48 - Flash non-secure boot address 1 register
    #[inline(always)]
    pub const fn nsbootadd1r(&self) -> &NSBOOTADD1R {
        &self.nsbootadd1r
    }
    ///0x4c - FFlash secure boot address 0 register
    #[inline(always)]
    pub const fn secbootadd0r(&self) -> &SECBOOTADD0R {
        &self.secbootadd0r
    }
    ///0x50 - Flash bank 1 secure watermak1 register
    #[inline(always)]
    pub const fn secwm1r1(&self) -> &SECWM1R1 {
        &self.secwm1r1
    }
    ///0x54 - Flash secure watermak1 register 2
    #[inline(always)]
    pub const fn secwm1r2(&self) -> &SECWM1R2 {
        &self.secwm1r2
    }
    ///0x58 - Flash Bank 1 WRP area A address register
    #[inline(always)]
    pub const fn wrp1ar(&self) -> &WRP1AR {
        &self.wrp1ar
    }
    ///0x5c - Flash Bank 1 WRP area B address register
    #[inline(always)]
    pub const fn wrp1br(&self) -> &WRP1BR {
        &self.wrp1br
    }
    ///0x60 - Flash secure watermak2 register
    #[inline(always)]
    pub const fn secwm2r1(&self) -> &SECWM2R1 {
        &self.secwm2r1
    }
    ///0x64 - Flash secure watermak2 register2
    #[inline(always)]
    pub const fn secwm2r2(&self) -> &SECWM2R2 {
        &self.secwm2r2
    }
    ///0x68 - Flash WPR2 area A address register
    #[inline(always)]
    pub const fn wrp2ar(&self) -> &WRP2AR {
        &self.wrp2ar
    }
    ///0x6c - Flash WPR2 area B address register
    #[inline(always)]
    pub const fn wrp2br(&self) -> &WRP2BR {
        &self.wrp2br
    }
    ///0x80..0x90 - FLASH secure block based bank 1
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `SECBB1R1` register.</div>
    #[inline(always)]
    pub const fn secbb1r(&self, n: usize) -> &SECBB1R {
        &self.secbb1r[n]
    }
    ///Iterator for array of:
    ///0x80..0x90 - FLASH secure block based bank 1
    #[inline(always)]
    pub fn secbb1r_iter(&self) -> impl Iterator<Item = &SECBB1R> {
        self.secbb1r.iter()
    }
    ///0x80 - FLASH secure block based bank 1
    #[inline(always)]
    pub const fn secbb1r1(&self) -> &SECBB1R {
        self.secbb1r(0)
    }
    ///0x84 - FLASH secure block based bank 1
    #[inline(always)]
    pub const fn secbb1r2(&self) -> &SECBB1R {
        self.secbb1r(1)
    }
    ///0x88 - FLASH secure block based bank 1
    #[inline(always)]
    pub const fn secbb1r3(&self) -> &SECBB1R {
        self.secbb1r(2)
    }
    ///0x8c - FLASH secure block based bank 1
    #[inline(always)]
    pub const fn secbb1r4(&self) -> &SECBB1R {
        self.secbb1r(3)
    }
    ///0xa0..0xb0 - FLASH secure block based bank 2
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `SECBB2R1` register.</div>
    #[inline(always)]
    pub const fn secbb2r(&self, n: usize) -> &SECBB2R {
        &self.secbb2r[n]
    }
    ///Iterator for array of:
    ///0xa0..0xb0 - FLASH secure block based bank 2
    #[inline(always)]
    pub fn secbb2r_iter(&self) -> impl Iterator<Item = &SECBB2R> {
        self.secbb2r.iter()
    }
    ///0xa0 - FLASH secure block based bank 2
    #[inline(always)]
    pub const fn secbb2r1(&self) -> &SECBB2R {
        self.secbb2r(0)
    }
    ///0xa4 - FLASH secure block based bank 2
    #[inline(always)]
    pub const fn secbb2r2(&self) -> &SECBB2R {
        self.secbb2r(1)
    }
    ///0xa8 - FLASH secure block based bank 2
    #[inline(always)]
    pub const fn secbb2r3(&self) -> &SECBB2R {
        self.secbb2r(2)
    }
    ///0xac - FLASH secure block based bank 2
    #[inline(always)]
    pub const fn secbb2r4(&self) -> &SECBB2R {
        self.secbb2r(3)
    }
    ///0xc0 - FLASH secure HDP control register
    #[inline(always)]
    pub const fn sechdpcr(&self) -> &SECHDPCR {
        &self.sechdpcr
    }
    ///0xc4 - Power privilege configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
}
/**ACR (rw) register accessor: Access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:ACR)

For information about available fields see [`mod@acr`] module*/
pub type ACR = crate::Reg<acr::ACRrs>;
///Access control register
pub mod acr;
/**PDKEYR (w) register accessor: Power down key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:PDKEYR)

For information about available fields see [`mod@pdkeyr`] module*/
pub type PDKEYR = crate::Reg<pdkeyr::PDKEYRrs>;
///Power down key register
pub mod pdkeyr;
/**NSKEYR (w) register accessor: Flash non-secure key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nskeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:NSKEYR)

For information about available fields see [`mod@nskeyr`] module*/
pub type NSKEYR = crate::Reg<nskeyr::NSKEYRrs>;
///Flash non-secure key register
pub mod nskeyr;
/**SECKEYR (w) register accessor: Flash secure key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:SECKEYR)

For information about available fields see [`mod@seckeyr`] module*/
pub type SECKEYR = crate::Reg<seckeyr::SECKEYRrs>;
///Flash secure key register
pub mod seckeyr;
/**OPTKEYR (w) register accessor: Flash option key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:OPTKEYR)

For information about available fields see [`mod@optkeyr`] module*/
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
///Flash option key register
pub mod optkeyr;
/**LVEKEYR (w) register accessor: Flash low voltage key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvekeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:LVEKEYR)

For information about available fields see [`mod@lvekeyr`] module*/
pub type LVEKEYR = crate::Reg<lvekeyr::LVEKEYRrs>;
///Flash low voltage key register
pub mod lvekeyr;
/**NSSR (rw) register accessor: Flash status register

You can [`read`](crate::Reg::read) this register and get [`nssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:NSSR)

For information about available fields see [`mod@nssr`] module*/
pub type NSSR = crate::Reg<nssr::NSSRrs>;
///Flash status register
pub mod nssr;
/**SECSR (rw) register accessor: Flash status register

You can [`read`](crate::Reg::read) this register and get [`secsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:SECSR)

For information about available fields see [`mod@secsr`] module*/
pub type SECSR = crate::Reg<secsr::SECSRrs>;
///Flash status register
pub mod secsr;
/**NSCR (rw) register accessor: Flash non-secure control register

You can [`read`](crate::Reg::read) this register and get [`nscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:NSCR)

For information about available fields see [`mod@nscr`] module*/
pub type NSCR = crate::Reg<nscr::NSCRrs>;
///Flash non-secure control register
pub mod nscr;
/**SECCR (rw) register accessor: Flash secure control register

You can [`read`](crate::Reg::read) this register and get [`seccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:SECCR)

For information about available fields see [`mod@seccr`] module*/
pub type SECCR = crate::Reg<seccr::SECCRrs>;
///Flash secure control register
pub mod seccr;
/**ECCR (rw) register accessor: Flash ECC register

You can [`read`](crate::Reg::read) this register and get [`eccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:ECCR)

For information about available fields see [`mod@eccr`] module*/
pub type ECCR = crate::Reg<eccr::ECCRrs>;
///Flash ECC register
pub mod eccr;
/**OPTR (rw) register accessor: Flash option register

You can [`read`](crate::Reg::read) this register and get [`optr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:OPTR)

For information about available fields see [`mod@optr`] module*/
pub type OPTR = crate::Reg<optr::OPTRrs>;
///Flash option register
pub mod optr;
/**NSBOOTADD0R (w) register accessor: Flash non-secure boot address 0 register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsbootadd0r::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:NSBOOTADD0R)

For information about available fields see [`mod@nsbootadd0r`] module*/
pub type NSBOOTADD0R = crate::Reg<nsbootadd0r::NSBOOTADD0Rrs>;
///Flash non-secure boot address 0 register
pub mod nsbootadd0r;
/**NSBOOTADD1R (w) register accessor: Flash non-secure boot address 1 register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsbootadd1r::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:NSBOOTADD1R)

For information about available fields see [`mod@nsbootadd1r`] module*/
pub type NSBOOTADD1R = crate::Reg<nsbootadd1r::NSBOOTADD1Rrs>;
///Flash non-secure boot address 1 register
pub mod nsbootadd1r;
/**SECBOOTADD0R (rw) register accessor: FFlash secure boot address 0 register

You can [`read`](crate::Reg::read) this register and get [`secbootadd0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbootadd0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:SECBOOTADD0R)

For information about available fields see [`mod@secbootadd0r`] module*/
pub type SECBOOTADD0R = crate::Reg<secbootadd0r::SECBOOTADD0Rrs>;
///FFlash secure boot address 0 register
pub mod secbootadd0r;
/**SECWM1R1 (rw) register accessor: Flash bank 1 secure watermak1 register

You can [`read`](crate::Reg::read) this register and get [`secwm1r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwm1r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:SECWM1R1)

For information about available fields see [`mod@secwm1r1`] module*/
pub type SECWM1R1 = crate::Reg<secwm1r1::SECWM1R1rs>;
///Flash bank 1 secure watermak1 register
pub mod secwm1r1;
/**SECWM1R2 (rw) register accessor: Flash secure watermak1 register 2

You can [`read`](crate::Reg::read) this register and get [`secwm1r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwm1r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:SECWM1R2)

For information about available fields see [`mod@secwm1r2`] module*/
pub type SECWM1R2 = crate::Reg<secwm1r2::SECWM1R2rs>;
///Flash secure watermak1 register 2
pub mod secwm1r2;
/**WRP1AR (rw) register accessor: Flash Bank 1 WRP area A address register

You can [`read`](crate::Reg::read) this register and get [`wrp1ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:WRP1AR)

For information about available fields see [`mod@wrp1ar`] module*/
pub type WRP1AR = crate::Reg<wrp1ar::WRP1ARrs>;
///Flash Bank 1 WRP area A address register
pub mod wrp1ar;
/**WRP1BR (rw) register accessor: Flash Bank 1 WRP area B address register

You can [`read`](crate::Reg::read) this register and get [`wrp1br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:WRP1BR)

For information about available fields see [`mod@wrp1br`] module*/
pub type WRP1BR = crate::Reg<wrp1br::WRP1BRrs>;
///Flash Bank 1 WRP area B address register
pub mod wrp1br;
/**SECWM2R1 (rw) register accessor: Flash secure watermak2 register

You can [`read`](crate::Reg::read) this register and get [`secwm2r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwm2r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:SECWM2R1)

For information about available fields see [`mod@secwm2r1`] module*/
pub type SECWM2R1 = crate::Reg<secwm2r1::SECWM2R1rs>;
///Flash secure watermak2 register
pub mod secwm2r1;
/**SECWM2R2 (rw) register accessor: Flash secure watermak2 register2

You can [`read`](crate::Reg::read) this register and get [`secwm2r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwm2r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:SECWM2R2)

For information about available fields see [`mod@secwm2r2`] module*/
pub type SECWM2R2 = crate::Reg<secwm2r2::SECWM2R2rs>;
///Flash secure watermak2 register2
pub mod secwm2r2;
/**WRP2AR (rw) register accessor: Flash WPR2 area A address register

You can [`read`](crate::Reg::read) this register and get [`wrp2ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:WRP2AR)

For information about available fields see [`mod@wrp2ar`] module*/
pub type WRP2AR = crate::Reg<wrp2ar::WRP2ARrs>;
///Flash WPR2 area A address register
pub mod wrp2ar;
/**WRP2BR (rw) register accessor: Flash WPR2 area B address register

You can [`read`](crate::Reg::read) this register and get [`wrp2br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:WRP2BR)

For information about available fields see [`mod@wrp2br`] module*/
pub type WRP2BR = crate::Reg<wrp2br::WRP2BRrs>;
///Flash WPR2 area B address register
pub mod wrp2br;
/**SECBB1R (rw) register accessor: FLASH secure block based bank 1

You can [`read`](crate::Reg::read) this register and get [`secbb1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbb1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:SECBB1R[1])

For information about available fields see [`mod@secbb1r`] module*/
pub type SECBB1R = crate::Reg<secbb1r::SECBB1Rrs>;
///FLASH secure block based bank 1
pub mod secbb1r;
/**SECBB2R (rw) register accessor: FLASH secure block based bank 2

You can [`read`](crate::Reg::read) this register and get [`secbb2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbb2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:SECBB2R[1])

For information about available fields see [`mod@secbb2r`] module*/
pub type SECBB2R = crate::Reg<secbb2r::SECBB2Rrs>;
///FLASH secure block based bank 2
pub mod secbb2r;
/**SECHDPCR (rw) register accessor: FLASH secure HDP control register

You can [`read`](crate::Reg::read) this register and get [`sechdpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sechdpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:SECHDPCR)

For information about available fields see [`mod@sechdpcr`] module*/
pub type SECHDPCR = crate::Reg<sechdpcr::SECHDPCRrs>;
///FLASH secure HDP control register
pub mod sechdpcr;
/**PRIVCFGR (rw) register accessor: Power privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///Power privilege configuration register
pub mod privcfgr;
