#[doc = "Reader of register DDRCTRL_PCCFG"]
pub type R = crate::R<u32, super::DDRCTRL_PCCFG>;
#[doc = "Writer for register DDRCTRL_PCCFG"]
pub type W = crate::W<u32, super::DDRCTRL_PCCFG>;
#[doc = "Register DDRCTRL_PCCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_PCCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GO2CRITICAL_EN`"]
pub type GO2CRITICAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GO2CRITICAL_EN`"]
pub struct GO2CRITICAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GO2CRITICAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `PAGEMATCH_LIMIT`"]
pub type PAGEMATCH_LIMIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAGEMATCH_LIMIT`"]
pub struct PAGEMATCH_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGEMATCH_LIMIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `BL_EXP_MODE`"]
pub type BL_EXP_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BL_EXP_MODE`"]
pub struct BL_EXP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_EXP_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GO2CRITICAL_EN"]
    #[inline(always)]
    pub fn go2critical_en(&self) -> GO2CRITICAL_EN_R {
        GO2CRITICAL_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - PAGEMATCH_LIMIT"]
    #[inline(always)]
    pub fn pagematch_limit(&self) -> PAGEMATCH_LIMIT_R {
        PAGEMATCH_LIMIT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BL_EXP_MODE"]
    #[inline(always)]
    pub fn bl_exp_mode(&self) -> BL_EXP_MODE_R {
        BL_EXP_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GO2CRITICAL_EN"]
    #[inline(always)]
    pub fn go2critical_en(&mut self) -> GO2CRITICAL_EN_W {
        GO2CRITICAL_EN_W { w: self }
    }
    #[doc = "Bit 4 - PAGEMATCH_LIMIT"]
    #[inline(always)]
    pub fn pagematch_limit(&mut self) -> PAGEMATCH_LIMIT_W {
        PAGEMATCH_LIMIT_W { w: self }
    }
    #[doc = "Bit 8 - BL_EXP_MODE"]
    #[inline(always)]
    pub fn bl_exp_mode(&mut self) -> BL_EXP_MODE_W {
        BL_EXP_MODE_W { w: self }
    }
}
