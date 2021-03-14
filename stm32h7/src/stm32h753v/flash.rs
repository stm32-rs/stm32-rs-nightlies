#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: ACR,
    _reserved_1_bank1: [u8; 96usize],
    _reserved2: [u8; 156usize],
    #[doc = "0x100 - Access control register"]
    pub acr_: ACR_,
    _reserved_3_bank2: [u8; 96usize],
}
impl RegisterBlock {
    #[doc = "0x04 - Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
    #[inline(always)]
    pub fn bank1(&self) -> &BANK {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const BANK) }
    }
    #[doc = "0x04 - Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
    #[inline(always)]
    pub fn bank1_mut(&self) -> &mut BANK {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut BANK) }
    }
    #[doc = "0x08 - FLASH option key register"]
    #[inline(always)]
    pub fn optkeyr(&self) -> &OPTKEYR {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const OPTKEYR) }
    }
    #[doc = "0x08 - FLASH option key register"]
    #[inline(always)]
    pub fn optkeyr_mut(&self) -> &mut OPTKEYR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut OPTKEYR) }
    }
    #[doc = "0x18 - FLASH option control register"]
    #[inline(always)]
    pub fn optcr(&self) -> &OPTCR {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const OPTCR) }
    }
    #[doc = "0x18 - FLASH option control register"]
    #[inline(always)]
    pub fn optcr_mut(&self) -> &mut OPTCR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut OPTCR) }
    }
    #[doc = "0x1c - FLASH option status register"]
    #[inline(always)]
    pub fn optsr_cur(&self) -> &OPTSR_CUR {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const OPTSR_CUR) }
    }
    #[doc = "0x1c - FLASH option status register"]
    #[inline(always)]
    pub fn optsr_cur_mut(&self) -> &mut OPTSR_CUR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut OPTSR_CUR) }
    }
    #[doc = "0x20 - FLASH option status register"]
    #[inline(always)]
    pub fn optsr_prg(&self) -> &OPTSR_PRG {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const OPTSR_PRG) }
    }
    #[doc = "0x20 - FLASH option status register"]
    #[inline(always)]
    pub fn optsr_prg_mut(&self) -> &mut OPTSR_PRG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut OPTSR_PRG) }
    }
    #[doc = "0x24 - FLASH option clear control register"]
    #[inline(always)]
    pub fn optccr(&self) -> &OPTCCR {
        unsafe { &*(((self as *const Self) as *const u8).add(36usize) as *const OPTCCR) }
    }
    #[doc = "0x24 - FLASH option clear control register"]
    #[inline(always)]
    pub fn optccr_mut(&self) -> &mut OPTCCR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(36usize) as *mut OPTCCR) }
    }
    #[doc = "0x40 - FLASH register with boot address"]
    #[inline(always)]
    pub fn boot_curr(&self) -> &BOOT_CURR {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const BOOT_CURR) }
    }
    #[doc = "0x40 - FLASH register with boot address"]
    #[inline(always)]
    pub fn boot_curr_mut(&self) -> &mut BOOT_CURR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut BOOT_CURR) }
    }
    #[doc = "0x44 - FLASH register with boot address"]
    #[inline(always)]
    pub fn boot_prgr(&self) -> &BOOT_PRGR {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const BOOT_PRGR) }
    }
    #[doc = "0x44 - FLASH register with boot address"]
    #[inline(always)]
    pub fn boot_prgr_mut(&self) -> &mut BOOT_PRGR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut BOOT_PRGR) }
    }
    #[doc = "0x5c - FLASH CRC data register"]
    #[inline(always)]
    pub fn crcdatar(&self) -> &CRCDATAR {
        unsafe { &*(((self as *const Self) as *const u8).add(92usize) as *const CRCDATAR) }
    }
    #[doc = "0x5c - FLASH CRC data register"]
    #[inline(always)]
    pub fn crcdatar_mut(&self) -> &mut CRCDATAR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(92usize) as *mut CRCDATAR) }
    }
    #[doc = "0x104 - Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
    #[inline(always)]
    pub fn bank2(&self) -> &BANK {
        unsafe { &*(((self as *const Self) as *const u8).add(260usize) as *const BANK) }
    }
    #[doc = "0x104 - Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
    #[inline(always)]
    pub fn bank2_mut(&self) -> &mut BANK {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(260usize) as *mut BANK) }
    }
    #[doc = "0x108 - FLASH option key register"]
    #[inline(always)]
    pub fn optkeyr_(&self) -> &OPTKEYR_ {
        unsafe { &*(((self as *const Self) as *const u8).add(264usize) as *const OPTKEYR_) }
    }
    #[doc = "0x108 - FLASH option key register"]
    #[inline(always)]
    pub fn optkeyr__mut(&self) -> &mut OPTKEYR_ {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(264usize) as *mut OPTKEYR_) }
    }
    #[doc = "0x118 - FLASH option control register"]
    #[inline(always)]
    pub fn optcr_(&self) -> &OPTCR_ {
        unsafe { &*(((self as *const Self) as *const u8).add(280usize) as *const OPTCR_) }
    }
    #[doc = "0x118 - FLASH option control register"]
    #[inline(always)]
    pub fn optcr__mut(&self) -> &mut OPTCR_ {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(280usize) as *mut OPTCR_) }
    }
    #[doc = "0x11c - FLASH option status register"]
    #[inline(always)]
    pub fn optsr_cur_(&self) -> &OPTSR_CUR_ {
        unsafe { &*(((self as *const Self) as *const u8).add(284usize) as *const OPTSR_CUR_) }
    }
    #[doc = "0x11c - FLASH option status register"]
    #[inline(always)]
    pub fn optsr_cur__mut(&self) -> &mut OPTSR_CUR_ {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(284usize) as *mut OPTSR_CUR_) }
    }
    #[doc = "0x120 - FLASH option status register"]
    #[inline(always)]
    pub fn optsr_prg_(&self) -> &OPTSR_PRG_ {
        unsafe { &*(((self as *const Self) as *const u8).add(288usize) as *const OPTSR_PRG_) }
    }
    #[doc = "0x120 - FLASH option status register"]
    #[inline(always)]
    pub fn optsr_prg__mut(&self) -> &mut OPTSR_PRG_ {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(288usize) as *mut OPTSR_PRG_) }
    }
    #[doc = "0x124 - FLASH option clear control register"]
    #[inline(always)]
    pub fn optccr_(&self) -> &OPTCCR_ {
        unsafe { &*(((self as *const Self) as *const u8).add(292usize) as *const OPTCCR_) }
    }
    #[doc = "0x124 - FLASH option clear control register"]
    #[inline(always)]
    pub fn optccr__mut(&self) -> &mut OPTCCR_ {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(292usize) as *mut OPTCCR_) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct BANK {
    #[doc = "0x00 - FLASH key register for bank 1"]
    pub keyr: self::bank::KEYR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - FLASH control register for bank 1"]
    pub cr: self::bank::CR,
    #[doc = "0x0c - FLASH status register for bank 1"]
    pub sr: self::bank::SR,
    #[doc = "0x10 - FLASH clear control register for bank 1"]
    pub ccr: self::bank::CCR,
    _reserved4: [u8; 16usize],
    #[doc = "0x24 - FLASH protection address for bank 1"]
    pub prar_cur: self::bank::PRAR_CUR,
    #[doc = "0x28 - FLASH protection address for bank 1"]
    pub prar_prg: self::bank::PRAR_PRG,
    #[doc = "0x2c - FLASH secure address for bank 1"]
    pub scar_cur: self::bank::SCAR_CUR,
    #[doc = "0x30 - FLASH secure address for bank 1"]
    pub scar_prg: self::bank::SCAR_PRG,
    #[doc = "0x34 - FLASH write sector protection for bank 1"]
    pub wpsn_curr: self::bank::WPSN_CURR,
    #[doc = "0x38 - FLASH write sector protection for bank 1"]
    pub wpsn_prgr: self::bank::WPSN_PRGR,
    _reserved10: [u8; 16usize],
    #[doc = "0x4c - FLASH CRC control register for bank 1"]
    pub crccr: self::bank::CRCCR,
    #[doc = "0x50 - FLASH CRC start address register for bank 1"]
    pub crcsaddr: self::bank::CRCSADDR,
    #[doc = "0x54 - FLASH CRC end address register for bank 1"]
    pub crceaddr: self::bank::CRCEADDR,
    _reserved13: [u8; 4usize],
    #[doc = "0x5c - FLASH ECC fail address for bank 1"]
    pub far: self::bank::FAR,
}
#[doc = r"Register block"]
#[doc = "Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
pub mod bank;
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
#[doc = "Access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr_](acr_) module"]
pub type ACR_ = crate::Reg<u32, _ACR_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACR_;
#[doc = "`read()` method returns [acr_::R](acr_::R) reader structure"]
impl crate::Readable for ACR_ {}
#[doc = "`write(|w| ..)` method takes [acr_::W](acr_::W) writer structure"]
impl crate::Writable for ACR_ {}
#[doc = "Access control register"]
pub mod acr_;
#[doc = "FLASH option key register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optkeyr](optkeyr) module"]
pub type OPTKEYR = crate::Reg<u32, _OPTKEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTKEYR;
#[doc = "`read()` method returns [optkeyr::R](optkeyr::R) reader structure"]
impl crate::Readable for OPTKEYR {}
#[doc = "`write(|w| ..)` method takes [optkeyr::W](optkeyr::W) writer structure"]
impl crate::Writable for OPTKEYR {}
#[doc = "FLASH option key register"]
pub mod optkeyr;
#[doc = "FLASH option key register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optkeyr_](optkeyr_) module"]
pub type OPTKEYR_ = crate::Reg<u32, _OPTKEYR_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTKEYR_;
#[doc = "`read()` method returns [optkeyr_::R](optkeyr_::R) reader structure"]
impl crate::Readable for OPTKEYR_ {}
#[doc = "`write(|w| ..)` method takes [optkeyr_::W](optkeyr_::W) writer structure"]
impl crate::Writable for OPTKEYR_ {}
#[doc = "FLASH option key register"]
pub mod optkeyr_;
#[doc = "FLASH option control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optcr](optcr) module"]
pub type OPTCR = crate::Reg<u32, _OPTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTCR;
#[doc = "`read()` method returns [optcr::R](optcr::R) reader structure"]
impl crate::Readable for OPTCR {}
#[doc = "`write(|w| ..)` method takes [optcr::W](optcr::W) writer structure"]
impl crate::Writable for OPTCR {}
#[doc = "FLASH option control register"]
pub mod optcr;
#[doc = "FLASH option control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optcr_](optcr_) module"]
pub type OPTCR_ = crate::Reg<u32, _OPTCR_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTCR_;
#[doc = "`read()` method returns [optcr_::R](optcr_::R) reader structure"]
impl crate::Readable for OPTCR_ {}
#[doc = "`write(|w| ..)` method takes [optcr_::W](optcr_::W) writer structure"]
impl crate::Writable for OPTCR_ {}
#[doc = "FLASH option control register"]
pub mod optcr_;
#[doc = "FLASH option status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optsr_cur_](optsr_cur_) module"]
pub type OPTSR_CUR_ = crate::Reg<u32, _OPTSR_CUR_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTSR_CUR_;
#[doc = "`read()` method returns [optsr_cur_::R](optsr_cur_::R) reader structure"]
impl crate::Readable for OPTSR_CUR_ {}
#[doc = "`write(|w| ..)` method takes [optsr_cur_::W](optsr_cur_::W) writer structure"]
impl crate::Writable for OPTSR_CUR_ {}
#[doc = "FLASH option status register"]
pub mod optsr_cur_;
#[doc = "FLASH option status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optsr_cur](optsr_cur) module"]
pub type OPTSR_CUR = crate::Reg<u32, _OPTSR_CUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTSR_CUR;
#[doc = "`read()` method returns [optsr_cur::R](optsr_cur::R) reader structure"]
impl crate::Readable for OPTSR_CUR {}
#[doc = "`write(|w| ..)` method takes [optsr_cur::W](optsr_cur::W) writer structure"]
impl crate::Writable for OPTSR_CUR {}
#[doc = "FLASH option status register"]
pub mod optsr_cur;
#[doc = "FLASH option status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optsr_prg](optsr_prg) module"]
pub type OPTSR_PRG = crate::Reg<u32, _OPTSR_PRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTSR_PRG;
#[doc = "`read()` method returns [optsr_prg::R](optsr_prg::R) reader structure"]
impl crate::Readable for OPTSR_PRG {}
#[doc = "`write(|w| ..)` method takes [optsr_prg::W](optsr_prg::W) writer structure"]
impl crate::Writable for OPTSR_PRG {}
#[doc = "FLASH option status register"]
pub mod optsr_prg;
#[doc = "FLASH option status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optsr_prg_](optsr_prg_) module"]
pub type OPTSR_PRG_ = crate::Reg<u32, _OPTSR_PRG_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTSR_PRG_;
#[doc = "`read()` method returns [optsr_prg_::R](optsr_prg_::R) reader structure"]
impl crate::Readable for OPTSR_PRG_ {}
#[doc = "`write(|w| ..)` method takes [optsr_prg_::W](optsr_prg_::W) writer structure"]
impl crate::Writable for OPTSR_PRG_ {}
#[doc = "FLASH option status register"]
pub mod optsr_prg_;
#[doc = "FLASH option clear control register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optccr_](optccr_) module"]
pub type OPTCCR_ = crate::Reg<u32, _OPTCCR_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTCCR_;
#[doc = "`write(|w| ..)` method takes [optccr_::W](optccr_::W) writer structure"]
impl crate::Writable for OPTCCR_ {}
#[doc = "FLASH option clear control register"]
pub mod optccr_;
#[doc = "FLASH option clear control register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optccr](optccr) module"]
pub type OPTCCR = crate::Reg<u32, _OPTCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTCCR;
#[doc = "`write(|w| ..)` method takes [optccr::W](optccr::W) writer structure"]
impl crate::Writable for OPTCCR {}
#[doc = "FLASH option clear control register"]
pub mod optccr;
#[doc = "FLASH register with boot address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boot_curr](boot_curr) module"]
pub type BOOT_CURR = crate::Reg<u32, _BOOT_CURR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOOT_CURR;
#[doc = "`read()` method returns [boot_curr::R](boot_curr::R) reader structure"]
impl crate::Readable for BOOT_CURR {}
#[doc = "FLASH register with boot address"]
pub mod boot_curr;
#[doc = "FLASH register with boot address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boot_prgr](boot_prgr) module"]
pub type BOOT_PRGR = crate::Reg<u32, _BOOT_PRGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOOT_PRGR;
#[doc = "`read()` method returns [boot_prgr::R](boot_prgr::R) reader structure"]
impl crate::Readable for BOOT_PRGR {}
#[doc = "`write(|w| ..)` method takes [boot_prgr::W](boot_prgr::W) writer structure"]
impl crate::Writable for BOOT_PRGR {}
#[doc = "FLASH register with boot address"]
pub mod boot_prgr;
#[doc = "FLASH CRC data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcdatar](crcdatar) module"]
pub type CRCDATAR = crate::Reg<u32, _CRCDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCDATAR;
#[doc = "`read()` method returns [crcdatar::R](crcdatar::R) reader structure"]
impl crate::Readable for CRCDATAR {}
#[doc = "`write(|w| ..)` method takes [crcdatar::W](crcdatar::W) writer structure"]
impl crate::Writable for CRCDATAR {}
#[doc = "FLASH CRC data register"]
pub mod crcdatar;
