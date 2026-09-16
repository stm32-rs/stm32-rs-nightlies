#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    acr: ACR,
    _reserved1: [u8; 0x04],
    nskeyr: NSKEYR,
    seckeyr: SECKEYR,
    optkeyr: OPTKEYR,
    _reserved4: [u8; 0x04],
    pdkeyr: PDKEYR,
    _reserved5: [u8; 0x04],
    nssr: NSSR,
    secsr: SECSR,
    nscr1: NSCR1,
    seccr1: SECCR1,
    eccr: ECCR,
    opsr: OPSR,
    nscr2: NSCR2,
    seccr2: SECCR2,
    optr: OPTR,
    nsbootadd0r: NSBOOTADD0R,
    nsbootadd1r: NSBOOTADD1R,
    secbootadd0r: SECBOOTADD0R,
    secwmr1: SECWMR1,
    secwmr2: SECWMR2,
    wrpar: WRPAR,
    wrpbr: WRPBR,
    _reserved21: [u8; 0x10],
    oem1keyr1: OEM1KEYR1,
    oem1keyr2: OEM1KEYR2,
    oem2keyr1: OEM2KEYR1,
    oem2keyr2: OEM2KEYR2,
    secbbr1: SECBBR1,
    secbbr2: SECBBR2,
    _reserved27: [u8; 0x38],
    sechdpcr: SECHDPCR,
    prifcfgr: PRIFCFGR,
    _reserved29: [u8; 0x08],
    privbbr1: PRIVBBR1,
    privbbr2: PRIVBBR2,
    privbbr3: PRIVBBR3,
    privbbr4: PRIVBBR4,
}
impl RegisterBlock {
    ///0x00 - FLASH access control register
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    ///0x08 - FLASH key register
    #[inline(always)]
    pub const fn nskeyr(&self) -> &NSKEYR {
        &self.nskeyr
    }
    ///0x0c - FLASH secure key register
    #[inline(always)]
    pub const fn seckeyr(&self) -> &SECKEYR {
        &self.seckeyr
    }
    ///0x10 - FLASH option key register
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    ///0x18 - FLASH power-down key register
    #[inline(always)]
    pub const fn pdkeyr(&self) -> &PDKEYR {
        &self.pdkeyr
    }
    ///0x20 - FLASH status register
    #[inline(always)]
    pub const fn nssr(&self) -> &NSSR {
        &self.nssr
    }
    ///0x24 - FLASH secure status register
    #[inline(always)]
    pub const fn secsr(&self) -> &SECSR {
        &self.secsr
    }
    ///0x28 - FLASH control register
    #[inline(always)]
    pub const fn nscr1(&self) -> &NSCR1 {
        &self.nscr1
    }
    ///0x2c - FLASH secure control register
    #[inline(always)]
    pub const fn seccr1(&self) -> &SECCR1 {
        &self.seccr1
    }
    ///0x30 - FLASH ECC register
    #[inline(always)]
    pub const fn eccr(&self) -> &ECCR {
        &self.eccr
    }
    ///0x34 - FLASH operation status register
    #[inline(always)]
    pub const fn opsr(&self) -> &OPSR {
        &self.opsr
    }
    ///0x38 - FLASH control 2 register
    #[inline(always)]
    pub const fn nscr2(&self) -> &NSCR2 {
        &self.nscr2
    }
    ///0x3c - FLASH secure control 2 register
    #[inline(always)]
    pub const fn seccr2(&self) -> &SECCR2 {
        &self.seccr2
    }
    ///0x40 - FLASH option register
    #[inline(always)]
    pub const fn optr(&self) -> &OPTR {
        &self.optr
    }
    ///0x44 - FLASH boot address 0 register
    #[inline(always)]
    pub const fn nsbootadd0r(&self) -> &NSBOOTADD0R {
        &self.nsbootadd0r
    }
    ///0x48 - FLASH boot address 1 register
    #[inline(always)]
    pub const fn nsbootadd1r(&self) -> &NSBOOTADD1R {
        &self.nsbootadd1r
    }
    ///0x4c - FLASH secure boot address 0 register
    #[inline(always)]
    pub const fn secbootadd0r(&self) -> &SECBOOTADD0R {
        &self.secbootadd0r
    }
    ///0x50 - FLASH secure watermark register 1
    #[inline(always)]
    pub const fn secwmr1(&self) -> &SECWMR1 {
        &self.secwmr1
    }
    ///0x54 - FLASH secure watermark register 2
    #[inline(always)]
    pub const fn secwmr2(&self) -> &SECWMR2 {
        &self.secwmr2
    }
    ///0x58 - FLASH WRP area A address register
    #[inline(always)]
    pub const fn wrpar(&self) -> &WRPAR {
        &self.wrpar
    }
    ///0x5c - FLASH WRP area B address register
    #[inline(always)]
    pub const fn wrpbr(&self) -> &WRPBR {
        &self.wrpbr
    }
    ///0x70 - FLASH OEM1 key register 1
    #[inline(always)]
    pub const fn oem1keyr1(&self) -> &OEM1KEYR1 {
        &self.oem1keyr1
    }
    ///0x74 - FLASH OEM1 key register 2
    #[inline(always)]
    pub const fn oem1keyr2(&self) -> &OEM1KEYR2 {
        &self.oem1keyr2
    }
    ///0x78 - FLASH OEM2 key register 1
    #[inline(always)]
    pub const fn oem2keyr1(&self) -> &OEM2KEYR1 {
        &self.oem2keyr1
    }
    ///0x7c - FLASH OEM2 key register 2
    #[inline(always)]
    pub const fn oem2keyr2(&self) -> &OEM2KEYR2 {
        &self.oem2keyr2
    }
    ///0x80 - FLASH secure block based register 1
    #[inline(always)]
    pub const fn secbbr1(&self) -> &SECBBR1 {
        &self.secbbr1
    }
    ///0x84 - FLASH secure block based register 2
    #[inline(always)]
    pub const fn secbbr2(&self) -> &SECBBR2 {
        &self.secbbr2
    }
    ///0xc0 - FLASH secure HDP control register
    #[inline(always)]
    pub const fn sechdpcr(&self) -> &SECHDPCR {
        &self.sechdpcr
    }
    ///0xc4 - FLASH privilege configuration register
    #[inline(always)]
    pub const fn prifcfgr(&self) -> &PRIFCFGR {
        &self.prifcfgr
    }
    ///0xd0 - FLASH privilege block based register 1
    #[inline(always)]
    pub const fn privbbr1(&self) -> &PRIVBBR1 {
        &self.privbbr1
    }
    ///0xd4 - FLASH privilege block based register 2
    #[inline(always)]
    pub const fn privbbr2(&self) -> &PRIVBBR2 {
        &self.privbbr2
    }
    ///0xd8 - FLASH privilege block based register 3
    #[inline(always)]
    pub const fn privbbr3(&self) -> &PRIVBBR3 {
        &self.privbbr3
    }
    ///0xdc - FLASH privilege block based register 4
    #[inline(always)]
    pub const fn privbbr4(&self) -> &PRIVBBR4 {
        &self.privbbr4
    }
}
/**ACR (rw) register accessor: FLASH access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:ACR)

For information about available fields see [`mod@acr`] module*/
pub type ACR = crate::Reg<acr::ACRrs>;
///FLASH access control register
pub mod acr;
/**NSKEYR (w) register accessor: FLASH key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nskeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:NSKEYR)

For information about available fields see [`mod@nskeyr`] module*/
pub type NSKEYR = crate::Reg<nskeyr::NSKEYRrs>;
///FLASH key register
pub mod nskeyr;
/**SECKEYR (w) register accessor: FLASH secure key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:SECKEYR)

For information about available fields see [`mod@seckeyr`] module*/
pub type SECKEYR = crate::Reg<seckeyr::SECKEYRrs>;
///FLASH secure key register
pub mod seckeyr;
/**OPTKEYR (w) register accessor: FLASH option key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:OPTKEYR)

For information about available fields see [`mod@optkeyr`] module*/
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
///FLASH option key register
pub mod optkeyr;
/**PDKEYR (w) register accessor: FLASH power-down key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:PDKEYR)

For information about available fields see [`mod@pdkeyr`] module*/
pub type PDKEYR = crate::Reg<pdkeyr::PDKEYRrs>;
///FLASH power-down key register
pub mod pdkeyr;
/**NSSR (rw) register accessor: FLASH status register

You can [`read`](crate::Reg::read) this register and get [`nssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:NSSR)

For information about available fields see [`mod@nssr`] module*/
pub type NSSR = crate::Reg<nssr::NSSRrs>;
///FLASH status register
pub mod nssr;
/**SECSR (rw) register accessor: FLASH secure status register

You can [`read`](crate::Reg::read) this register and get [`secsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:SECSR)

For information about available fields see [`mod@secsr`] module*/
pub type SECSR = crate::Reg<secsr::SECSRrs>;
///FLASH secure status register
pub mod secsr;
/**NSCR1 (rw) register accessor: FLASH control register

You can [`read`](crate::Reg::read) this register and get [`nscr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nscr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:NSCR1)

For information about available fields see [`mod@nscr1`] module*/
pub type NSCR1 = crate::Reg<nscr1::NSCR1rs>;
///FLASH control register
pub mod nscr1;
/**SECCR1 (rw) register accessor: FLASH secure control register

You can [`read`](crate::Reg::read) this register and get [`seccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:SECCR1)

For information about available fields see [`mod@seccr1`] module*/
pub type SECCR1 = crate::Reg<seccr1::SECCR1rs>;
///FLASH secure control register
pub mod seccr1;
/**ECCR (rw) register accessor: FLASH ECC register

You can [`read`](crate::Reg::read) this register and get [`eccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:ECCR)

For information about available fields see [`mod@eccr`] module*/
pub type ECCR = crate::Reg<eccr::ECCRrs>;
///FLASH ECC register
pub mod eccr;
/**OPSR (r) register accessor: FLASH operation status register

You can [`read`](crate::Reg::read) this register and get [`opsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:OPSR)

For information about available fields see [`mod@opsr`] module*/
pub type OPSR = crate::Reg<opsr::OPSRrs>;
///FLASH operation status register
pub mod opsr;
/**NSCR2 (rw) register accessor: FLASH control 2 register

You can [`read`](crate::Reg::read) this register and get [`nscr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nscr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:NSCR2)

For information about available fields see [`mod@nscr2`] module*/
pub type NSCR2 = crate::Reg<nscr2::NSCR2rs>;
///FLASH control 2 register
pub mod nscr2;
/**SECCR2 (rw) register accessor: FLASH secure control 2 register

You can [`read`](crate::Reg::read) this register and get [`seccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:SECCR2)

For information about available fields see [`mod@seccr2`] module*/
pub type SECCR2 = crate::Reg<seccr2::SECCR2rs>;
///FLASH secure control 2 register
pub mod seccr2;
/**OPTR (rw) register accessor: FLASH option register

You can [`read`](crate::Reg::read) this register and get [`optr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:OPTR)

For information about available fields see [`mod@optr`] module*/
pub type OPTR = crate::Reg<optr::OPTRrs>;
///FLASH option register
pub mod optr;
/**NSBOOTADD0R (rw) register accessor: FLASH boot address 0 register

You can [`read`](crate::Reg::read) this register and get [`nsbootadd0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsbootadd0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:NSBOOTADD0R)

For information about available fields see [`mod@nsbootadd0r`] module*/
pub type NSBOOTADD0R = crate::Reg<nsbootadd0r::NSBOOTADD0Rrs>;
///FLASH boot address 0 register
pub mod nsbootadd0r;
/**NSBOOTADD1R (rw) register accessor: FLASH boot address 1 register

You can [`read`](crate::Reg::read) this register and get [`nsbootadd1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsbootadd1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:NSBOOTADD1R)

For information about available fields see [`mod@nsbootadd1r`] module*/
pub type NSBOOTADD1R = crate::Reg<nsbootadd1r::NSBOOTADD1Rrs>;
///FLASH boot address 1 register
pub mod nsbootadd1r;
/**SECBOOTADD0R (rw) register accessor: FLASH secure boot address 0 register

You can [`read`](crate::Reg::read) this register and get [`secbootadd0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbootadd0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:SECBOOTADD0R)

For information about available fields see [`mod@secbootadd0r`] module*/
pub type SECBOOTADD0R = crate::Reg<secbootadd0r::SECBOOTADD0Rrs>;
///FLASH secure boot address 0 register
pub mod secbootadd0r;
/**SECWMR1 (rw) register accessor: FLASH secure watermark register 1

You can [`read`](crate::Reg::read) this register and get [`secwmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:SECWMR1)

For information about available fields see [`mod@secwmr1`] module*/
pub type SECWMR1 = crate::Reg<secwmr1::SECWMR1rs>;
///FLASH secure watermark register 1
pub mod secwmr1;
/**SECWMR2 (rw) register accessor: FLASH secure watermark register 2

You can [`read`](crate::Reg::read) this register and get [`secwmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:SECWMR2)

For information about available fields see [`mod@secwmr2`] module*/
pub type SECWMR2 = crate::Reg<secwmr2::SECWMR2rs>;
///FLASH secure watermark register 2
pub mod secwmr2;
/**WRPAR (rw) register accessor: FLASH WRP area A address register

You can [`read`](crate::Reg::read) this register and get [`wrpar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:WRPAR)

For information about available fields see [`mod@wrpar`] module*/
pub type WRPAR = crate::Reg<wrpar::WRPARrs>;
///FLASH WRP area A address register
pub mod wrpar;
/**WRPBR (rw) register accessor: FLASH WRP area B address register

You can [`read`](crate::Reg::read) this register and get [`wrpbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:WRPBR)

For information about available fields see [`mod@wrpbr`] module*/
pub type WRPBR = crate::Reg<wrpbr::WRPBRrs>;
///FLASH WRP area B address register
pub mod wrpbr;
/**OEM1KEYR1 (w) register accessor: FLASH OEM1 key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oem1keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:OEM1KEYR1)

For information about available fields see [`mod@oem1keyr1`] module*/
pub type OEM1KEYR1 = crate::Reg<oem1keyr1::OEM1KEYR1rs>;
///FLASH OEM1 key register 1
pub mod oem1keyr1;
/**OEM1KEYR2 (w) register accessor: FLASH OEM1 key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oem1keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:OEM1KEYR2)

For information about available fields see [`mod@oem1keyr2`] module*/
pub type OEM1KEYR2 = crate::Reg<oem1keyr2::OEM1KEYR2rs>;
///FLASH OEM1 key register 2
pub mod oem1keyr2;
/**OEM2KEYR1 (w) register accessor: FLASH OEM2 key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oem2keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:OEM2KEYR1)

For information about available fields see [`mod@oem2keyr1`] module*/
pub type OEM2KEYR1 = crate::Reg<oem2keyr1::OEM2KEYR1rs>;
///FLASH OEM2 key register 1
pub mod oem2keyr1;
/**OEM2KEYR2 (w) register accessor: FLASH OEM2 key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oem2keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:OEM2KEYR2)

For information about available fields see [`mod@oem2keyr2`] module*/
pub type OEM2KEYR2 = crate::Reg<oem2keyr2::OEM2KEYR2rs>;
///FLASH OEM2 key register 2
pub mod oem2keyr2;
/**SECBBR1 (rw) register accessor: FLASH secure block based register 1

You can [`read`](crate::Reg::read) this register and get [`secbbr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbbr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:SECBBR1)

For information about available fields see [`mod@secbbr1`] module*/
pub type SECBBR1 = crate::Reg<secbbr1::SECBBR1rs>;
///FLASH secure block based register 1
pub mod secbbr1;
/**SECBBR2 (rw) register accessor: FLASH secure block based register 2

You can [`read`](crate::Reg::read) this register and get [`secbbr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbbr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:SECBBR2)

For information about available fields see [`mod@secbbr2`] module*/
pub type SECBBR2 = crate::Reg<secbbr2::SECBBR2rs>;
///FLASH secure block based register 2
pub mod secbbr2;
/**SECHDPCR (rw) register accessor: FLASH secure HDP control register

You can [`read`](crate::Reg::read) this register and get [`sechdpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sechdpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:SECHDPCR)

For information about available fields see [`mod@sechdpcr`] module*/
pub type SECHDPCR = crate::Reg<sechdpcr::SECHDPCRrs>;
///FLASH secure HDP control register
pub mod sechdpcr;
/**PRIFCFGR (rw) register accessor: FLASH privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`prifcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prifcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:PRIFCFGR)

For information about available fields see [`mod@prifcfgr`] module*/
pub type PRIFCFGR = crate::Reg<prifcfgr::PRIFCFGRrs>;
///FLASH privilege configuration register
pub mod prifcfgr;
/**PRIVBBR1 (rw) register accessor: FLASH privilege block based register 1

You can [`read`](crate::Reg::read) this register and get [`privbbr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbbr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:PRIVBBR1)

For information about available fields see [`mod@privbbr1`] module*/
pub type PRIVBBR1 = crate::Reg<privbbr1::PRIVBBR1rs>;
///FLASH privilege block based register 1
pub mod privbbr1;
/**PRIVBBR2 (rw) register accessor: FLASH privilege block based register 2

You can [`read`](crate::Reg::read) this register and get [`privbbr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbbr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:PRIVBBR2)

For information about available fields see [`mod@privbbr2`] module*/
pub type PRIVBBR2 = crate::Reg<privbbr2::PRIVBBR2rs>;
///FLASH privilege block based register 2
pub mod privbbr2;
/**PRIVBBR3 (rw) register accessor: FLASH privilege block based register 3

You can [`read`](crate::Reg::read) this register and get [`privbbr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbbr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:PRIVBBR3)

For information about available fields see [`mod@privbbr3`] module*/
pub type PRIVBBR3 = crate::Reg<privbbr3::PRIVBBR3rs>;
///FLASH privilege block based register 3
pub mod privbbr3;
/**PRIVBBR4 (rw) register accessor: FLASH privilege block based register 4

You can [`read`](crate::Reg::read) this register and get [`privbbr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbbr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:PRIVBBR4)

For information about available fields see [`mod@privbbr4`] module*/
pub type PRIVBBR4 = crate::Reg<privbbr4::PRIVBBR4rs>;
///FLASH privilege block based register 4
pub mod privbbr4;
