#[doc = "Reader of register OPTR"]
pub type R = crate::R<u32, super::OPTR>;
#[doc = "Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RDPROT_A {
    #[doc = "170: Level 0"]
    LEVEL0 = 170,
    #[doc = "0: Level 1"]
    LEVEL1 = 0,
    #[doc = "204: Level 2"]
    LEVEL2 = 204,
}
impl From<RDPROT_A> for u8 {
    #[inline(always)]
    fn from(variant: RDPROT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RDPROT`"]
pub type RDPROT_R = crate::R<u8, RDPROT_A>;
impl RDPROT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RDPROT_A> {
        use crate::Variant::*;
        match self.bits {
            170 => Val(RDPROT_A::LEVEL0),
            0 => Val(RDPROT_A::LEVEL1),
            204 => Val(RDPROT_A::LEVEL2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == RDPROT_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == RDPROT_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RDPROT_A::LEVEL2
    }
}
#[doc = "BOR_LEV\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOR_LEV_A {
    #[doc = "0: This is the reset threshold level for the 1.45 V - 1.55 V voltage range (power-down only)"]
    BOR_OFF = 0,
    #[doc = "1: Reset threshold level for VBOR0 (around 1.8 V)"]
    BOR_LEVEL1 = 1,
    #[doc = "2: Reset threshold level for VBOR1 (around 2.0 V)"]
    BOR_LEVEL2 = 2,
    #[doc = "3: Reset threshold level for VBOR2 (around 2.5 V)"]
    BOR_LEVEL3 = 3,
    #[doc = "4: Reset threshold level for VBOR3 (around 2.7 V)"]
    BOR_LEVEL4 = 4,
    #[doc = "5: Reset threshold level for VBOR4 (around 3.0 V)"]
    BOR_LEVEL5 = 5,
}
impl From<BOR_LEV_A> for u8 {
    #[inline(always)]
    fn from(variant: BOR_LEV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BOR_LEV`"]
pub type BOR_LEV_R = crate::R<u8, BOR_LEV_A>;
impl BOR_LEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BOR_LEV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BOR_LEV_A::BOR_OFF),
            1 => Val(BOR_LEV_A::BOR_LEVEL1),
            2 => Val(BOR_LEV_A::BOR_LEVEL2),
            3 => Val(BOR_LEV_A::BOR_LEVEL3),
            4 => Val(BOR_LEV_A::BOR_LEVEL4),
            5 => Val(BOR_LEV_A::BOR_LEVEL5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BOR_OFF`"]
    #[inline(always)]
    pub fn is_bor_off(&self) -> bool {
        *self == BOR_LEV_A::BOR_OFF
    }
    #[doc = "Checks if the value of the field is `BOR_LEVEL1`"]
    #[inline(always)]
    pub fn is_bor_level1(&self) -> bool {
        *self == BOR_LEV_A::BOR_LEVEL1
    }
    #[doc = "Checks if the value of the field is `BOR_LEVEL2`"]
    #[inline(always)]
    pub fn is_bor_level2(&self) -> bool {
        *self == BOR_LEV_A::BOR_LEVEL2
    }
    #[doc = "Checks if the value of the field is `BOR_LEVEL3`"]
    #[inline(always)]
    pub fn is_bor_level3(&self) -> bool {
        *self == BOR_LEV_A::BOR_LEVEL3
    }
    #[doc = "Checks if the value of the field is `BOR_LEVEL4`"]
    #[inline(always)]
    pub fn is_bor_level4(&self) -> bool {
        *self == BOR_LEV_A::BOR_LEVEL4
    }
    #[doc = "Checks if the value of the field is `BOR_LEVEL5`"]
    #[inline(always)]
    pub fn is_bor_level5(&self) -> bool {
        *self == BOR_LEV_A::BOR_LEVEL5
    }
}
#[doc = "Selection of protection mode of WPR bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPRMOD_A {
    #[doc = "0: PCROP disabled. The WRPROT bits are used as a write protection on a sector."]
    DISABLED = 0,
    #[doc = "1: PCROP enabled. The WRPROT bits are used as a read protection on a sector."]
    ENABLED = 1,
}
impl From<WPRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: WPRMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WPRMOD`"]
pub type WPRMOD_R = crate::R<bool, WPRMOD_A>;
impl WPRMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPRMOD_A {
        match self.bits {
            false => WPRMOD_A::DISABLED,
            true => WPRMOD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WPRMOD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WPRMOD_A::ENABLED
    }
}
impl R {
    #[doc = "Bits 0:7 - Read protection"]
    #[inline(always)]
    pub fn rdprot(&self) -> RDPROT_R {
        RDPROT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - BOR_LEV"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Selection of protection mode of WPR bits"]
    #[inline(always)]
    pub fn wprmod(&self) -> WPRMOD_R {
        WPRMOD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
