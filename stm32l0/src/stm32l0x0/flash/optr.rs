#[doc = "Register `OPTR` reader"]
pub struct R(crate::R<OPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTR_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `RDPROT` reader - Read protection"]
pub struct RDPROT_R(crate::FieldReader<u8, RDPROT_A>);
impl RDPROT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDPROT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RDPROT_A> {
        match self.bits {
            170 => Some(RDPROT_A::LEVEL0),
            0 => Some(RDPROT_A::LEVEL1),
            204 => Some(RDPROT_A::LEVEL2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        **self == RDPROT_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        **self == RDPROT_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        **self == RDPROT_A::LEVEL2
    }
}
impl core::ops::Deref for RDPROT_R {
    type Target = crate::FieldReader<u8, RDPROT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WPRMOD\n\nValue on reset: 0"]
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
#[doc = "Field `WPRMOD` reader - WPRMOD"]
pub struct WPRMOD_R(crate::FieldReader<bool, WPRMOD_A>);
impl WPRMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPRMOD_R(crate::FieldReader::new(bits))
    }
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
        **self == WPRMOD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WPRMOD_A::ENABLED
    }
}
impl core::ops::Deref for WPRMOD_R {
    type Target = crate::FieldReader<bool, WPRMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `BOR_LEV` reader - BOR_LEV"]
pub struct BOR_LEV_R(crate::FieldReader<u8, BOR_LEV_A>);
impl BOR_LEV_R {
    pub(crate) fn new(bits: u8) -> Self {
        BOR_LEV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BOR_LEV_A> {
        match self.bits {
            0 => Some(BOR_LEV_A::BOR_OFF),
            1 => Some(BOR_LEV_A::BOR_LEVEL1),
            2 => Some(BOR_LEV_A::BOR_LEVEL2),
            3 => Some(BOR_LEV_A::BOR_LEVEL3),
            4 => Some(BOR_LEV_A::BOR_LEVEL4),
            5 => Some(BOR_LEV_A::BOR_LEVEL5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BOR_OFF`"]
    #[inline(always)]
    pub fn is_bor_off(&self) -> bool {
        **self == BOR_LEV_A::BOR_OFF
    }
    #[doc = "Checks if the value of the field is `BOR_LEVEL1`"]
    #[inline(always)]
    pub fn is_bor_level1(&self) -> bool {
        **self == BOR_LEV_A::BOR_LEVEL1
    }
    #[doc = "Checks if the value of the field is `BOR_LEVEL2`"]
    #[inline(always)]
    pub fn is_bor_level2(&self) -> bool {
        **self == BOR_LEV_A::BOR_LEVEL2
    }
    #[doc = "Checks if the value of the field is `BOR_LEVEL3`"]
    #[inline(always)]
    pub fn is_bor_level3(&self) -> bool {
        **self == BOR_LEV_A::BOR_LEVEL3
    }
    #[doc = "Checks if the value of the field is `BOR_LEVEL4`"]
    #[inline(always)]
    pub fn is_bor_level4(&self) -> bool {
        **self == BOR_LEV_A::BOR_LEVEL4
    }
    #[doc = "Checks if the value of the field is `BOR_LEVEL5`"]
    #[inline(always)]
    pub fn is_bor_level5(&self) -> bool {
        **self == BOR_LEV_A::BOR_LEVEL5
    }
}
impl core::ops::Deref for BOR_LEV_R {
    type Target = crate::FieldReader<u8, BOR_LEV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDG_SW` reader - WDG_SW"]
pub struct WDG_SW_R(crate::FieldReader<bool, bool>);
impl WDG_SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDG_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDG_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub struct NRST_STOP_R(crate::FieldReader<bool, bool>);
impl NRST_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRST_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRST_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub struct NRST_STDBY_R(crate::FieldReader<bool, bool>);
impl NRST_STDBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRST_STDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRST_STDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFB2` reader - BFB2"]
pub struct BFB2_R(crate::FieldReader<bool, bool>);
impl BFB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BFB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nBOOT1` reader - nBOOT1"]
pub struct NBOOT1_R(crate::FieldReader<bool, bool>);
impl NBOOT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        NBOOT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBOOT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Read protection"]
    #[inline(always)]
    pub fn rdprot(&self) -> RDPROT_R {
        RDPROT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - WPRMOD"]
    #[inline(always)]
    pub fn wprmod(&self) -> WPRMOD_R {
        WPRMOD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - BOR_LEV"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - WDG_SW"]
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - BFB2"]
    #[inline(always)]
    pub fn bfb2(&self) -> BFB2_R {
        BFB2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 31 - nBOOT1"]
    #[inline(always)]
    pub fn n_boot1(&self) -> NBOOT1_R {
        NBOOT1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Option byte register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optr](index.html) module"]
pub struct OPTR_SPEC;
impl crate::RegisterSpec for OPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optr::R](R) reader structure"]
impl crate::Readable for OPTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OPTR to value 0x00f8_0000"]
impl crate::Resettable for OPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00f8_0000
    }
}
