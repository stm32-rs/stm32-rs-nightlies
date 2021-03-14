#[doc = "Reader of register ADC_DIFSEL"]
pub type R = crate::R<u32, super::ADC_DIFSEL>;
#[doc = "Writer for register ADC_DIFSEL"]
pub type W = crate::W<u32, super::ADC_DIFSEL>;
#[doc = "Register ADC_DIFSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_DIFSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIFSEL`"]
pub type DIFSEL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DIFSEL`"]
pub struct DIFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - DIFSEL"]
    #[inline(always)]
    pub fn difsel(&self) -> DIFSEL_R {
        DIFSEL_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - DIFSEL"]
    #[inline(always)]
    pub fn difsel(&mut self) -> DIFSEL_W {
        DIFSEL_W { w: self }
    }
}
