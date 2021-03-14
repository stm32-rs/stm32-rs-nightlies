#[doc = "Reader of register ADC_LTR1"]
pub type R = crate::R<u32, super::ADC_LTR1>;
#[doc = "Writer for register ADC_LTR1"]
pub type W = crate::W<u32, super::ADC_LTR1>;
#[doc = "Register ADC_LTR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_LTR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LTR1`"]
pub type LTR1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LTR1`"]
pub struct LTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> LTR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25 - LTR1"]
    #[inline(always)]
    pub fn ltr1(&self) -> LTR1_R {
        LTR1_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25 - LTR1"]
    #[inline(always)]
    pub fn ltr1(&mut self) -> LTR1_W {
        LTR1_W { w: self }
    }
}
