#[doc = "Reader of register AXIMC_M2_FN_MOD_AHB"]
pub type R = crate::R<u32, super::AXIMC_M2_FN_MOD_AHB>;
#[doc = "Writer for register AXIMC_M2_FN_MOD_AHB"]
pub type W = crate::W<u32, super::AXIMC_M2_FN_MOD_AHB>;
#[doc = "Register AXIMC_M2_FN_MOD_AHB `reset()`'s with value 0"]
impl crate::ResetValue for super::AXIMC_M2_FN_MOD_AHB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RD_INC_OVERRIDE`"]
pub type RD_INC_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_INC_OVERRIDE`"]
pub struct RD_INC_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_INC_OVERRIDE_W<'a> {
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
#[doc = "Reader of field `WR_INC_OVERRIDE`"]
pub type WR_INC_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WR_INC_OVERRIDE`"]
pub struct WR_INC_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_INC_OVERRIDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RD_INC_OVERRIDE"]
    #[inline(always)]
    pub fn rd_inc_override(&self) -> RD_INC_OVERRIDE_R {
        RD_INC_OVERRIDE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WR_INC_OVERRIDE"]
    #[inline(always)]
    pub fn wr_inc_override(&self) -> WR_INC_OVERRIDE_R {
        WR_INC_OVERRIDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RD_INC_OVERRIDE"]
    #[inline(always)]
    pub fn rd_inc_override(&mut self) -> RD_INC_OVERRIDE_W {
        RD_INC_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 1 - WR_INC_OVERRIDE"]
    #[inline(always)]
    pub fn wr_inc_override(&mut self) -> WR_INC_OVERRIDE_W {
        WR_INC_OVERRIDE_W { w: self }
    }
}
