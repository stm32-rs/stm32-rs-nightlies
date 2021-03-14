#[doc = "Reader of register RCC_PLL1CSGR"]
pub type R = crate::R<u32, super::RCC_PLL1CSGR>;
#[doc = "Writer for register RCC_PLL1CSGR"]
pub type W = crate::W<u32, super::RCC_PLL1CSGR>;
#[doc = "Register RCC_PLL1CSGR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_PLL1CSGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MOD_PER`"]
pub type MOD_PER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MOD_PER`"]
pub struct MOD_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
#[doc = "Reader of field `TPDFN_DIS`"]
pub type TPDFN_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPDFN_DIS`"]
pub struct TPDFN_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TPDFN_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RPDFN_DIS`"]
pub type RPDFN_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPDFN_DIS`"]
pub struct RPDFN_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RPDFN_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SSCG_MODE`"]
pub type SSCG_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSCG_MODE`"]
pub struct SSCG_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSCG_MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `INC_STEP`"]
pub type INC_STEP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INC_STEP`"]
pub struct INC_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | (((value as u32) & 0x7fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - MOD_PER"]
    #[inline(always)]
    pub fn mod_per(&self) -> MOD_PER_R {
        MOD_PER_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - TPDFN_DIS"]
    #[inline(always)]
    pub fn tpdfn_dis(&self) -> TPDFN_DIS_R {
        TPDFN_DIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RPDFN_DIS"]
    #[inline(always)]
    pub fn rpdfn_dis(&self) -> RPDFN_DIS_R {
        RPDFN_DIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SSCG_MODE"]
    #[inline(always)]
    pub fn sscg_mode(&self) -> SSCG_MODE_R {
        SSCG_MODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:30 - INC_STEP"]
    #[inline(always)]
    pub fn inc_step(&self) -> INC_STEP_R {
        INC_STEP_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - MOD_PER"]
    #[inline(always)]
    pub fn mod_per(&mut self) -> MOD_PER_W {
        MOD_PER_W { w: self }
    }
    #[doc = "Bit 13 - TPDFN_DIS"]
    #[inline(always)]
    pub fn tpdfn_dis(&mut self) -> TPDFN_DIS_W {
        TPDFN_DIS_W { w: self }
    }
    #[doc = "Bit 14 - RPDFN_DIS"]
    #[inline(always)]
    pub fn rpdfn_dis(&mut self) -> RPDFN_DIS_W {
        RPDFN_DIS_W { w: self }
    }
    #[doc = "Bit 15 - SSCG_MODE"]
    #[inline(always)]
    pub fn sscg_mode(&mut self) -> SSCG_MODE_W {
        SSCG_MODE_W { w: self }
    }
    #[doc = "Bits 16:30 - INC_STEP"]
    #[inline(always)]
    pub fn inc_step(&mut self) -> INC_STEP_W {
        INC_STEP_W { w: self }
    }
}
