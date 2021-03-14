#[doc = "Reader of register TARG1_FN_MOD2"]
pub type R = crate::R<u32, super::TARG1_FN_MOD2>;
#[doc = "Writer for register TARG1_FN_MOD2"]
pub type W = crate::W<u32, super::TARG1_FN_MOD2>;
#[doc = "Register TARG1_FN_MOD2 `reset()`'s with value 0x04"]
impl crate::ResetValue for super::TARG1_FN_MOD2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `BYPASS_MERGE`"]
pub type BYPASS_MERGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS_MERGE`"]
pub struct BYPASS_MERGE_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_MERGE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Disable packing of beats to match the output data width"]
    #[inline(always)]
    pub fn bypass_merge(&self) -> BYPASS_MERGE_R {
        BYPASS_MERGE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable packing of beats to match the output data width"]
    #[inline(always)]
    pub fn bypass_merge(&mut self) -> BYPASS_MERGE_W {
        BYPASS_MERGE_W { w: self }
    }
}
