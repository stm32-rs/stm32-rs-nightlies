#[doc = "Reader of register ADC_HTR2"]
pub type R = crate::R<u32, super::ADC_HTR2>;
#[doc = "Writer for register ADC_HTR2"]
pub type W = crate::W<u32, super::ADC_HTR2>;
#[doc = "Register ADC_HTR2 `reset()`'s with value 0x03ff_ffff"]
impl crate::ResetValue for super::ADC_HTR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03ff_ffff
    }
}
#[doc = "Reader of field `HTR2`"]
pub type HTR2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HTR2`"]
pub struct HTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> HTR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25 - HTR2"]
    #[inline(always)]
    pub fn htr2(&self) -> HTR2_R {
        HTR2_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25 - HTR2"]
    #[inline(always)]
    pub fn htr2(&mut self) -> HTR2_W {
        HTR2_W { w: self }
    }
}
