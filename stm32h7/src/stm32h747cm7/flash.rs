#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FLASH access control register"]
    pub acr: ACR,
    _reserved_1_bank1: [u8; 96usize],
    _reserved2: [u8; 160usize],
    #[doc = "0x104 - Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
    pub bank2: BANK,
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
    #[doc = "0x40 - FLASH register boot address for Arm Cortex-M7 core"]
    #[inline(always)]
    pub fn boot7_curr(&self) -> &BOOT7_CURR {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const BOOT7_CURR) }
    }
    #[doc = "0x40 - FLASH register boot address for Arm Cortex-M7 core"]
    #[inline(always)]
    pub fn boot7_curr_mut(&self) -> &mut BOOT7_CURR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut BOOT7_CURR) }
    }
    #[doc = "0x44 - FLASH register boot address for Arm Cortex-M7 core"]
    #[inline(always)]
    pub fn boot7_prgr(&self) -> &BOOT7_PRGR {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const BOOT7_PRGR) }
    }
    #[doc = "0x44 - FLASH register boot address for Arm Cortex-M7 core"]
    #[inline(always)]
    pub fn boot7_prgr_mut(&self) -> &mut BOOT7_PRGR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut BOOT7_PRGR) }
    }
    #[doc = "0x48 - FLASH register boot address for Arm Cortex-M4 core"]
    #[inline(always)]
    pub fn boot4_curr(&self) -> &BOOT4_CURR {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const BOOT4_CURR) }
    }
    #[doc = "0x48 - FLASH register boot address for Arm Cortex-M4 core"]
    #[inline(always)]
    pub fn boot4_curr_mut(&self) -> &mut BOOT4_CURR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut BOOT4_CURR) }
    }
    #[doc = "0x4c - FLASH register boot address for Arm Cortex-M4 core"]
    #[inline(always)]
    pub fn boot4_prgr(&self) -> &BOOT4_PRGR {
        unsafe { &*(((self as *const Self) as *const u8).add(76usize) as *const BOOT4_PRGR) }
    }
    #[doc = "0x4c - FLASH register boot address for Arm Cortex-M4 core"]
    #[inline(always)]
    pub fn boot4_prgr_mut(&self) -> &mut BOOT4_PRGR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(76usize) as *mut BOOT4_PRGR) }
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
#[doc = "FLASH access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](acr) module"]
pub type ACR = crate::Reg<u32, _ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACR;
#[doc = "`read()` method returns [acr::R](acr::R) reader structure"]
impl crate::Readable for ACR {}
#[doc = "`write(|w| ..)` method takes [acr::W](acr::W) writer structure"]
impl crate::Writable for ACR {}
#[doc = "FLASH access control register"]
pub mod acr;
#[doc = "FLASH option key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optkeyr](optkeyr) module"]
pub type OPTKEYR = crate::Reg<u32, _OPTKEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTKEYR;
#[doc = "`write(|w| ..)` method takes [optkeyr::W](optkeyr::W) writer structure"]
impl crate::Writable for OPTKEYR {}
#[doc = "FLASH option key register"]
pub mod optkeyr;
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
#[doc = "FLASH option clear control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optccr](optccr) module"]
pub type OPTCCR = crate::Reg<u32, _OPTCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTCCR;
#[doc = "`read()` method returns [optccr::R](optccr::R) reader structure"]
impl crate::Readable for OPTCCR {}
#[doc = "`write(|w| ..)` method takes [optccr::W](optccr::W) writer structure"]
impl crate::Writable for OPTCCR {}
#[doc = "FLASH option clear control register"]
pub mod optccr;
#[doc = "FLASH register boot address for Arm Cortex-M7 core\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boot7_curr](boot7_curr) module"]
pub type BOOT7_CURR = crate::Reg<u32, _BOOT7_CURR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOOT7_CURR;
#[doc = "`read()` method returns [boot7_curr::R](boot7_curr::R) reader structure"]
impl crate::Readable for BOOT7_CURR {}
#[doc = "`write(|w| ..)` method takes [boot7_curr::W](boot7_curr::W) writer structure"]
impl crate::Writable for BOOT7_CURR {}
#[doc = "FLASH register boot address for Arm Cortex-M7 core"]
pub mod boot7_curr;
#[doc = "FLASH register boot address for Arm Cortex-M7 core\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boot7_prgr](boot7_prgr) module"]
pub type BOOT7_PRGR = crate::Reg<u32, _BOOT7_PRGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOOT7_PRGR;
#[doc = "`read()` method returns [boot7_prgr::R](boot7_prgr::R) reader structure"]
impl crate::Readable for BOOT7_PRGR {}
#[doc = "`write(|w| ..)` method takes [boot7_prgr::W](boot7_prgr::W) writer structure"]
impl crate::Writable for BOOT7_PRGR {}
#[doc = "FLASH register boot address for Arm Cortex-M7 core"]
pub mod boot7_prgr;
#[doc = "FLASH register boot address for Arm Cortex-M4 core\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boot4_curr](boot4_curr) module"]
pub type BOOT4_CURR = crate::Reg<u32, _BOOT4_CURR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOOT4_CURR;
#[doc = "`read()` method returns [boot4_curr::R](boot4_curr::R) reader structure"]
impl crate::Readable for BOOT4_CURR {}
#[doc = "`write(|w| ..)` method takes [boot4_curr::W](boot4_curr::W) writer structure"]
impl crate::Writable for BOOT4_CURR {}
#[doc = "FLASH register boot address for Arm Cortex-M4 core"]
pub mod boot4_curr;
#[doc = "FLASH register boot address for Arm Cortex-M4 core\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boot4_prgr](boot4_prgr) module"]
pub type BOOT4_PRGR = crate::Reg<u32, _BOOT4_PRGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOOT4_PRGR;
#[doc = "`read()` method returns [boot4_prgr::R](boot4_prgr::R) reader structure"]
impl crate::Readable for BOOT4_PRGR {}
#[doc = "`write(|w| ..)` method takes [boot4_prgr::W](boot4_prgr::W) writer structure"]
impl crate::Writable for BOOT4_PRGR {}
#[doc = "FLASH register boot address for Arm Cortex-M4 core"]
pub mod boot4_prgr;
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
