///Register `OPTR` reader
pub type R = crate::R<OPTRrs>;
/**Read protection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDPROT {
    ///0: Level 1
    Level1 = 0,
    ///170: Level 0
    Level0 = 170,
    ///204: Level 2
    Level2 = 204,
}
impl From<RDPROT> for u8 {
    #[inline(always)]
    fn from(variant: RDPROT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RDPROT {
    type Ux = u8;
}
impl crate::IsEnum for RDPROT {}
///Field `RDPROT` reader - Read protection
pub type RDPROT_R = crate::FieldReader<RDPROT>;
impl RDPROT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RDPROT> {
        match self.bits {
            0 => Some(RDPROT::Level1),
            170 => Some(RDPROT::Level0),
            204 => Some(RDPROT::Level2),
            _ => None,
        }
    }
    ///Level 1
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == RDPROT::Level1
    }
    ///Level 0
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == RDPROT::Level0
    }
    ///Level 2
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RDPROT::Level2
    }
}
/**WPRMOD

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPRMOD {
    ///0: PCROP disabled. The WRPROT bits are used as a write protection on a sector.
    Disabled = 0,
    ///1: PCROP enabled. The WRPROT bits are used as a read protection on a sector.
    Enabled = 1,
}
impl From<WPRMOD> for bool {
    #[inline(always)]
    fn from(variant: WPRMOD) -> Self {
        variant as u8 != 0
    }
}
///Field `WPRMOD` reader - WPRMOD
pub type WPRMOD_R = crate::BitReader<WPRMOD>;
impl WPRMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WPRMOD {
        match self.bits {
            false => WPRMOD::Disabled,
            true => WPRMOD::Enabled,
        }
    }
    ///PCROP disabled. The WRPROT bits are used as a write protection on a sector.
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WPRMOD::Disabled
    }
    ///PCROP enabled. The WRPROT bits are used as a read protection on a sector.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WPRMOD::Enabled
    }
}
/**BOR_LEV

Value on reset: 8*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOR_LEV {
    ///0: This is the reset threshold level for the 1.45 V - 1.55 V voltage range (power-down only)
    BorOff = 0,
    ///1: Reset threshold level for VBOR0 (around 1.8 V)
    BorLevel1 = 1,
    ///2: Reset threshold level for VBOR1 (around 2.0 V)
    BorLevel2 = 2,
    ///3: Reset threshold level for VBOR2 (around 2.5 V)
    BorLevel3 = 3,
    ///4: Reset threshold level for VBOR3 (around 2.7 V)
    BorLevel4 = 4,
    ///5: Reset threshold level for VBOR4 (around 3.0 V)
    BorLevel5 = 5,
}
impl From<BOR_LEV> for u8 {
    #[inline(always)]
    fn from(variant: BOR_LEV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BOR_LEV {
    type Ux = u8;
}
impl crate::IsEnum for BOR_LEV {}
///Field `BOR_LEV` reader - BOR_LEV
pub type BOR_LEV_R = crate::FieldReader<BOR_LEV>;
impl BOR_LEV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<BOR_LEV> {
        match self.bits {
            0 => Some(BOR_LEV::BorOff),
            1 => Some(BOR_LEV::BorLevel1),
            2 => Some(BOR_LEV::BorLevel2),
            3 => Some(BOR_LEV::BorLevel3),
            4 => Some(BOR_LEV::BorLevel4),
            5 => Some(BOR_LEV::BorLevel5),
            _ => None,
        }
    }
    ///This is the reset threshold level for the 1.45 V - 1.55 V voltage range (power-down only)
    #[inline(always)]
    pub fn is_bor_off(&self) -> bool {
        *self == BOR_LEV::BorOff
    }
    ///Reset threshold level for VBOR0 (around 1.8 V)
    #[inline(always)]
    pub fn is_bor_level1(&self) -> bool {
        *self == BOR_LEV::BorLevel1
    }
    ///Reset threshold level for VBOR1 (around 2.0 V)
    #[inline(always)]
    pub fn is_bor_level2(&self) -> bool {
        *self == BOR_LEV::BorLevel2
    }
    ///Reset threshold level for VBOR2 (around 2.5 V)
    #[inline(always)]
    pub fn is_bor_level3(&self) -> bool {
        *self == BOR_LEV::BorLevel3
    }
    ///Reset threshold level for VBOR3 (around 2.7 V)
    #[inline(always)]
    pub fn is_bor_level4(&self) -> bool {
        *self == BOR_LEV::BorLevel4
    }
    ///Reset threshold level for VBOR4 (around 3.0 V)
    #[inline(always)]
    pub fn is_bor_level5(&self) -> bool {
        *self == BOR_LEV::BorLevel5
    }
}
///Field `WDG_SW` reader - WDG_SW
pub type WDG_SW_R = crate::BitReader;
///Field `nRST_STOP` reader - nRST_STOP
pub type N_RST_STOP_R = crate::BitReader;
///Field `nRST_STDBY` reader - nRST_STDBY
pub type N_RST_STDBY_R = crate::BitReader;
///Field `BFB2` reader - BFB2
pub type BFB2_R = crate::BitReader;
///Field `nBOOT1` reader - nBOOT1
pub type N_BOOT1_R = crate::BitReader;
impl R {
    ///Bits 0:7 - Read protection
    #[inline(always)]
    pub fn rdprot(&self) -> RDPROT_R {
        RDPROT_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - WPRMOD
    #[inline(always)]
    pub fn wprmod(&self) -> WPRMOD_R {
        WPRMOD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:19 - BOR_LEV
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - WDG_SW
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - BFB2
    #[inline(always)]
    pub fn bfb2(&self) -> BFB2_R {
        BFB2_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - nBOOT1
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTR")
            .field("rdprot", &self.rdprot())
            .field("wprmod", &self.wprmod())
            .field("bor_lev", &self.bor_lev())
            .field("wdg_sw", &self.wdg_sw())
            .field("n_rst_stop", &self.n_rst_stop())
            .field("n_rst_stdby", &self.n_rst_stdby())
            .field("bfb2", &self.bfb2())
            .field("n_boot1", &self.n_boot1())
            .finish()
    }
}
/**Option byte register

You can [`read`](crate::Reg::read) this register and get [`optr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#FLASH:OPTR)*/
pub struct OPTRrs;
impl crate::RegisterSpec for OPTRrs {
    type Ux = u32;
}
///`read()` method returns [`optr::R`](R) reader structure
impl crate::Readable for OPTRrs {}
///`reset()` method sets OPTR to value 0x00f8_0000
impl crate::Resettable for OPTRrs {
    const RESET_VALUE: u32 = 0x00f8_0000;
}
