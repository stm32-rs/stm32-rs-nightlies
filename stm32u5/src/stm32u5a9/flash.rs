#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    flash_acr: FLASH_ACR,
    _reserved1: [u8; 0x04],
    flash_nskeyr: FLASH_NSKEYR,
    flash_seckeyr: FLASH_SECKEYR,
    flash_optkeyr: FLASH_OPTKEYR,
    _reserved4: [u8; 0x04],
    flash_pdkey1r: FLASH_PDKEY1R,
    flash_pdkey2r: FLASH_PDKEY2R,
    flash_nssr: FLASH_NSSR,
    flash_secsr: FLASH_SECSR,
    flash_nscr: FLASH_NSCR,
    flash_seccr: FLASH_SECCR,
    flash_eccr: FLASH_ECCR,
    flash_opsr: FLASH_OPSR,
    _reserved12: [u8; 0x08],
    flash_optr: FLASH_OPTR,
    flash_nsbootadd0r: FLASH_NSBOOTADD0R,
    flash_nsbootadd1r: FLASH_NSBOOTADD1R,
    flash_secbootadd0r: FLASH_SECBOOTADD0R,
    flash_secwm1r1: FLASH_SECWM1R1,
    flash_secwm1r2: FLASH_SECWM1R2,
    flash_wrp1ar: FLASH_WRP1AR,
    flash_wrp1br: FLASH_WRP1BR,
    flash_secwm2r1: FLASH_SECWM2R1,
    flash_secwm2r2: FLASH_SECWM2R2,
    flash_wrp2ar: FLASH_WRP2AR,
    flash_wrp2br: FLASH_WRP2BR,
    flash_oem1keyr1: FLASH_OEM1KEYR1,
    flash_oem1keyr2: FLASH_OEM1KEYR2,
    flash_oem2keyr1: FLASH_OEM2KEYR1,
    flash_oem2keyr2: FLASH_OEM2KEYR2,
    flash_sec1bbr1: FLASH_SEC1BBR1,
    flash_sec1bbr2: FLASH_SEC1BBR2,
    flash_sec1bbr3: FLASH_SEC1BBR3,
    flash_sec1bbr4: FLASH_SEC1BBR4,
    flash_sec1bbr5: FLASH_SEC1BBR5,
    flash_sec1bbr6: FLASH_SEC1BBR6,
    flash_sec1bbr7: FLASH_SEC1BBR7,
    flash_sec1bbr8: FLASH_SEC1BBR8,
    flash_sec2bbr1: FLASH_SEC2BBR1,
    flash_sec2bbr2: FLASH_SEC2BBR2,
    flash_sec2bbr3: FLASH_SEC2BBR3,
    flash_sec2bbr4: FLASH_SEC2BBR4,
    flash_sec2bbr5: FLASH_SEC2BBR5,
    flash_sec2bbr6: FLASH_SEC2BBR6,
    flash_sec2bbr7: FLASH_SEC2BBR7,
    flash_sec2bbr8: FLASH_SEC2BBR8,
    flash_sechdpcr: FLASH_SECHDPCR,
    flash_privcfgr: FLASH_PRIVCFGR,
    _reserved46: [u8; 0x08],
    flash_priv1bbr1: FLASH_PRIV1BBR1,
    flash_priv1bbr2: FLASH_PRIV1BBR2,
    flash_priv1bbr3: FLASH_PRIV1BBR3,
    flash_priv1bbr4: FLASH_PRIV1BBR4,
    flash_priv1bbr5: FLASH_PRIV1BBR5,
    flash_priv1bbr6: FLASH_PRIV1BBR6,
    flash_priv1bbr7: FLASH_PRIV1BBR7,
    flash_priv1bbr8: FLASH_PRIV1BBR8,
    flash_priv2bbr1: FLASH_PRIV2BBR1,
    flash_priv2bbr2: FLASH_PRIV2BBR2,
    flash_priv2bbr3: FLASH_PRIV2BBR3,
    flash_priv2bbr4: FLASH_PRIV2BBR4,
    flash_priv2bbr5: FLASH_PRIV2BBR5,
    flash_priv2bbr6: FLASH_PRIV2BBR6,
    flash_priv2bbr7: FLASH_PRIV2BBR7,
    flash_priv2bbr8: FLASH_PRIV2BBR8,
}
impl RegisterBlock {
    #[doc = "0x00 - FLASH access control register"]
    #[inline(always)]
    pub const fn flash_acr(&self) -> &FLASH_ACR {
        &self.flash_acr
    }
    #[doc = "0x08 - FLASH non-secure key register"]
    #[inline(always)]
    pub const fn flash_nskeyr(&self) -> &FLASH_NSKEYR {
        &self.flash_nskeyr
    }
    #[doc = "0x0c - FLASH secure key register"]
    #[inline(always)]
    pub const fn flash_seckeyr(&self) -> &FLASH_SECKEYR {
        &self.flash_seckeyr
    }
    #[doc = "0x10 - FLASH option key register"]
    #[inline(always)]
    pub const fn flash_optkeyr(&self) -> &FLASH_OPTKEYR {
        &self.flash_optkeyr
    }
    #[doc = "0x18 - FLASH bank 1 power-down key register"]
    #[inline(always)]
    pub const fn flash_pdkey1r(&self) -> &FLASH_PDKEY1R {
        &self.flash_pdkey1r
    }
    #[doc = "0x1c - FLASH bank 2 power-down key register"]
    #[inline(always)]
    pub const fn flash_pdkey2r(&self) -> &FLASH_PDKEY2R {
        &self.flash_pdkey2r
    }
    #[doc = "0x20 - FLASH non-secure status register"]
    #[inline(always)]
    pub const fn flash_nssr(&self) -> &FLASH_NSSR {
        &self.flash_nssr
    }
    #[doc = "0x24 - FLASH secure status register"]
    #[inline(always)]
    pub const fn flash_secsr(&self) -> &FLASH_SECSR {
        &self.flash_secsr
    }
    #[doc = "0x28 - FLASH non-secure control register"]
    #[inline(always)]
    pub const fn flash_nscr(&self) -> &FLASH_NSCR {
        &self.flash_nscr
    }
    #[doc = "0x2c - FLASH secure control register"]
    #[inline(always)]
    pub const fn flash_seccr(&self) -> &FLASH_SECCR {
        &self.flash_seccr
    }
    #[doc = "0x30 - FLASH ECC register"]
    #[inline(always)]
    pub const fn flash_eccr(&self) -> &FLASH_ECCR {
        &self.flash_eccr
    }
    #[doc = "0x34 - FLASH operation status register"]
    #[inline(always)]
    pub const fn flash_opsr(&self) -> &FLASH_OPSR {
        &self.flash_opsr
    }
    #[doc = "0x40 - FLASH option register"]
    #[inline(always)]
    pub const fn flash_optr(&self) -> &FLASH_OPTR {
        &self.flash_optr
    }
    #[doc = "0x44 - FLASH non-secure boot address 0 register"]
    #[inline(always)]
    pub const fn flash_nsbootadd0r(&self) -> &FLASH_NSBOOTADD0R {
        &self.flash_nsbootadd0r
    }
    #[doc = "0x48 - FLASH non-secure boot address 1 register"]
    #[inline(always)]
    pub const fn flash_nsbootadd1r(&self) -> &FLASH_NSBOOTADD1R {
        &self.flash_nsbootadd1r
    }
    #[doc = "0x4c - FLASH secure boot address 0 register"]
    #[inline(always)]
    pub const fn flash_secbootadd0r(&self) -> &FLASH_SECBOOTADD0R {
        &self.flash_secbootadd0r
    }
    #[doc = "0x50 - FLASH secure watermark1 register 1"]
    #[inline(always)]
    pub const fn flash_secwm1r1(&self) -> &FLASH_SECWM1R1 {
        &self.flash_secwm1r1
    }
    #[doc = "0x54 - FLASH secure watermark1 register 2"]
    #[inline(always)]
    pub const fn flash_secwm1r2(&self) -> &FLASH_SECWM1R2 {
        &self.flash_secwm1r2
    }
    #[doc = "0x58 - FLASH WRP1 area A address register"]
    #[inline(always)]
    pub const fn flash_wrp1ar(&self) -> &FLASH_WRP1AR {
        &self.flash_wrp1ar
    }
    #[doc = "0x5c - FLASH WRP1 area B address register"]
    #[inline(always)]
    pub const fn flash_wrp1br(&self) -> &FLASH_WRP1BR {
        &self.flash_wrp1br
    }
    #[doc = "0x60 - FLASH secure watermark2 register 1"]
    #[inline(always)]
    pub const fn flash_secwm2r1(&self) -> &FLASH_SECWM2R1 {
        &self.flash_secwm2r1
    }
    #[doc = "0x64 - FLASH secure watermark2 register 2"]
    #[inline(always)]
    pub const fn flash_secwm2r2(&self) -> &FLASH_SECWM2R2 {
        &self.flash_secwm2r2
    }
    #[doc = "0x68 - FLASH WPR2 area A address register"]
    #[inline(always)]
    pub const fn flash_wrp2ar(&self) -> &FLASH_WRP2AR {
        &self.flash_wrp2ar
    }
    #[doc = "0x6c - FLASH WPR2 area B address register"]
    #[inline(always)]
    pub const fn flash_wrp2br(&self) -> &FLASH_WRP2BR {
        &self.flash_wrp2br
    }
    #[doc = "0x70 - FLASH OEM1 key register 1"]
    #[inline(always)]
    pub const fn flash_oem1keyr1(&self) -> &FLASH_OEM1KEYR1 {
        &self.flash_oem1keyr1
    }
    #[doc = "0x74 - FLASH OEM1 key register 2"]
    #[inline(always)]
    pub const fn flash_oem1keyr2(&self) -> &FLASH_OEM1KEYR2 {
        &self.flash_oem1keyr2
    }
    #[doc = "0x78 - FLASH OEM2 key register 1"]
    #[inline(always)]
    pub const fn flash_oem2keyr1(&self) -> &FLASH_OEM2KEYR1 {
        &self.flash_oem2keyr1
    }
    #[doc = "0x7c - FLASH OEM2 key register 2"]
    #[inline(always)]
    pub const fn flash_oem2keyr2(&self) -> &FLASH_OEM2KEYR2 {
        &self.flash_oem2keyr2
    }
    #[doc = "0x80 - FLASH secure block based bank 1 register 1"]
    #[inline(always)]
    pub const fn flash_sec1bbr1(&self) -> &FLASH_SEC1BBR1 {
        &self.flash_sec1bbr1
    }
    #[doc = "0x84 - FLASH secure block based bank 1 register 2"]
    #[inline(always)]
    pub const fn flash_sec1bbr2(&self) -> &FLASH_SEC1BBR2 {
        &self.flash_sec1bbr2
    }
    #[doc = "0x88 - FLASH secure block based bank 1 register 3"]
    #[inline(always)]
    pub const fn flash_sec1bbr3(&self) -> &FLASH_SEC1BBR3 {
        &self.flash_sec1bbr3
    }
    #[doc = "0x8c - FLASH secure block based bank 1 register 4"]
    #[inline(always)]
    pub const fn flash_sec1bbr4(&self) -> &FLASH_SEC1BBR4 {
        &self.flash_sec1bbr4
    }
    #[doc = "0x90 - FLASH secure block based bank 1 register 5"]
    #[inline(always)]
    pub const fn flash_sec1bbr5(&self) -> &FLASH_SEC1BBR5 {
        &self.flash_sec1bbr5
    }
    #[doc = "0x94 - FLASH secure block based bank 1 register 6"]
    #[inline(always)]
    pub const fn flash_sec1bbr6(&self) -> &FLASH_SEC1BBR6 {
        &self.flash_sec1bbr6
    }
    #[doc = "0x98 - FLASH secure block based bank 1 register 7"]
    #[inline(always)]
    pub const fn flash_sec1bbr7(&self) -> &FLASH_SEC1BBR7 {
        &self.flash_sec1bbr7
    }
    #[doc = "0x9c - FLASH secure block based bank 1 register 8"]
    #[inline(always)]
    pub const fn flash_sec1bbr8(&self) -> &FLASH_SEC1BBR8 {
        &self.flash_sec1bbr8
    }
    #[doc = "0xa0 - FLASH secure block based bank 2 register 1"]
    #[inline(always)]
    pub const fn flash_sec2bbr1(&self) -> &FLASH_SEC2BBR1 {
        &self.flash_sec2bbr1
    }
    #[doc = "0xa4 - FLASH secure block based bank 2 register 2"]
    #[inline(always)]
    pub const fn flash_sec2bbr2(&self) -> &FLASH_SEC2BBR2 {
        &self.flash_sec2bbr2
    }
    #[doc = "0xa8 - FLASH secure block based bank 2 register 3"]
    #[inline(always)]
    pub const fn flash_sec2bbr3(&self) -> &FLASH_SEC2BBR3 {
        &self.flash_sec2bbr3
    }
    #[doc = "0xac - FLASH secure block based bank 2 register 4"]
    #[inline(always)]
    pub const fn flash_sec2bbr4(&self) -> &FLASH_SEC2BBR4 {
        &self.flash_sec2bbr4
    }
    #[doc = "0xb0 - FLASH secure block based bank 2 register 5"]
    #[inline(always)]
    pub const fn flash_sec2bbr5(&self) -> &FLASH_SEC2BBR5 {
        &self.flash_sec2bbr5
    }
    #[doc = "0xb4 - FLASH secure block based bank 2 register 6"]
    #[inline(always)]
    pub const fn flash_sec2bbr6(&self) -> &FLASH_SEC2BBR6 {
        &self.flash_sec2bbr6
    }
    #[doc = "0xb8 - FLASH secure block based bank 2 register 7"]
    #[inline(always)]
    pub const fn flash_sec2bbr7(&self) -> &FLASH_SEC2BBR7 {
        &self.flash_sec2bbr7
    }
    #[doc = "0xbc - FLASH secure block based bank 2 register 8"]
    #[inline(always)]
    pub const fn flash_sec2bbr8(&self) -> &FLASH_SEC2BBR8 {
        &self.flash_sec2bbr8
    }
    #[doc = "0xc0 - FLASH secure HDP control register"]
    #[inline(always)]
    pub const fn flash_sechdpcr(&self) -> &FLASH_SECHDPCR {
        &self.flash_sechdpcr
    }
    #[doc = "0xc4 - FLASH privilege configuration register"]
    #[inline(always)]
    pub const fn flash_privcfgr(&self) -> &FLASH_PRIVCFGR {
        &self.flash_privcfgr
    }
    #[doc = "0xd0 - FLASH privilege block based bank 1 register 1"]
    #[inline(always)]
    pub const fn flash_priv1bbr1(&self) -> &FLASH_PRIV1BBR1 {
        &self.flash_priv1bbr1
    }
    #[doc = "0xd4 - FLASH privilege block based bank 1 register 2"]
    #[inline(always)]
    pub const fn flash_priv1bbr2(&self) -> &FLASH_PRIV1BBR2 {
        &self.flash_priv1bbr2
    }
    #[doc = "0xd8 - FLASH privilege block based bank 1 register 3"]
    #[inline(always)]
    pub const fn flash_priv1bbr3(&self) -> &FLASH_PRIV1BBR3 {
        &self.flash_priv1bbr3
    }
    #[doc = "0xdc - FLASH privilege block based bank 1 register 4"]
    #[inline(always)]
    pub const fn flash_priv1bbr4(&self) -> &FLASH_PRIV1BBR4 {
        &self.flash_priv1bbr4
    }
    #[doc = "0xe0 - FLASH privilege block based bank 1 register 5"]
    #[inline(always)]
    pub const fn flash_priv1bbr5(&self) -> &FLASH_PRIV1BBR5 {
        &self.flash_priv1bbr5
    }
    #[doc = "0xe4 - FLASH privilege block based bank 1 register 6"]
    #[inline(always)]
    pub const fn flash_priv1bbr6(&self) -> &FLASH_PRIV1BBR6 {
        &self.flash_priv1bbr6
    }
    #[doc = "0xe8 - FLASH privilege block based bank 1 register 7"]
    #[inline(always)]
    pub const fn flash_priv1bbr7(&self) -> &FLASH_PRIV1BBR7 {
        &self.flash_priv1bbr7
    }
    #[doc = "0xec - FLASH privilege block based bank 1 register 8"]
    #[inline(always)]
    pub const fn flash_priv1bbr8(&self) -> &FLASH_PRIV1BBR8 {
        &self.flash_priv1bbr8
    }
    #[doc = "0xf0 - FLASH privilege block based bank 2 register 1"]
    #[inline(always)]
    pub const fn flash_priv2bbr1(&self) -> &FLASH_PRIV2BBR1 {
        &self.flash_priv2bbr1
    }
    #[doc = "0xf4 - FLASH privilege block based bank 2 register 2"]
    #[inline(always)]
    pub const fn flash_priv2bbr2(&self) -> &FLASH_PRIV2BBR2 {
        &self.flash_priv2bbr2
    }
    #[doc = "0xf8 - FLASH privilege block based bank 2 register 3"]
    #[inline(always)]
    pub const fn flash_priv2bbr3(&self) -> &FLASH_PRIV2BBR3 {
        &self.flash_priv2bbr3
    }
    #[doc = "0xfc - FLASH privilege block based bank 2 register 4"]
    #[inline(always)]
    pub const fn flash_priv2bbr4(&self) -> &FLASH_PRIV2BBR4 {
        &self.flash_priv2bbr4
    }
    #[doc = "0x100 - FLASH privilege block based bank 2 register 5"]
    #[inline(always)]
    pub const fn flash_priv2bbr5(&self) -> &FLASH_PRIV2BBR5 {
        &self.flash_priv2bbr5
    }
    #[doc = "0x104 - FLASH privilege block based bank 2 register 6"]
    #[inline(always)]
    pub const fn flash_priv2bbr6(&self) -> &FLASH_PRIV2BBR6 {
        &self.flash_priv2bbr6
    }
    #[doc = "0x108 - FLASH privilege block based bank 2 register 7"]
    #[inline(always)]
    pub const fn flash_priv2bbr7(&self) -> &FLASH_PRIV2BBR7 {
        &self.flash_priv2bbr7
    }
    #[doc = "0x10c - FLASH privilege block based bank 2 register 8"]
    #[inline(always)]
    pub const fn flash_priv2bbr8(&self) -> &FLASH_PRIV2BBR8 {
        &self.flash_priv2bbr8
    }
}
#[doc = "FLASH_ACR (rw) register accessor: FLASH access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_acr`]
module"]
pub type FLASH_ACR = crate::Reg<flash_acr::FLASH_ACRrs>;
#[doc = "FLASH access control register"]
pub mod flash_acr;
#[doc = "FLASH_NSKEYR (w) register accessor: FLASH non-secure key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_nskeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_nskeyr`]
module"]
pub type FLASH_NSKEYR = crate::Reg<flash_nskeyr::FLASH_NSKEYRrs>;
#[doc = "FLASH non-secure key register"]
pub mod flash_nskeyr;
#[doc = "FLASH_SECKEYR (w) register accessor: FLASH secure key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_seckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_seckeyr`]
module"]
pub type FLASH_SECKEYR = crate::Reg<flash_seckeyr::FLASH_SECKEYRrs>;
#[doc = "FLASH secure key register"]
pub mod flash_seckeyr;
#[doc = "FLASH_OPTKEYR (w) register accessor: FLASH option key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_optkeyr`]
module"]
pub type FLASH_OPTKEYR = crate::Reg<flash_optkeyr::FLASH_OPTKEYRrs>;
#[doc = "FLASH option key register"]
pub mod flash_optkeyr;
#[doc = "FLASH_PDKEY1R (w) register accessor: FLASH bank 1 power-down key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_pdkey1r::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_pdkey1r`]
module"]
pub type FLASH_PDKEY1R = crate::Reg<flash_pdkey1r::FLASH_PDKEY1Rrs>;
#[doc = "FLASH bank 1 power-down key register"]
pub mod flash_pdkey1r;
#[doc = "FLASH_PDKEY2R (w) register accessor: FLASH bank 2 power-down key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_pdkey2r::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_pdkey2r`]
module"]
pub type FLASH_PDKEY2R = crate::Reg<flash_pdkey2r::FLASH_PDKEY2Rrs>;
#[doc = "FLASH bank 2 power-down key register"]
pub mod flash_pdkey2r;
#[doc = "FLASH_NSSR (rw) register accessor: FLASH non-secure status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_nssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_nssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_nssr`]
module"]
pub type FLASH_NSSR = crate::Reg<flash_nssr::FLASH_NSSRrs>;
#[doc = "FLASH non-secure status register"]
pub mod flash_nssr;
#[doc = "FLASH_SECSR (rw) register accessor: FLASH secure status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_secsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_secsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_secsr`]
module"]
pub type FLASH_SECSR = crate::Reg<flash_secsr::FLASH_SECSRrs>;
#[doc = "FLASH secure status register"]
pub mod flash_secsr;
#[doc = "FLASH_NSCR (rw) register accessor: FLASH non-secure control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_nscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_nscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_nscr`]
module"]
pub type FLASH_NSCR = crate::Reg<flash_nscr::FLASH_NSCRrs>;
#[doc = "FLASH non-secure control register"]
pub mod flash_nscr;
#[doc = "FLASH_SECCR (rw) register accessor: FLASH secure control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_seccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_seccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_seccr`]
module"]
pub type FLASH_SECCR = crate::Reg<flash_seccr::FLASH_SECCRrs>;
#[doc = "FLASH secure control register"]
pub mod flash_seccr;
#[doc = "FLASH_ECCR (rw) register accessor: FLASH ECC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_eccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_eccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_eccr`]
module"]
pub type FLASH_ECCR = crate::Reg<flash_eccr::FLASH_ECCRrs>;
#[doc = "FLASH ECC register"]
pub mod flash_eccr;
#[doc = "FLASH_OPSR (r) register accessor: FLASH operation status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_opsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_opsr`]
module"]
pub type FLASH_OPSR = crate::Reg<flash_opsr::FLASH_OPSRrs>;
#[doc = "FLASH operation status register"]
pub mod flash_opsr;
#[doc = "FLASH_OPTR (rw) register accessor: FLASH option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_optr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_optr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_optr`]
module"]
pub type FLASH_OPTR = crate::Reg<flash_optr::FLASH_OPTRrs>;
#[doc = "FLASH option register"]
pub mod flash_optr;
#[doc = "FLASH_NSBOOTADD0R (rw) register accessor: FLASH non-secure boot address 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_nsbootadd0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_nsbootadd0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_nsbootadd0r`]
module"]
pub type FLASH_NSBOOTADD0R = crate::Reg<flash_nsbootadd0r::FLASH_NSBOOTADD0Rrs>;
#[doc = "FLASH non-secure boot address 0 register"]
pub mod flash_nsbootadd0r;
#[doc = "FLASH_NSBOOTADD1R (rw) register accessor: FLASH non-secure boot address 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_nsbootadd1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_nsbootadd1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_nsbootadd1r`]
module"]
pub type FLASH_NSBOOTADD1R = crate::Reg<flash_nsbootadd1r::FLASH_NSBOOTADD1Rrs>;
#[doc = "FLASH non-secure boot address 1 register"]
pub mod flash_nsbootadd1r;
#[doc = "FLASH_SECBOOTADD0R (rw) register accessor: FLASH secure boot address 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_secbootadd0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_secbootadd0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_secbootadd0r`]
module"]
pub type FLASH_SECBOOTADD0R = crate::Reg<flash_secbootadd0r::FLASH_SECBOOTADD0Rrs>;
#[doc = "FLASH secure boot address 0 register"]
pub mod flash_secbootadd0r;
#[doc = "FLASH_SECWM1R1 (rw) register accessor: FLASH secure watermark1 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_secwm1r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_secwm1r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_secwm1r1`]
module"]
pub type FLASH_SECWM1R1 = crate::Reg<flash_secwm1r1::FLASH_SECWM1R1rs>;
#[doc = "FLASH secure watermark1 register 1"]
pub mod flash_secwm1r1;
#[doc = "FLASH_SECWM1R2 (rw) register accessor: FLASH secure watermark1 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_secwm1r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_secwm1r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_secwm1r2`]
module"]
pub type FLASH_SECWM1R2 = crate::Reg<flash_secwm1r2::FLASH_SECWM1R2rs>;
#[doc = "FLASH secure watermark1 register 2"]
pub mod flash_secwm1r2;
#[doc = "FLASH_WRP1AR (rw) register accessor: FLASH WRP1 area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_wrp1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_wrp1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_wrp1ar`]
module"]
pub type FLASH_WRP1AR = crate::Reg<flash_wrp1ar::FLASH_WRP1ARrs>;
#[doc = "FLASH WRP1 area A address register"]
pub mod flash_wrp1ar;
#[doc = "FLASH_WRP1BR (rw) register accessor: FLASH WRP1 area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_wrp1br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_wrp1br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_wrp1br`]
module"]
pub type FLASH_WRP1BR = crate::Reg<flash_wrp1br::FLASH_WRP1BRrs>;
#[doc = "FLASH WRP1 area B address register"]
pub mod flash_wrp1br;
#[doc = "FLASH_SECWM2R1 (rw) register accessor: FLASH secure watermark2 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_secwm2r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_secwm2r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_secwm2r1`]
module"]
pub type FLASH_SECWM2R1 = crate::Reg<flash_secwm2r1::FLASH_SECWM2R1rs>;
#[doc = "FLASH secure watermark2 register 1"]
pub mod flash_secwm2r1;
#[doc = "FLASH_SECWM2R2 (rw) register accessor: FLASH secure watermark2 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_secwm2r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_secwm2r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_secwm2r2`]
module"]
pub type FLASH_SECWM2R2 = crate::Reg<flash_secwm2r2::FLASH_SECWM2R2rs>;
#[doc = "FLASH secure watermark2 register 2"]
pub mod flash_secwm2r2;
#[doc = "FLASH_WRP2AR (rw) register accessor: FLASH WPR2 area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_wrp2ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_wrp2ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_wrp2ar`]
module"]
pub type FLASH_WRP2AR = crate::Reg<flash_wrp2ar::FLASH_WRP2ARrs>;
#[doc = "FLASH WPR2 area A address register"]
pub mod flash_wrp2ar;
#[doc = "FLASH_WRP2BR (rw) register accessor: FLASH WPR2 area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_wrp2br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_wrp2br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_wrp2br`]
module"]
pub type FLASH_WRP2BR = crate::Reg<flash_wrp2br::FLASH_WRP2BRrs>;
#[doc = "FLASH WPR2 area B address register"]
pub mod flash_wrp2br;
#[doc = "FLASH_OEM1KEYR1 (w) register accessor: FLASH OEM1 key register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_oem1keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_oem1keyr1`]
module"]
pub type FLASH_OEM1KEYR1 = crate::Reg<flash_oem1keyr1::FLASH_OEM1KEYR1rs>;
#[doc = "FLASH OEM1 key register 1"]
pub mod flash_oem1keyr1;
#[doc = "FLASH_OEM1KEYR2 (w) register accessor: FLASH OEM1 key register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_oem1keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_oem1keyr2`]
module"]
pub type FLASH_OEM1KEYR2 = crate::Reg<flash_oem1keyr2::FLASH_OEM1KEYR2rs>;
#[doc = "FLASH OEM1 key register 2"]
pub mod flash_oem1keyr2;
#[doc = "FLASH_OEM2KEYR1 (w) register accessor: FLASH OEM2 key register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_oem2keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_oem2keyr1`]
module"]
pub type FLASH_OEM2KEYR1 = crate::Reg<flash_oem2keyr1::FLASH_OEM2KEYR1rs>;
#[doc = "FLASH OEM2 key register 1"]
pub mod flash_oem2keyr1;
#[doc = "FLASH_OEM2KEYR2 (w) register accessor: FLASH OEM2 key register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_oem2keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_oem2keyr2`]
module"]
pub type FLASH_OEM2KEYR2 = crate::Reg<flash_oem2keyr2::FLASH_OEM2KEYR2rs>;
#[doc = "FLASH OEM2 key register 2"]
pub mod flash_oem2keyr2;
#[doc = "FLASH_SEC1BBR1 (rw) register accessor: FLASH secure block based bank 1 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sec1bbr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sec1bbr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sec1bbr1`]
module"]
pub type FLASH_SEC1BBR1 = crate::Reg<flash_sec1bbr1::FLASH_SEC1BBR1rs>;
#[doc = "FLASH secure block based bank 1 register 1"]
pub mod flash_sec1bbr1;
#[doc = "FLASH_SEC1BBR2 (rw) register accessor: FLASH secure block based bank 1 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sec1bbr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sec1bbr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sec1bbr2`]
module"]
pub type FLASH_SEC1BBR2 = crate::Reg<flash_sec1bbr2::FLASH_SEC1BBR2rs>;
#[doc = "FLASH secure block based bank 1 register 2"]
pub mod flash_sec1bbr2;
#[doc = "FLASH_SEC1BBR3 (rw) register accessor: FLASH secure block based bank 1 register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sec1bbr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sec1bbr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sec1bbr3`]
module"]
pub type FLASH_SEC1BBR3 = crate::Reg<flash_sec1bbr3::FLASH_SEC1BBR3rs>;
#[doc = "FLASH secure block based bank 1 register 3"]
pub mod flash_sec1bbr3;
#[doc = "FLASH_SEC1BBR4 (rw) register accessor: FLASH secure block based bank 1 register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sec1bbr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sec1bbr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sec1bbr4`]
module"]
pub type FLASH_SEC1BBR4 = crate::Reg<flash_sec1bbr4::FLASH_SEC1BBR4rs>;
#[doc = "FLASH secure block based bank 1 register 4"]
pub mod flash_sec1bbr4;
#[doc = "FLASH_SEC1BBR5 (rw) register accessor: FLASH secure block based bank 1 register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sec1bbr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sec1bbr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sec1bbr5`]
module"]
pub type FLASH_SEC1BBR5 = crate::Reg<flash_sec1bbr5::FLASH_SEC1BBR5rs>;
#[doc = "FLASH secure block based bank 1 register 5"]
pub mod flash_sec1bbr5;
#[doc = "FLASH_SEC1BBR6 (rw) register accessor: FLASH secure block based bank 1 register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sec1bbr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sec1bbr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sec1bbr6`]
module"]
pub type FLASH_SEC1BBR6 = crate::Reg<flash_sec1bbr6::FLASH_SEC1BBR6rs>;
#[doc = "FLASH secure block based bank 1 register 6"]
pub mod flash_sec1bbr6;
#[doc = "FLASH_SEC1BBR7 (rw) register accessor: FLASH secure block based bank 1 register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sec1bbr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sec1bbr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sec1bbr7`]
module"]
pub type FLASH_SEC1BBR7 = crate::Reg<flash_sec1bbr7::FLASH_SEC1BBR7rs>;
#[doc = "FLASH secure block based bank 1 register 7"]
pub mod flash_sec1bbr7;
#[doc = "FLASH_SEC1BBR8 (rw) register accessor: FLASH secure block based bank 1 register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sec1bbr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sec1bbr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sec1bbr8`]
module"]
pub type FLASH_SEC1BBR8 = crate::Reg<flash_sec1bbr8::FLASH_SEC1BBR8rs>;
#[doc = "FLASH secure block based bank 1 register 8"]
pub mod flash_sec1bbr8;
#[doc = "FLASH_SEC2BBR1 (rw) register accessor: FLASH secure block based bank 2 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sec2bbr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sec2bbr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sec2bbr1`]
module"]
pub type FLASH_SEC2BBR1 = crate::Reg<flash_sec2bbr1::FLASH_SEC2BBR1rs>;
#[doc = "FLASH secure block based bank 2 register 1"]
pub mod flash_sec2bbr1;
#[doc = "FLASH_SEC2BBR2 (rw) register accessor: FLASH secure block based bank 2 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sec2bbr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sec2bbr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sec2bbr2`]
module"]
pub type FLASH_SEC2BBR2 = crate::Reg<flash_sec2bbr2::FLASH_SEC2BBR2rs>;
#[doc = "FLASH secure block based bank 2 register 2"]
pub mod flash_sec2bbr2;
#[doc = "FLASH_SEC2BBR3 (rw) register accessor: FLASH secure block based bank 2 register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sec2bbr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sec2bbr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sec2bbr3`]
module"]
pub type FLASH_SEC2BBR3 = crate::Reg<flash_sec2bbr3::FLASH_SEC2BBR3rs>;
#[doc = "FLASH secure block based bank 2 register 3"]
pub mod flash_sec2bbr3;
#[doc = "FLASH_SEC2BBR4 (rw) register accessor: FLASH secure block based bank 2 register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sec2bbr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sec2bbr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sec2bbr4`]
module"]
pub type FLASH_SEC2BBR4 = crate::Reg<flash_sec2bbr4::FLASH_SEC2BBR4rs>;
#[doc = "FLASH secure block based bank 2 register 4"]
pub mod flash_sec2bbr4;
#[doc = "FLASH_SEC2BBR5 (rw) register accessor: FLASH secure block based bank 2 register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sec2bbr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sec2bbr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sec2bbr5`]
module"]
pub type FLASH_SEC2BBR5 = crate::Reg<flash_sec2bbr5::FLASH_SEC2BBR5rs>;
#[doc = "FLASH secure block based bank 2 register 5"]
pub mod flash_sec2bbr5;
#[doc = "FLASH_SEC2BBR6 (rw) register accessor: FLASH secure block based bank 2 register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sec2bbr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sec2bbr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sec2bbr6`]
module"]
pub type FLASH_SEC2BBR6 = crate::Reg<flash_sec2bbr6::FLASH_SEC2BBR6rs>;
#[doc = "FLASH secure block based bank 2 register 6"]
pub mod flash_sec2bbr6;
#[doc = "FLASH_SEC2BBR7 (rw) register accessor: FLASH secure block based bank 2 register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sec2bbr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sec2bbr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sec2bbr7`]
module"]
pub type FLASH_SEC2BBR7 = crate::Reg<flash_sec2bbr7::FLASH_SEC2BBR7rs>;
#[doc = "FLASH secure block based bank 2 register 7"]
pub mod flash_sec2bbr7;
#[doc = "FLASH_SEC2BBR8 (rw) register accessor: FLASH secure block based bank 2 register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sec2bbr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sec2bbr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sec2bbr8`]
module"]
pub type FLASH_SEC2BBR8 = crate::Reg<flash_sec2bbr8::FLASH_SEC2BBR8rs>;
#[doc = "FLASH secure block based bank 2 register 8"]
pub mod flash_sec2bbr8;
#[doc = "FLASH_SECHDPCR (rw) register accessor: FLASH secure HDP control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sechdpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sechdpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sechdpcr`]
module"]
pub type FLASH_SECHDPCR = crate::Reg<flash_sechdpcr::FLASH_SECHDPCRrs>;
#[doc = "FLASH secure HDP control register"]
pub mod flash_sechdpcr;
#[doc = "FLASH_PRIVCFGR (rw) register accessor: FLASH privilege configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_privcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_privcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_privcfgr`]
module"]
pub type FLASH_PRIVCFGR = crate::Reg<flash_privcfgr::FLASH_PRIVCFGRrs>;
#[doc = "FLASH privilege configuration register"]
pub mod flash_privcfgr;
#[doc = "FLASH_PRIV1BBR1 (rw) register accessor: FLASH privilege block based bank 1 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_priv1bbr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_priv1bbr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_priv1bbr1`]
module"]
pub type FLASH_PRIV1BBR1 = crate::Reg<flash_priv1bbr1::FLASH_PRIV1BBR1rs>;
#[doc = "FLASH privilege block based bank 1 register 1"]
pub mod flash_priv1bbr1;
#[doc = "FLASH_PRIV1BBR2 (rw) register accessor: FLASH privilege block based bank 1 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_priv1bbr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_priv1bbr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_priv1bbr2`]
module"]
pub type FLASH_PRIV1BBR2 = crate::Reg<flash_priv1bbr2::FLASH_PRIV1BBR2rs>;
#[doc = "FLASH privilege block based bank 1 register 2"]
pub mod flash_priv1bbr2;
#[doc = "FLASH_PRIV1BBR3 (rw) register accessor: FLASH privilege block based bank 1 register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_priv1bbr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_priv1bbr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_priv1bbr3`]
module"]
pub type FLASH_PRIV1BBR3 = crate::Reg<flash_priv1bbr3::FLASH_PRIV1BBR3rs>;
#[doc = "FLASH privilege block based bank 1 register 3"]
pub mod flash_priv1bbr3;
#[doc = "FLASH_PRIV1BBR4 (rw) register accessor: FLASH privilege block based bank 1 register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_priv1bbr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_priv1bbr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_priv1bbr4`]
module"]
pub type FLASH_PRIV1BBR4 = crate::Reg<flash_priv1bbr4::FLASH_PRIV1BBR4rs>;
#[doc = "FLASH privilege block based bank 1 register 4"]
pub mod flash_priv1bbr4;
#[doc = "FLASH_PRIV1BBR5 (rw) register accessor: FLASH privilege block based bank 1 register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_priv1bbr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_priv1bbr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_priv1bbr5`]
module"]
pub type FLASH_PRIV1BBR5 = crate::Reg<flash_priv1bbr5::FLASH_PRIV1BBR5rs>;
#[doc = "FLASH privilege block based bank 1 register 5"]
pub mod flash_priv1bbr5;
#[doc = "FLASH_PRIV1BBR6 (rw) register accessor: FLASH privilege block based bank 1 register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_priv1bbr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_priv1bbr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_priv1bbr6`]
module"]
pub type FLASH_PRIV1BBR6 = crate::Reg<flash_priv1bbr6::FLASH_PRIV1BBR6rs>;
#[doc = "FLASH privilege block based bank 1 register 6"]
pub mod flash_priv1bbr6;
#[doc = "FLASH_PRIV1BBR7 (rw) register accessor: FLASH privilege block based bank 1 register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_priv1bbr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_priv1bbr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_priv1bbr7`]
module"]
pub type FLASH_PRIV1BBR7 = crate::Reg<flash_priv1bbr7::FLASH_PRIV1BBR7rs>;
#[doc = "FLASH privilege block based bank 1 register 7"]
pub mod flash_priv1bbr7;
#[doc = "FLASH_PRIV1BBR8 (rw) register accessor: FLASH privilege block based bank 1 register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_priv1bbr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_priv1bbr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_priv1bbr8`]
module"]
pub type FLASH_PRIV1BBR8 = crate::Reg<flash_priv1bbr8::FLASH_PRIV1BBR8rs>;
#[doc = "FLASH privilege block based bank 1 register 8"]
pub mod flash_priv1bbr8;
#[doc = "FLASH_PRIV2BBR1 (rw) register accessor: FLASH privilege block based bank 2 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_priv2bbr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_priv2bbr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_priv2bbr1`]
module"]
pub type FLASH_PRIV2BBR1 = crate::Reg<flash_priv2bbr1::FLASH_PRIV2BBR1rs>;
#[doc = "FLASH privilege block based bank 2 register 1"]
pub mod flash_priv2bbr1;
#[doc = "FLASH_PRIV2BBR2 (rw) register accessor: FLASH privilege block based bank 2 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_priv2bbr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_priv2bbr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_priv2bbr2`]
module"]
pub type FLASH_PRIV2BBR2 = crate::Reg<flash_priv2bbr2::FLASH_PRIV2BBR2rs>;
#[doc = "FLASH privilege block based bank 2 register 2"]
pub mod flash_priv2bbr2;
#[doc = "FLASH_PRIV2BBR3 (rw) register accessor: FLASH privilege block based bank 2 register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_priv2bbr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_priv2bbr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_priv2bbr3`]
module"]
pub type FLASH_PRIV2BBR3 = crate::Reg<flash_priv2bbr3::FLASH_PRIV2BBR3rs>;
#[doc = "FLASH privilege block based bank 2 register 3"]
pub mod flash_priv2bbr3;
#[doc = "FLASH_PRIV2BBR4 (rw) register accessor: FLASH privilege block based bank 2 register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_priv2bbr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_priv2bbr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_priv2bbr4`]
module"]
pub type FLASH_PRIV2BBR4 = crate::Reg<flash_priv2bbr4::FLASH_PRIV2BBR4rs>;
#[doc = "FLASH privilege block based bank 2 register 4"]
pub mod flash_priv2bbr4;
#[doc = "FLASH_PRIV2BBR5 (rw) register accessor: FLASH privilege block based bank 2 register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_priv2bbr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_priv2bbr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_priv2bbr5`]
module"]
pub type FLASH_PRIV2BBR5 = crate::Reg<flash_priv2bbr5::FLASH_PRIV2BBR5rs>;
#[doc = "FLASH privilege block based bank 2 register 5"]
pub mod flash_priv2bbr5;
#[doc = "FLASH_PRIV2BBR6 (rw) register accessor: FLASH privilege block based bank 2 register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_priv2bbr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_priv2bbr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_priv2bbr6`]
module"]
pub type FLASH_PRIV2BBR6 = crate::Reg<flash_priv2bbr6::FLASH_PRIV2BBR6rs>;
#[doc = "FLASH privilege block based bank 2 register 6"]
pub mod flash_priv2bbr6;
#[doc = "FLASH_PRIV2BBR7 (rw) register accessor: FLASH privilege block based bank 2 register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_priv2bbr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_priv2bbr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_priv2bbr7`]
module"]
pub type FLASH_PRIV2BBR7 = crate::Reg<flash_priv2bbr7::FLASH_PRIV2BBR7rs>;
#[doc = "FLASH privilege block based bank 2 register 7"]
pub mod flash_priv2bbr7;
#[doc = "FLASH_PRIV2BBR8 (rw) register accessor: FLASH privilege block based bank 2 register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_priv2bbr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_priv2bbr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_priv2bbr8`]
module"]
pub type FLASH_PRIV2BBR8 = crate::Reg<flash_priv2bbr8::FLASH_PRIV2BBR8rs>;
#[doc = "FLASH privilege block based bank 2 register 8"]
pub mod flash_priv2bbr8;
