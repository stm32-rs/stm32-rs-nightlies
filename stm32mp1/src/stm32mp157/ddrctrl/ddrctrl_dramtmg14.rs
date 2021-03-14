#[doc = "Reader of register DDRCTRL_DRAMTMG14"]
pub type R = crate::R<u32, super::DDRCTRL_DRAMTMG14>;
#[doc = "Writer for register DDRCTRL_DRAMTMG14"]
pub type W = crate::W<u32, super::DDRCTRL_DRAMTMG14>;
#[doc = "Register DDRCTRL_DRAMTMG14 `reset()`'s with value 0xa0"]
impl crate::ResetValue for super::DDRCTRL_DRAMTMG14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xa0
    }
}
#[doc = "Reader of field `T_XSR`"]
pub type T_XSR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `T_XSR`"]
pub struct T_XSR_W<'a> {
    w: &'a mut W,
}
impl<'a> T_XSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - T_XSR"]
    #[inline(always)]
    pub fn t_xsr(&self) -> T_XSR_R {
        T_XSR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - T_XSR"]
    #[inline(always)]
    pub fn t_xsr(&mut self) -> T_XSR_W {
        T_XSR_W { w: self }
    }
}
