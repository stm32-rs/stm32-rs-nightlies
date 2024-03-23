#[repr(C)]
#[doc = "Register block"]
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
    #[doc = "0x00 - Access control register"]
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    #[doc = "0x04 - Power down key register"]
    #[inline(always)]
    pub const fn pdkeyr(&self) -> &PDKEYR {
        &self.pdkeyr
    }
    #[doc = "0x08 - Flash non-secure key register"]
    #[inline(always)]
    pub const fn nskeyr(&self) -> &NSKEYR {
        &self.nskeyr
    }
    #[doc = "0x0c - Flash secure key register"]
    #[inline(always)]
    pub const fn seckeyr(&self) -> &SECKEYR {
        &self.seckeyr
    }
    #[doc = "0x10 - Flash option key register"]
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    #[doc = "0x14 - Flash low voltage key register"]
    #[inline(always)]
    pub const fn lvekeyr(&self) -> &LVEKEYR {
        &self.lvekeyr
    }
    #[doc = "0x20 - Flash status register"]
    #[inline(always)]
    pub const fn nssr(&self) -> &NSSR {
        &self.nssr
    }
    #[doc = "0x24 - Flash status register"]
    #[inline(always)]
    pub const fn secsr(&self) -> &SECSR {
        &self.secsr
    }
    #[doc = "0x28 - Flash non-secure control register"]
    #[inline(always)]
    pub const fn nscr(&self) -> &NSCR {
        &self.nscr
    }
    #[doc = "0x2c - Flash secure control register"]
    #[inline(always)]
    pub const fn seccr(&self) -> &SECCR {
        &self.seccr
    }
    #[doc = "0x30 - Flash ECC register"]
    #[inline(always)]
    pub const fn eccr(&self) -> &ECCR {
        &self.eccr
    }
    #[doc = "0x40 - Flash option register"]
    #[inline(always)]
    pub const fn optr(&self) -> &OPTR {
        &self.optr
    }
    #[doc = "0x44 - Flash non-secure boot address 0 register"]
    #[inline(always)]
    pub const fn nsbootadd0r(&self) -> &NSBOOTADD0R {
        &self.nsbootadd0r
    }
    #[doc = "0x48 - Flash non-secure boot address 1 register"]
    #[inline(always)]
    pub const fn nsbootadd1r(&self) -> &NSBOOTADD1R {
        &self.nsbootadd1r
    }
    #[doc = "0x4c - FFlash secure boot address 0 register"]
    #[inline(always)]
    pub const fn secbootadd0r(&self) -> &SECBOOTADD0R {
        &self.secbootadd0r
    }
    #[doc = "0x50 - Flash bank 1 secure watermak1 register"]
    #[inline(always)]
    pub const fn secwm1r1(&self) -> &SECWM1R1 {
        &self.secwm1r1
    }
    #[doc = "0x54 - Flash secure watermak1 register 2"]
    #[inline(always)]
    pub const fn secwm1r2(&self) -> &SECWM1R2 {
        &self.secwm1r2
    }
    #[doc = "0x58 - Flash Bank 1 WRP area A address register"]
    #[inline(always)]
    pub const fn wrp1ar(&self) -> &WRP1AR {
        &self.wrp1ar
    }
    #[doc = "0x5c - Flash Bank 1 WRP area B address register"]
    #[inline(always)]
    pub const fn wrp1br(&self) -> &WRP1BR {
        &self.wrp1br
    }
    #[doc = "0x60 - Flash secure watermak2 register"]
    #[inline(always)]
    pub const fn secwm2r1(&self) -> &SECWM2R1 {
        &self.secwm2r1
    }
    #[doc = "0x64 - Flash secure watermak2 register2"]
    #[inline(always)]
    pub const fn secwm2r2(&self) -> &SECWM2R2 {
        &self.secwm2r2
    }
    #[doc = "0x68 - Flash WPR2 area A address register"]
    #[inline(always)]
    pub const fn wrp2ar(&self) -> &WRP2AR {
        &self.wrp2ar
    }
    #[doc = "0x6c - Flash WPR2 area B address register"]
    #[inline(always)]
    pub const fn wrp2br(&self) -> &WRP2BR {
        &self.wrp2br
    }
    #[doc = "0x80..0x90 - FLASH secure block based bank 1"]
    #[inline(always)]
    pub const fn secbb1r(&self, n: usize) -> &SECBB1R {
        &self.secbb1r[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x90 - FLASH secure block based bank 1"]
    #[inline(always)]
    pub fn secbb1r_iter(&self) -> impl Iterator<Item = &SECBB1R> {
        self.secbb1r.iter()
    }
    #[doc = "0x80 - FLASH secure block based bank 1"]
    #[inline(always)]
    pub const fn secbb1r1(&self) -> &SECBB1R {
        self.secbb1r(0)
    }
    #[doc = "0x84 - FLASH secure block based bank 1"]
    #[inline(always)]
    pub const fn secbb1r2(&self) -> &SECBB1R {
        self.secbb1r(1)
    }
    #[doc = "0x88 - FLASH secure block based bank 1"]
    #[inline(always)]
    pub const fn secbb1r3(&self) -> &SECBB1R {
        self.secbb1r(2)
    }
    #[doc = "0x8c - FLASH secure block based bank 1"]
    #[inline(always)]
    pub const fn secbb1r4(&self) -> &SECBB1R {
        self.secbb1r(3)
    }
    #[doc = "0xa0..0xb0 - FLASH secure block based bank 2"]
    #[inline(always)]
    pub const fn secbb2r(&self, n: usize) -> &SECBB2R {
        &self.secbb2r[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0xb0 - FLASH secure block based bank 2"]
    #[inline(always)]
    pub fn secbb2r_iter(&self) -> impl Iterator<Item = &SECBB2R> {
        self.secbb2r.iter()
    }
    #[doc = "0xa0 - FLASH secure block based bank 2"]
    #[inline(always)]
    pub const fn secbb2r1(&self) -> &SECBB2R {
        self.secbb2r(0)
    }
    #[doc = "0xa4 - FLASH secure block based bank 2"]
    #[inline(always)]
    pub const fn secbb2r2(&self) -> &SECBB2R {
        self.secbb2r(1)
    }
    #[doc = "0xa8 - FLASH secure block based bank 2"]
    #[inline(always)]
    pub const fn secbb2r3(&self) -> &SECBB2R {
        self.secbb2r(2)
    }
    #[doc = "0xac - FLASH secure block based bank 2"]
    #[inline(always)]
    pub const fn secbb2r4(&self) -> &SECBB2R {
        self.secbb2r(3)
    }
    #[doc = "0xc0 - FLASH secure HDP control register"]
    #[inline(always)]
    pub const fn sechdpcr(&self) -> &SECHDPCR {
        &self.sechdpcr
    }
    #[doc = "0xc4 - Power privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
}
#[doc = "ACR (rw) register accessor: Access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
pub type ACR = crate::Reg<acr::ACRrs>;
#[doc = "Access control register"]
pub mod acr;
#[doc = "PDKEYR (w) register accessor: Power down key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdkeyr`]
module"]
pub type PDKEYR = crate::Reg<pdkeyr::PDKEYRrs>;
#[doc = "Power down key register"]
pub mod pdkeyr;
#[doc = "NSKEYR (w) register accessor: Flash non-secure key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nskeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nskeyr`]
module"]
pub type NSKEYR = crate::Reg<nskeyr::NSKEYRrs>;
#[doc = "Flash non-secure key register"]
pub mod nskeyr;
#[doc = "SECKEYR (w) register accessor: Flash secure key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seckeyr`]
module"]
pub type SECKEYR = crate::Reg<seckeyr::SECKEYRrs>;
#[doc = "Flash secure key register"]
pub mod seckeyr;
#[doc = "OPTKEYR (w) register accessor: Flash option key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr`]
module"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
#[doc = "Flash option key register"]
pub mod optkeyr;
#[doc = "LVEKEYR (w) register accessor: Flash low voltage key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvekeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvekeyr`]
module"]
pub type LVEKEYR = crate::Reg<lvekeyr::LVEKEYRrs>;
#[doc = "Flash low voltage key register"]
pub mod lvekeyr;
#[doc = "NSSR (rw) register accessor: Flash status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nssr`]
module"]
pub type NSSR = crate::Reg<nssr::NSSRrs>;
#[doc = "Flash status register"]
pub mod nssr;
#[doc = "SECSR (rw) register accessor: Flash status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secsr`]
module"]
pub type SECSR = crate::Reg<secsr::SECSRrs>;
#[doc = "Flash status register"]
pub mod secsr;
#[doc = "NSCR (rw) register accessor: Flash non-secure control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nscr`]
module"]
pub type NSCR = crate::Reg<nscr::NSCRrs>;
#[doc = "Flash non-secure control register"]
pub mod nscr;
#[doc = "SECCR (rw) register accessor: Flash secure control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seccr`]
module"]
pub type SECCR = crate::Reg<seccr::SECCRrs>;
#[doc = "Flash secure control register"]
pub mod seccr;
#[doc = "ECCR (rw) register accessor: Flash ECC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccr`]
module"]
pub type ECCR = crate::Reg<eccr::ECCRrs>;
#[doc = "Flash ECC register"]
pub mod eccr;
#[doc = "OPTR (rw) register accessor: Flash option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optr`]
module"]
pub type OPTR = crate::Reg<optr::OPTRrs>;
#[doc = "Flash option register"]
pub mod optr;
#[doc = "NSBOOTADD0R (w) register accessor: Flash non-secure boot address 0 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsbootadd0r::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsbootadd0r`]
module"]
pub type NSBOOTADD0R = crate::Reg<nsbootadd0r::NSBOOTADD0Rrs>;
#[doc = "Flash non-secure boot address 0 register"]
pub mod nsbootadd0r;
#[doc = "NSBOOTADD1R (w) register accessor: Flash non-secure boot address 1 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsbootadd1r::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsbootadd1r`]
module"]
pub type NSBOOTADD1R = crate::Reg<nsbootadd1r::NSBOOTADD1Rrs>;
#[doc = "Flash non-secure boot address 1 register"]
pub mod nsbootadd1r;
#[doc = "SECBOOTADD0R (rw) register accessor: FFlash secure boot address 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secbootadd0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secbootadd0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secbootadd0r`]
module"]
pub type SECBOOTADD0R = crate::Reg<secbootadd0r::SECBOOTADD0Rrs>;
#[doc = "FFlash secure boot address 0 register"]
pub mod secbootadd0r;
#[doc = "SECWM1R1 (rw) register accessor: Flash bank 1 secure watermak1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secwm1r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secwm1r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secwm1r1`]
module"]
pub type SECWM1R1 = crate::Reg<secwm1r1::SECWM1R1rs>;
#[doc = "Flash bank 1 secure watermak1 register"]
pub mod secwm1r1;
#[doc = "SECWM1R2 (rw) register accessor: Flash secure watermak1 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secwm1r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secwm1r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secwm1r2`]
module"]
pub type SECWM1R2 = crate::Reg<secwm1r2::SECWM1R2rs>;
#[doc = "Flash secure watermak1 register 2"]
pub mod secwm1r2;
#[doc = "WRP1AR (rw) register accessor: Flash Bank 1 WRP area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp1ar`]
module"]
pub type WRP1AR = crate::Reg<wrp1ar::WRP1ARrs>;
#[doc = "Flash Bank 1 WRP area A address register"]
pub mod wrp1ar;
#[doc = "WRP1BR (rw) register accessor: Flash Bank 1 WRP area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp1br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp1br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp1br`]
module"]
pub type WRP1BR = crate::Reg<wrp1br::WRP1BRrs>;
#[doc = "Flash Bank 1 WRP area B address register"]
pub mod wrp1br;
#[doc = "SECWM2R1 (rw) register accessor: Flash secure watermak2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secwm2r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secwm2r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secwm2r1`]
module"]
pub type SECWM2R1 = crate::Reg<secwm2r1::SECWM2R1rs>;
#[doc = "Flash secure watermak2 register"]
pub mod secwm2r1;
#[doc = "SECWM2R2 (rw) register accessor: Flash secure watermak2 register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secwm2r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secwm2r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secwm2r2`]
module"]
pub type SECWM2R2 = crate::Reg<secwm2r2::SECWM2R2rs>;
#[doc = "Flash secure watermak2 register2"]
pub mod secwm2r2;
#[doc = "WRP2AR (rw) register accessor: Flash WPR2 area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp2ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp2ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp2ar`]
module"]
pub type WRP2AR = crate::Reg<wrp2ar::WRP2ARrs>;
#[doc = "Flash WPR2 area A address register"]
pub mod wrp2ar;
#[doc = "WRP2BR (rw) register accessor: Flash WPR2 area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp2br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp2br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrp2br`]
module"]
pub type WRP2BR = crate::Reg<wrp2br::WRP2BRrs>;
#[doc = "Flash WPR2 area B address register"]
pub mod wrp2br;
#[doc = "SECBB1R (rw) register accessor: FLASH secure block based bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secbb1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secbb1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secbb1r`]
module"]
pub type SECBB1R = crate::Reg<secbb1r::SECBB1Rrs>;
#[doc = "FLASH secure block based bank 1"]
pub mod secbb1r;
#[doc = "SECBB2R (rw) register accessor: FLASH secure block based bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secbb2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secbb2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secbb2r`]
module"]
pub type SECBB2R = crate::Reg<secbb2r::SECBB2Rrs>;
#[doc = "FLASH secure block based bank 2"]
pub mod secbb2r;
#[doc = "SECHDPCR (rw) register accessor: FLASH secure HDP control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sechdpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sechdpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sechdpcr`]
module"]
pub type SECHDPCR = crate::Reg<sechdpcr::SECHDPCRrs>;
#[doc = "FLASH secure HDP control register"]
pub mod sechdpcr;
#[doc = "PRIVCFGR (rw) register accessor: Power privilege configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr`]
module"]
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
#[doc = "Power privilege configuration register"]
pub mod privcfgr;
