#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: crate::Reg<acr::ACR_SPEC>,
    _reserved_1_bank1: [u8; 0x60],
    _reserved2: [u8; 0x9c],
    #[doc = "0x100 - Access control register"]
    pub acr_: crate::Reg<acr_::ACR__SPEC>,
    _reserved_3_bank2: [u8; 0x60],
}
impl RegisterBlock {
    #[doc = "0x04..0x64 - Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
    #[inline(always)]
    pub fn bank1(&self) -> &BANK {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const BANK) }
    }
    #[doc = "0x08 - FLASH option key register"]
    #[inline(always)]
    pub fn optkeyr(&self) -> &crate::Reg<optkeyr::OPTKEYR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<optkeyr::OPTKEYR_SPEC>)
        }
    }
    #[doc = "0x18 - FLASH option control register"]
    #[inline(always)]
    pub fn optcr(&self) -> &crate::Reg<optcr::OPTCR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<optcr::OPTCR_SPEC>)
        }
    }
    #[doc = "0x1c - FLASH option status register"]
    #[inline(always)]
    pub fn optsr_cur(&self) -> &crate::Reg<optsr_cur::OPTSR_CUR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<optsr_cur::OPTSR_CUR_SPEC>)
        }
    }
    #[doc = "0x20 - FLASH option status register"]
    #[inline(always)]
    pub fn optsr_prg(&self) -> &crate::Reg<optsr_prg::OPTSR_PRG_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<optsr_prg::OPTSR_PRG_SPEC>)
        }
    }
    #[doc = "0x24 - FLASH option clear control register"]
    #[inline(always)]
    pub fn optccr(&self) -> &crate::Reg<optccr::OPTCCR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(36usize)
                as *const crate::Reg<optccr::OPTCCR_SPEC>)
        }
    }
    #[doc = "0x40 - FLASH register with boot address"]
    #[inline(always)]
    pub fn boot_curr(&self) -> &crate::Reg<boot_curr::BOOT_CURR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize)
                as *const crate::Reg<boot_curr::BOOT_CURR_SPEC>)
        }
    }
    #[doc = "0x44 - FLASH register with boot address"]
    #[inline(always)]
    pub fn boot_prgr(&self) -> &crate::Reg<boot_prgr::BOOT_PRGR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const crate::Reg<boot_prgr::BOOT_PRGR_SPEC>)
        }
    }
    #[doc = "0x5c - FLASH CRC data register"]
    #[inline(always)]
    pub fn crcdatar(&self) -> &crate::Reg<crcdatar::CRCDATAR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(92usize)
                as *const crate::Reg<crcdatar::CRCDATAR_SPEC>)
        }
    }
    #[doc = "0x104..0x164 - Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
    #[inline(always)]
    pub fn bank2(&self) -> &BANK {
        unsafe { &*(((self as *const Self) as *const u8).add(260usize) as *const BANK) }
    }
    #[doc = "0x108 - FLASH option key register"]
    #[inline(always)]
    pub fn optkeyr_(&self) -> &crate::Reg<optkeyr_::OPTKEYR__SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(264usize)
                as *const crate::Reg<optkeyr_::OPTKEYR__SPEC>)
        }
    }
    #[doc = "0x118 - FLASH option control register"]
    #[inline(always)]
    pub fn optcr_(&self) -> &crate::Reg<optcr_::OPTCR__SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(280usize)
                as *const crate::Reg<optcr_::OPTCR__SPEC>)
        }
    }
    #[doc = "0x11c - FLASH option status register"]
    #[inline(always)]
    pub fn optsr_cur_(&self) -> &crate::Reg<optsr_cur_::OPTSR_CUR__SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(284usize)
                as *const crate::Reg<optsr_cur_::OPTSR_CUR__SPEC>)
        }
    }
    #[doc = "0x120 - FLASH option status register"]
    #[inline(always)]
    pub fn optsr_prg_(&self) -> &crate::Reg<optsr_prg_::OPTSR_PRG__SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(288usize)
                as *const crate::Reg<optsr_prg_::OPTSR_PRG__SPEC>)
        }
    }
    #[doc = "0x124 - FLASH option clear control register"]
    #[inline(always)]
    pub fn optccr_(&self) -> &crate::Reg<optccr_::OPTCCR__SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(292usize)
                as *const crate::Reg<optccr_::OPTCCR__SPEC>)
        }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct BANK {
    #[doc = "0x00 - FLASH key register for bank 1"]
    pub keyr: crate::Reg<self::bank::keyr::KEYR_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - FLASH control register for bank 1"]
    pub cr: crate::Reg<self::bank::cr::CR_SPEC>,
    #[doc = "0x0c - FLASH status register for bank 1"]
    pub sr: crate::Reg<self::bank::sr::SR_SPEC>,
    #[doc = "0x10 - FLASH clear control register for bank 1"]
    pub ccr: crate::Reg<self::bank::ccr::CCR_SPEC>,
    _reserved4: [u8; 0x10],
    #[doc = "0x24 - FLASH protection address for bank 1"]
    pub prar_cur: crate::Reg<self::bank::prar_cur::PRAR_CUR_SPEC>,
    #[doc = "0x28 - FLASH protection address for bank 1"]
    pub prar_prg: crate::Reg<self::bank::prar_prg::PRAR_PRG_SPEC>,
    #[doc = "0x2c - FLASH secure address for bank 1"]
    pub scar_cur: crate::Reg<self::bank::scar_cur::SCAR_CUR_SPEC>,
    #[doc = "0x30 - FLASH secure address for bank 1"]
    pub scar_prg: crate::Reg<self::bank::scar_prg::SCAR_PRG_SPEC>,
    #[doc = "0x34 - FLASH write sector protection for bank 1"]
    pub wpsn_curr: crate::Reg<self::bank::wpsn_curr::WPSN_CURR_SPEC>,
    #[doc = "0x38 - FLASH write sector protection for bank 1"]
    pub wpsn_prgr: crate::Reg<self::bank::wpsn_prgr::WPSN_PRGR_SPEC>,
    _reserved10: [u8; 0x10],
    #[doc = "0x4c - FLASH CRC control register for bank 1"]
    pub crccr: crate::Reg<self::bank::crccr::CRCCR_SPEC>,
    #[doc = "0x50 - FLASH CRC start address register for bank 1"]
    pub crcsaddr: crate::Reg<self::bank::crcsaddr::CRCSADDR_SPEC>,
    #[doc = "0x54 - FLASH CRC end address register for bank 1"]
    pub crceaddr: crate::Reg<self::bank::crceaddr::CRCEADDR_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x5c - FLASH ECC fail address for bank 1"]
    pub far: crate::Reg<self::bank::far::FAR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
pub mod bank;
#[doc = "ACR register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Access control register"]
pub mod acr;
#[doc = "ACR_ register accessor: an alias for `Reg<ACR__SPEC>`"]
pub type ACR_ = crate::Reg<acr_::ACR__SPEC>;
#[doc = "Access control register"]
pub mod acr_;
#[doc = "OPTKEYR register accessor: an alias for `Reg<OPTKEYR_SPEC>`"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
#[doc = "FLASH option key register"]
pub mod optkeyr;
#[doc = "OPTKEYR_ register accessor: an alias for `Reg<OPTKEYR__SPEC>`"]
pub type OPTKEYR_ = crate::Reg<optkeyr_::OPTKEYR__SPEC>;
#[doc = "FLASH option key register"]
pub mod optkeyr_;
#[doc = "OPTCR register accessor: an alias for `Reg<OPTCR_SPEC>`"]
pub type OPTCR = crate::Reg<optcr::OPTCR_SPEC>;
#[doc = "FLASH option control register"]
pub mod optcr;
#[doc = "OPTCR_ register accessor: an alias for `Reg<OPTCR__SPEC>`"]
pub type OPTCR_ = crate::Reg<optcr_::OPTCR__SPEC>;
#[doc = "FLASH option control register"]
pub mod optcr_;
#[doc = "OPTSR_CUR_ register accessor: an alias for `Reg<OPTSR_CUR__SPEC>`"]
pub type OPTSR_CUR_ = crate::Reg<optsr_cur_::OPTSR_CUR__SPEC>;
#[doc = "FLASH option status register"]
pub mod optsr_cur_;
#[doc = "OPTSR_CUR register accessor: an alias for `Reg<OPTSR_CUR_SPEC>`"]
pub type OPTSR_CUR = crate::Reg<optsr_cur::OPTSR_CUR_SPEC>;
#[doc = "FLASH option status register"]
pub mod optsr_cur;
#[doc = "OPTSR_PRG register accessor: an alias for `Reg<OPTSR_PRG_SPEC>`"]
pub type OPTSR_PRG = crate::Reg<optsr_prg::OPTSR_PRG_SPEC>;
#[doc = "FLASH option status register"]
pub mod optsr_prg;
#[doc = "OPTSR_PRG_ register accessor: an alias for `Reg<OPTSR_PRG__SPEC>`"]
pub type OPTSR_PRG_ = crate::Reg<optsr_prg_::OPTSR_PRG__SPEC>;
#[doc = "FLASH option status register"]
pub mod optsr_prg_;
#[doc = "OPTCCR_ register accessor: an alias for `Reg<OPTCCR__SPEC>`"]
pub type OPTCCR_ = crate::Reg<optccr_::OPTCCR__SPEC>;
#[doc = "FLASH option clear control register"]
pub mod optccr_;
#[doc = "OPTCCR register accessor: an alias for `Reg<OPTCCR_SPEC>`"]
pub type OPTCCR = crate::Reg<optccr::OPTCCR_SPEC>;
#[doc = "FLASH option clear control register"]
pub mod optccr;
#[doc = "BOOT_CURR register accessor: an alias for `Reg<BOOT_CURR_SPEC>`"]
pub type BOOT_CURR = crate::Reg<boot_curr::BOOT_CURR_SPEC>;
#[doc = "FLASH register with boot address"]
pub mod boot_curr;
#[doc = "BOOT_PRGR register accessor: an alias for `Reg<BOOT_PRGR_SPEC>`"]
pub type BOOT_PRGR = crate::Reg<boot_prgr::BOOT_PRGR_SPEC>;
#[doc = "FLASH register with boot address"]
pub mod boot_prgr;
#[doc = "CRCDATAR register accessor: an alias for `Reg<CRCDATAR_SPEC>`"]
pub type CRCDATAR = crate::Reg<crcdatar::CRCDATAR_SPEC>;
#[doc = "FLASH CRC data register"]
pub mod crcdatar;
