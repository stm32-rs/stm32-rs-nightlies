#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FLASH access control register"]
    pub acr: crate::Reg<acr::ACR_SPEC>,
    _reserved_1_bank1: [u8; 0x60],
    _reserved2: [u8; 0xa0],
    #[doc = "0x104..0x164 - Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
    pub bank2: BANK,
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
    #[doc = "0x40 - FLASH register boot address for Arm Cortex-M7 core"]
    #[inline(always)]
    pub fn boot7_curr(&self) -> &crate::Reg<boot7_curr::BOOT7_CURR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize)
                as *const crate::Reg<boot7_curr::BOOT7_CURR_SPEC>)
        }
    }
    #[doc = "0x44 - FLASH register boot address for Arm Cortex-M7 core"]
    #[inline(always)]
    pub fn boot7_prgr(&self) -> &crate::Reg<boot7_prgr::BOOT7_PRGR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const crate::Reg<boot7_prgr::BOOT7_PRGR_SPEC>)
        }
    }
    #[doc = "0x48 - FLASH register boot address for Arm Cortex-M4 core"]
    #[inline(always)]
    pub fn boot4_curr(&self) -> &crate::Reg<boot4_curr::BOOT4_CURR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(72usize)
                as *const crate::Reg<boot4_curr::BOOT4_CURR_SPEC>)
        }
    }
    #[doc = "0x4c - FLASH register boot address for Arm Cortex-M4 core"]
    #[inline(always)]
    pub fn boot4_prgr(&self) -> &crate::Reg<boot4_prgr::BOOT4_PRGR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(76usize)
                as *const crate::Reg<boot4_prgr::BOOT4_PRGR_SPEC>)
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
#[doc = "FLASH access control register"]
pub mod acr;
#[doc = "OPTKEYR register accessor: an alias for `Reg<OPTKEYR_SPEC>`"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
#[doc = "FLASH option key register"]
pub mod optkeyr;
#[doc = "OPTCR register accessor: an alias for `Reg<OPTCR_SPEC>`"]
pub type OPTCR = crate::Reg<optcr::OPTCR_SPEC>;
#[doc = "FLASH option control register"]
pub mod optcr;
#[doc = "OPTSR_CUR register accessor: an alias for `Reg<OPTSR_CUR_SPEC>`"]
pub type OPTSR_CUR = crate::Reg<optsr_cur::OPTSR_CUR_SPEC>;
#[doc = "FLASH option status register"]
pub mod optsr_cur;
#[doc = "OPTSR_PRG register accessor: an alias for `Reg<OPTSR_PRG_SPEC>`"]
pub type OPTSR_PRG = crate::Reg<optsr_prg::OPTSR_PRG_SPEC>;
#[doc = "FLASH option status register"]
pub mod optsr_prg;
#[doc = "OPTCCR register accessor: an alias for `Reg<OPTCCR_SPEC>`"]
pub type OPTCCR = crate::Reg<optccr::OPTCCR_SPEC>;
#[doc = "FLASH option clear control register"]
pub mod optccr;
#[doc = "BOOT7_CURR register accessor: an alias for `Reg<BOOT7_CURR_SPEC>`"]
pub type BOOT7_CURR = crate::Reg<boot7_curr::BOOT7_CURR_SPEC>;
#[doc = "FLASH register boot address for Arm Cortex-M7 core"]
pub mod boot7_curr;
#[doc = "BOOT7_PRGR register accessor: an alias for `Reg<BOOT7_PRGR_SPEC>`"]
pub type BOOT7_PRGR = crate::Reg<boot7_prgr::BOOT7_PRGR_SPEC>;
#[doc = "FLASH register boot address for Arm Cortex-M7 core"]
pub mod boot7_prgr;
#[doc = "BOOT4_CURR register accessor: an alias for `Reg<BOOT4_CURR_SPEC>`"]
pub type BOOT4_CURR = crate::Reg<boot4_curr::BOOT4_CURR_SPEC>;
#[doc = "FLASH register boot address for Arm Cortex-M4 core"]
pub mod boot4_curr;
#[doc = "BOOT4_PRGR register accessor: an alias for `Reg<BOOT4_PRGR_SPEC>`"]
pub type BOOT4_PRGR = crate::Reg<boot4_prgr::BOOT4_PRGR_SPEC>;
#[doc = "FLASH register boot address for Arm Cortex-M4 core"]
pub mod boot4_prgr;
#[doc = "CRCDATAR register accessor: an alias for `Reg<CRCDATAR_SPEC>`"]
pub type CRCDATAR = crate::Reg<crcdatar::CRCDATAR_SPEC>;
#[doc = "FLASH CRC data register"]
pub mod crcdatar;
