#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: ACR,
    #[doc = "0x04 - Power down key register"]
    pub pdkeyr: PDKEYR,
    #[doc = "0x08 - Flash non-secure key register"]
    pub nskeyr: NSKEYR,
    #[doc = "0x0c - Flash secure key register"]
    pub seckeyr: SECKEYR,
    #[doc = "0x10 - Flash option key register"]
    pub optkeyr: OPTKEYR,
    #[doc = "0x14 - Flash low voltage key register"]
    pub lvekeyr: LVEKEYR,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - Flash status register"]
    pub nssr: NSSR,
    #[doc = "0x24 - Flash status register"]
    pub secsr: SECSR,
    #[doc = "0x28 - Flash non-secure control register"]
    pub nscr: NSCR,
    #[doc = "0x2c - Flash secure control register"]
    pub seccr: SECCR,
    #[doc = "0x30 - Flash ECC register"]
    pub eccr: ECCR,
    _reserved11: [u8; 12usize],
    #[doc = "0x40 - Flash option register"]
    pub optr: OPTR,
    #[doc = "0x44 - Flash non-secure boot address 0 register"]
    pub nsbootadd0r: NSBOOTADD0R,
    #[doc = "0x48 - Flash non-secure boot address 1 register"]
    pub nsbootadd1r: NSBOOTADD1R,
    #[doc = "0x4c - FFlash secure boot address 0 register"]
    pub secbootadd0r: SECBOOTADD0R,
    #[doc = "0x50 - Flash bank 1 secure watermak1 register"]
    pub secwm1r1: SECWM1R1,
    #[doc = "0x54 - Flash secure watermak1 register 2"]
    pub secwm1r2: SECWM1R2,
    #[doc = "0x58 - Flash Bank 1 WRP area A address register"]
    pub wrp1ar: WRP1AR,
    #[doc = "0x5c - Flash Bank 1 WRP area B address register"]
    pub wrp1br: WRP1BR,
    #[doc = "0x60 - Flash secure watermak2 register"]
    pub secwm2r1: SECWM2R1,
    #[doc = "0x64 - Flash secure watermak2 register2"]
    pub secwm2r2: SECWM2R2,
    #[doc = "0x68 - Flash WPR2 area A address register"]
    pub wrp2ar: WRP2AR,
    #[doc = "0x6c - Flash WPR2 area B address register"]
    pub wrp2br: WRP2BR,
    _reserved23: [u8; 16usize],
    #[doc = "0x80 - FLASH secure block based bank 1 register"]
    pub secbb1r1: SECBB1R1,
    #[doc = "0x84 - FLASH secure block based bank 1 register"]
    pub secbb1r2: SECBB1R2,
    #[doc = "0x88 - FLASH secure block based bank 1 register"]
    pub secbb1r3: SECBB1R3,
    #[doc = "0x8c - FLASH secure block based bank 1 register"]
    pub secbb1r4: SECBB1R4,
    _reserved27: [u8; 16usize],
    #[doc = "0xa0 - FLASH secure block based bank 2 register"]
    pub secbb2r1: SECBB2R1,
    #[doc = "0xa4 - FLASH secure block based bank 2 register"]
    pub secbb2r2: SECBB2R2,
    #[doc = "0xa8 - FLASH secure block based bank 2 register"]
    pub secbb2r3: SECBB2R3,
    #[doc = "0xac - FLASH secure block based bank 2 register"]
    pub secbb2r4: SECBB2R4,
    _reserved31: [u8; 16usize],
    #[doc = "0xc0 - FLASH secure HDP control register"]
    pub sechdpcr: SECHDPCR,
    #[doc = "0xc4 - Power privilege configuration register"]
    pub privcfgr: PRIVCFGR,
}
#[doc = "Access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](acr) module"]
pub type ACR = crate::Reg<u32, _ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACR;
#[doc = "`read()` method returns [acr::R](acr::R) reader structure"]
impl crate::Readable for ACR {}
#[doc = "`write(|w| ..)` method takes [acr::W](acr::W) writer structure"]
impl crate::Writable for ACR {}
#[doc = "Access control register"]
pub mod acr;
#[doc = "Power down key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdkeyr](pdkeyr) module"]
pub type PDKEYR = crate::Reg<u32, _PDKEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDKEYR;
#[doc = "`write(|w| ..)` method takes [pdkeyr::W](pdkeyr::W) writer structure"]
impl crate::Writable for PDKEYR {}
#[doc = "Power down key register"]
pub mod pdkeyr;
#[doc = "Flash non-secure key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nskeyr](nskeyr) module"]
pub type NSKEYR = crate::Reg<u32, _NSKEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSKEYR;
#[doc = "`write(|w| ..)` method takes [nskeyr::W](nskeyr::W) writer structure"]
impl crate::Writable for NSKEYR {}
#[doc = "Flash non-secure key register"]
pub mod nskeyr;
#[doc = "Flash secure key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seckeyr](seckeyr) module"]
pub type SECKEYR = crate::Reg<u32, _SECKEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECKEYR;
#[doc = "`write(|w| ..)` method takes [seckeyr::W](seckeyr::W) writer structure"]
impl crate::Writable for SECKEYR {}
#[doc = "Flash secure key register"]
pub mod seckeyr;
#[doc = "Flash option key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optkeyr](optkeyr) module"]
pub type OPTKEYR = crate::Reg<u32, _OPTKEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTKEYR;
#[doc = "`write(|w| ..)` method takes [optkeyr::W](optkeyr::W) writer structure"]
impl crate::Writable for OPTKEYR {}
#[doc = "Flash option key register"]
pub mod optkeyr;
#[doc = "Flash low voltage key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvekeyr](lvekeyr) module"]
pub type LVEKEYR = crate::Reg<u32, _LVEKEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LVEKEYR;
#[doc = "`write(|w| ..)` method takes [lvekeyr::W](lvekeyr::W) writer structure"]
impl crate::Writable for LVEKEYR {}
#[doc = "Flash low voltage key register"]
pub mod lvekeyr;
#[doc = "Flash status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nssr](nssr) module"]
pub type NSSR = crate::Reg<u32, _NSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSSR;
#[doc = "`read()` method returns [nssr::R](nssr::R) reader structure"]
impl crate::Readable for NSSR {}
#[doc = "`write(|w| ..)` method takes [nssr::W](nssr::W) writer structure"]
impl crate::Writable for NSSR {}
#[doc = "Flash status register"]
pub mod nssr;
#[doc = "Flash status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secsr](secsr) module"]
pub type SECSR = crate::Reg<u32, _SECSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECSR;
#[doc = "`read()` method returns [secsr::R](secsr::R) reader structure"]
impl crate::Readable for SECSR {}
#[doc = "`write(|w| ..)` method takes [secsr::W](secsr::W) writer structure"]
impl crate::Writable for SECSR {}
#[doc = "Flash status register"]
pub mod secsr;
#[doc = "Flash non-secure control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nscr](nscr) module"]
pub type NSCR = crate::Reg<u32, _NSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSCR;
#[doc = "`read()` method returns [nscr::R](nscr::R) reader structure"]
impl crate::Readable for NSCR {}
#[doc = "`write(|w| ..)` method takes [nscr::W](nscr::W) writer structure"]
impl crate::Writable for NSCR {}
#[doc = "Flash non-secure control register"]
pub mod nscr;
#[doc = "Flash secure control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccr](seccr) module"]
pub type SECCR = crate::Reg<u32, _SECCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECCR;
#[doc = "`read()` method returns [seccr::R](seccr::R) reader structure"]
impl crate::Readable for SECCR {}
#[doc = "`write(|w| ..)` method takes [seccr::W](seccr::W) writer structure"]
impl crate::Writable for SECCR {}
#[doc = "Flash secure control register"]
pub mod seccr;
#[doc = "Flash ECC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccr](eccr) module"]
pub type ECCR = crate::Reg<u32, _ECCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECCR;
#[doc = "`read()` method returns [eccr::R](eccr::R) reader structure"]
impl crate::Readable for ECCR {}
#[doc = "`write(|w| ..)` method takes [eccr::W](eccr::W) writer structure"]
impl crate::Writable for ECCR {}
#[doc = "Flash ECC register"]
pub mod eccr;
#[doc = "Flash option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optr](optr) module"]
pub type OPTR = crate::Reg<u32, _OPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTR;
#[doc = "`read()` method returns [optr::R](optr::R) reader structure"]
impl crate::Readable for OPTR {}
#[doc = "`write(|w| ..)` method takes [optr::W](optr::W) writer structure"]
impl crate::Writable for OPTR {}
#[doc = "Flash option register"]
pub mod optr;
#[doc = "Flash non-secure boot address 0 register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsbootadd0r](nsbootadd0r) module"]
pub type NSBOOTADD0R = crate::Reg<u32, _NSBOOTADD0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSBOOTADD0R;
#[doc = "`write(|w| ..)` method takes [nsbootadd0r::W](nsbootadd0r::W) writer structure"]
impl crate::Writable for NSBOOTADD0R {}
#[doc = "Flash non-secure boot address 0 register"]
pub mod nsbootadd0r;
#[doc = "Flash non-secure boot address 1 register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsbootadd1r](nsbootadd1r) module"]
pub type NSBOOTADD1R = crate::Reg<u32, _NSBOOTADD1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSBOOTADD1R;
#[doc = "`write(|w| ..)` method takes [nsbootadd1r::W](nsbootadd1r::W) writer structure"]
impl crate::Writable for NSBOOTADD1R {}
#[doc = "Flash non-secure boot address 1 register"]
pub mod nsbootadd1r;
#[doc = "FFlash secure boot address 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secbootadd0r](secbootadd0r) module"]
pub type SECBOOTADD0R = crate::Reg<u32, _SECBOOTADD0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECBOOTADD0R;
#[doc = "`read()` method returns [secbootadd0r::R](secbootadd0r::R) reader structure"]
impl crate::Readable for SECBOOTADD0R {}
#[doc = "`write(|w| ..)` method takes [secbootadd0r::W](secbootadd0r::W) writer structure"]
impl crate::Writable for SECBOOTADD0R {}
#[doc = "FFlash secure boot address 0 register"]
pub mod secbootadd0r;
#[doc = "Flash bank 1 secure watermak1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secwm1r1](secwm1r1) module"]
pub type SECWM1R1 = crate::Reg<u32, _SECWM1R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECWM1R1;
#[doc = "`read()` method returns [secwm1r1::R](secwm1r1::R) reader structure"]
impl crate::Readable for SECWM1R1 {}
#[doc = "`write(|w| ..)` method takes [secwm1r1::W](secwm1r1::W) writer structure"]
impl crate::Writable for SECWM1R1 {}
#[doc = "Flash bank 1 secure watermak1 register"]
pub mod secwm1r1;
#[doc = "Flash secure watermak1 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secwm1r2](secwm1r2) module"]
pub type SECWM1R2 = crate::Reg<u32, _SECWM1R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECWM1R2;
#[doc = "`read()` method returns [secwm1r2::R](secwm1r2::R) reader structure"]
impl crate::Readable for SECWM1R2 {}
#[doc = "`write(|w| ..)` method takes [secwm1r2::W](secwm1r2::W) writer structure"]
impl crate::Writable for SECWM1R2 {}
#[doc = "Flash secure watermak1 register 2"]
pub mod secwm1r2;
#[doc = "Flash Bank 1 WRP area A address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrp1ar](wrp1ar) module"]
pub type WRP1AR = crate::Reg<u32, _WRP1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRP1AR;
#[doc = "`read()` method returns [wrp1ar::R](wrp1ar::R) reader structure"]
impl crate::Readable for WRP1AR {}
#[doc = "`write(|w| ..)` method takes [wrp1ar::W](wrp1ar::W) writer structure"]
impl crate::Writable for WRP1AR {}
#[doc = "Flash Bank 1 WRP area A address register"]
pub mod wrp1ar;
#[doc = "Flash Bank 1 WRP area B address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrp1br](wrp1br) module"]
pub type WRP1BR = crate::Reg<u32, _WRP1BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRP1BR;
#[doc = "`read()` method returns [wrp1br::R](wrp1br::R) reader structure"]
impl crate::Readable for WRP1BR {}
#[doc = "`write(|w| ..)` method takes [wrp1br::W](wrp1br::W) writer structure"]
impl crate::Writable for WRP1BR {}
#[doc = "Flash Bank 1 WRP area B address register"]
pub mod wrp1br;
#[doc = "Flash secure watermak2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secwm2r1](secwm2r1) module"]
pub type SECWM2R1 = crate::Reg<u32, _SECWM2R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECWM2R1;
#[doc = "`read()` method returns [secwm2r1::R](secwm2r1::R) reader structure"]
impl crate::Readable for SECWM2R1 {}
#[doc = "`write(|w| ..)` method takes [secwm2r1::W](secwm2r1::W) writer structure"]
impl crate::Writable for SECWM2R1 {}
#[doc = "Flash secure watermak2 register"]
pub mod secwm2r1;
#[doc = "Flash secure watermak2 register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secwm2r2](secwm2r2) module"]
pub type SECWM2R2 = crate::Reg<u32, _SECWM2R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECWM2R2;
#[doc = "`read()` method returns [secwm2r2::R](secwm2r2::R) reader structure"]
impl crate::Readable for SECWM2R2 {}
#[doc = "`write(|w| ..)` method takes [secwm2r2::W](secwm2r2::W) writer structure"]
impl crate::Writable for SECWM2R2 {}
#[doc = "Flash secure watermak2 register2"]
pub mod secwm2r2;
#[doc = "Flash WPR2 area A address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrp2ar](wrp2ar) module"]
pub type WRP2AR = crate::Reg<u32, _WRP2AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRP2AR;
#[doc = "`read()` method returns [wrp2ar::R](wrp2ar::R) reader structure"]
impl crate::Readable for WRP2AR {}
#[doc = "`write(|w| ..)` method takes [wrp2ar::W](wrp2ar::W) writer structure"]
impl crate::Writable for WRP2AR {}
#[doc = "Flash WPR2 area A address register"]
pub mod wrp2ar;
#[doc = "Flash WPR2 area B address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrp2br](wrp2br) module"]
pub type WRP2BR = crate::Reg<u32, _WRP2BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRP2BR;
#[doc = "`read()` method returns [wrp2br::R](wrp2br::R) reader structure"]
impl crate::Readable for WRP2BR {}
#[doc = "`write(|w| ..)` method takes [wrp2br::W](wrp2br::W) writer structure"]
impl crate::Writable for WRP2BR {}
#[doc = "Flash WPR2 area B address register"]
pub mod wrp2br;
#[doc = "FLASH secure block based bank 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secbb1r1](secbb1r1) module"]
pub type SECBB1R1 = crate::Reg<u32, _SECBB1R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECBB1R1;
#[doc = "`read()` method returns [secbb1r1::R](secbb1r1::R) reader structure"]
impl crate::Readable for SECBB1R1 {}
#[doc = "`write(|w| ..)` method takes [secbb1r1::W](secbb1r1::W) writer structure"]
impl crate::Writable for SECBB1R1 {}
#[doc = "FLASH secure block based bank 1 register"]
pub mod secbb1r1;
#[doc = "FLASH secure block based bank 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secbb1r2](secbb1r2) module"]
pub type SECBB1R2 = crate::Reg<u32, _SECBB1R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECBB1R2;
#[doc = "`read()` method returns [secbb1r2::R](secbb1r2::R) reader structure"]
impl crate::Readable for SECBB1R2 {}
#[doc = "`write(|w| ..)` method takes [secbb1r2::W](secbb1r2::W) writer structure"]
impl crate::Writable for SECBB1R2 {}
#[doc = "FLASH secure block based bank 1 register"]
pub mod secbb1r2;
#[doc = "FLASH secure block based bank 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secbb1r3](secbb1r3) module"]
pub type SECBB1R3 = crate::Reg<u32, _SECBB1R3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECBB1R3;
#[doc = "`read()` method returns [secbb1r3::R](secbb1r3::R) reader structure"]
impl crate::Readable for SECBB1R3 {}
#[doc = "`write(|w| ..)` method takes [secbb1r3::W](secbb1r3::W) writer structure"]
impl crate::Writable for SECBB1R3 {}
#[doc = "FLASH secure block based bank 1 register"]
pub mod secbb1r3;
#[doc = "FLASH secure block based bank 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secbb1r4](secbb1r4) module"]
pub type SECBB1R4 = crate::Reg<u32, _SECBB1R4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECBB1R4;
#[doc = "`read()` method returns [secbb1r4::R](secbb1r4::R) reader structure"]
impl crate::Readable for SECBB1R4 {}
#[doc = "`write(|w| ..)` method takes [secbb1r4::W](secbb1r4::W) writer structure"]
impl crate::Writable for SECBB1R4 {}
#[doc = "FLASH secure block based bank 1 register"]
pub mod secbb1r4;
#[doc = "FLASH secure block based bank 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secbb2r1](secbb2r1) module"]
pub type SECBB2R1 = crate::Reg<u32, _SECBB2R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECBB2R1;
#[doc = "`read()` method returns [secbb2r1::R](secbb2r1::R) reader structure"]
impl crate::Readable for SECBB2R1 {}
#[doc = "`write(|w| ..)` method takes [secbb2r1::W](secbb2r1::W) writer structure"]
impl crate::Writable for SECBB2R1 {}
#[doc = "FLASH secure block based bank 2 register"]
pub mod secbb2r1;
#[doc = "FLASH secure block based bank 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secbb2r2](secbb2r2) module"]
pub type SECBB2R2 = crate::Reg<u32, _SECBB2R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECBB2R2;
#[doc = "`read()` method returns [secbb2r2::R](secbb2r2::R) reader structure"]
impl crate::Readable for SECBB2R2 {}
#[doc = "`write(|w| ..)` method takes [secbb2r2::W](secbb2r2::W) writer structure"]
impl crate::Writable for SECBB2R2 {}
#[doc = "FLASH secure block based bank 2 register"]
pub mod secbb2r2;
#[doc = "FLASH secure block based bank 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secbb2r3](secbb2r3) module"]
pub type SECBB2R3 = crate::Reg<u32, _SECBB2R3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECBB2R3;
#[doc = "`read()` method returns [secbb2r3::R](secbb2r3::R) reader structure"]
impl crate::Readable for SECBB2R3 {}
#[doc = "`write(|w| ..)` method takes [secbb2r3::W](secbb2r3::W) writer structure"]
impl crate::Writable for SECBB2R3 {}
#[doc = "FLASH secure block based bank 2 register"]
pub mod secbb2r3;
#[doc = "FLASH secure block based bank 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secbb2r4](secbb2r4) module"]
pub type SECBB2R4 = crate::Reg<u32, _SECBB2R4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECBB2R4;
#[doc = "`read()` method returns [secbb2r4::R](secbb2r4::R) reader structure"]
impl crate::Readable for SECBB2R4 {}
#[doc = "`write(|w| ..)` method takes [secbb2r4::W](secbb2r4::W) writer structure"]
impl crate::Writable for SECBB2R4 {}
#[doc = "FLASH secure block based bank 2 register"]
pub mod secbb2r4;
#[doc = "FLASH secure HDP control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sechdpcr](sechdpcr) module"]
pub type SECHDPCR = crate::Reg<u32, _SECHDPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECHDPCR;
#[doc = "`read()` method returns [sechdpcr::R](sechdpcr::R) reader structure"]
impl crate::Readable for SECHDPCR {}
#[doc = "`write(|w| ..)` method takes [sechdpcr::W](sechdpcr::W) writer structure"]
impl crate::Writable for SECHDPCR {}
#[doc = "FLASH secure HDP control register"]
pub mod sechdpcr;
#[doc = "Power privilege configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [privcfgr](privcfgr) module"]
pub type PRIVCFGR = crate::Reg<u32, _PRIVCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIVCFGR;
#[doc = "`read()` method returns [privcfgr::R](privcfgr::R) reader structure"]
impl crate::Readable for PRIVCFGR {}
#[doc = "`write(|w| ..)` method takes [privcfgr::W](privcfgr::W) writer structure"]
impl crate::Writable for PRIVCFGR {}
#[doc = "Power privilege configuration register"]
pub mod privcfgr;
