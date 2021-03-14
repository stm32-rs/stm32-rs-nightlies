#[doc = "Reader of register DDRCTRL_SWCTL"]
pub type R = crate::R<u32, super::DDRCTRL_SWCTL>;
#[doc = "Writer for register DDRCTRL_SWCTL"]
pub type W = crate::W<u32, super::DDRCTRL_SWCTL>;
#[doc = "Register DDRCTRL_SWCTL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::DDRCTRL_SWCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `SW_DONE`"]
pub type SW_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_DONE`"]
pub struct SW_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_DONE_W<'a> {
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
    #[doc = "Bit 0 - SW_DONE"]
    #[inline(always)]
    pub fn sw_done(&self) -> SW_DONE_R {
        SW_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW_DONE"]
    #[inline(always)]
    pub fn sw_done(&mut self) -> SW_DONE_W {
        SW_DONE_W { w: self }
    }
}
