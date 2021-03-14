#[doc = "Reader of register DDRCTRL_ZQCTL2"]
pub type R = crate::R<u32, super::DDRCTRL_ZQCTL2>;
#[doc = "Writer for register DDRCTRL_ZQCTL2"]
pub type W = crate::W<u32, super::DDRCTRL_ZQCTL2>;
#[doc = "Register DDRCTRL_ZQCTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_ZQCTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ZQ_RESET`"]
pub type ZQ_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ZQ_RESET`"]
pub struct ZQ_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> ZQ_RESET_W<'a> {
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
    #[doc = "Bit 0 - ZQ_RESET"]
    #[inline(always)]
    pub fn zq_reset(&self) -> ZQ_RESET_R {
        ZQ_RESET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ZQ_RESET"]
    #[inline(always)]
    pub fn zq_reset(&mut self) -> ZQ_RESET_W {
        ZQ_RESET_W { w: self }
    }
}
