#[doc = "Register `OPTR` reader"]
pub type R = crate::R<OPTRrs>;
#[doc = "Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDPROT {
    #[doc = "0: Level 1"]
    Level1 = 0,
    #[doc = "170: Level 0"]
    Level0 = 170,
    #[doc = "204: Level 2"]
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
#[doc = "Field `RDPROT` reader - Read protection"]
pub type RDPROT_R = crate::FieldReader<RDPROT>;
impl RDPROT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RDPROT> {
        match self.bits {
            0 => Some(RDPROT::Level1),
            170 => Some(RDPROT::Level0),
            204 => Some(RDPROT::Level2),
            _ => None,
        }
    }
    #[doc = "Level 1"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == RDPROT::Level1
    }
    #[doc = "Level 0"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == RDPROT::Level0
    }
    #[doc = "Level 2"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RDPROT::Level2
    }
}
#[doc = "Selection of protection mode of WPR bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPRMOD {
    #[doc = "0: PCROP disabled. The WRPROT bits are used as a write protection on a sector."]
    Disabled = 0,
    #[doc = "1: PCROP enabled. The WRPROT bits are used as a read protection on a sector."]
    Enabled = 1,
}
impl From<WPRMOD> for bool {
    #[inline(always)]
    fn from(variant: WPRMOD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPRMOD` reader - Selection of protection mode of WPR bits"]
pub type WPRMOD_R = crate::BitReader<WPRMOD>;
impl WPRMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WPRMOD {
        match self.bits {
            false => WPRMOD::Disabled,
            true => WPRMOD::Enabled,
        }
    }
    #[doc = "PCROP disabled. The WRPROT bits are used as a write protection on a sector."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WPRMOD::Disabled
    }
    #[doc = "PCROP enabled. The WRPROT bits are used as a read protection on a sector."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WPRMOD::Enabled
    }
}
#[doc = "BOR_LEV\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOR_LEV {
    #[doc = "0: This is the reset threshold level for the 1.45 V - 1.55 V voltage range (power-down only)"]
    BorOff = 0,
    #[doc = "1: Reset threshold level for VBOR0 (around 1.8 V)"]
    BorLevel1 = 1,
    #[doc = "2: Reset threshold level for VBOR1 (around 2.0 V)"]
    BorLevel2 = 2,
    #[doc = "3: Reset threshold level for VBOR2 (around 2.5 V)"]
    BorLevel3 = 3,
    #[doc = "4: Reset threshold level for VBOR3 (around 2.7 V)"]
    BorLevel4 = 4,
    #[doc = "5: Reset threshold level for VBOR4 (around 3.0 V)"]
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
#[doc = "Field `BOR_LEV` reader - BOR_LEV"]
pub type BOR_LEV_R = crate::FieldReader<BOR_LEV>;
impl BOR_LEV_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "This is the reset threshold level for the 1.45 V - 1.55 V voltage range (power-down only)"]
    #[inline(always)]
    pub fn is_bor_off(&self) -> bool {
        *self == BOR_LEV::BorOff
    }
    #[doc = "Reset threshold level for VBOR0 (around 1.8 V)"]
    #[inline(always)]
    pub fn is_bor_level1(&self) -> bool {
        *self == BOR_LEV::BorLevel1
    }
    #[doc = "Reset threshold level for VBOR1 (around 2.0 V)"]
    #[inline(always)]
    pub fn is_bor_level2(&self) -> bool {
        *self == BOR_LEV::BorLevel2
    }
    #[doc = "Reset threshold level for VBOR2 (around 2.5 V)"]
    #[inline(always)]
    pub fn is_bor_level3(&self) -> bool {
        *self == BOR_LEV::BorLevel3
    }
    #[doc = "Reset threshold level for VBOR3 (around 2.7 V)"]
    #[inline(always)]
    pub fn is_bor_level4(&self) -> bool {
        *self == BOR_LEV::BorLevel4
    }
    #[doc = "Reset threshold level for VBOR4 (around 3.0 V)"]
    #[inline(always)]
    pub fn is_bor_level5(&self) -> bool {
        *self == BOR_LEV::BorLevel5
    }
}
impl R {
    #[doc = "Bits 0:7 - Read protection"]
    #[inline(always)]
    pub fn rdprot(&self) -> RDPROT_R {
        RDPROT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Selection of protection mode of WPR bits"]
    #[inline(always)]
    pub fn wprmod(&self) -> WPRMOD_R {
        WPRMOD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - BOR_LEV"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Option byte register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTRrs;
impl crate::RegisterSpec for OPTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optr::R`](R) reader structure"]
impl crate::Readable for OPTRrs {}
#[doc = "`reset()` method sets OPTR to value 0x00f8_0000"]
impl crate::Resettable for OPTRrs {
    const RESET_VALUE: u32 = 0x00f8_0000;
}
