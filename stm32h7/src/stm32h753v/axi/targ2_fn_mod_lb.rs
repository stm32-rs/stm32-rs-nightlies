#[doc = "Reader of register TARG2_FN_MOD_LB"]
pub type R = crate::R<u32, super::TARG2_FN_MOD_LB>;
#[doc = "Writer for register TARG2_FN_MOD_LB"]
pub type W = crate::W<u32, super::TARG2_FN_MOD_LB>;
#[doc = "Register TARG2_FN_MOD_LB `reset()`'s with value 0x04"]
impl crate::ResetValue for super::TARG2_FN_MOD_LB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `FN_MOD_LB`"]
pub type FN_MOD_LB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FN_MOD_LB`"]
pub struct FN_MOD_LB_W<'a> {
    w: &'a mut W,
}
impl<'a> FN_MOD_LB_W<'a> {
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
    #[doc = "Bit 0 - Controls burst breaking of long bursts"]
    #[inline(always)]
    pub fn fn_mod_lb(&self) -> FN_MOD_LB_R {
        FN_MOD_LB_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls burst breaking of long bursts"]
    #[inline(always)]
    pub fn fn_mod_lb(&mut self) -> FN_MOD_LB_W {
        FN_MOD_LB_W { w: self }
    }
}
