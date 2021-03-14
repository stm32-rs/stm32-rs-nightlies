#[doc = "Reader of register CALFACT"]
pub type R = crate::R<u32, super::CALFACT>;
#[doc = "Writer for register CALFACT"]
pub type W = crate::W<u32, super::CALFACT>;
#[doc = "Register CALFACT `reset()`'s with value 0"]
impl crate::ResetValue for super::CALFACT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALFACT`"]
pub type CALFACT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALFACT`"]
pub struct CALFACT_W<'a> {
    w: &'a mut W,
}
impl<'a> CALFACT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - ADC calibration factor in single-ended mode"]
    #[inline(always)]
    pub fn calfact(&self) -> CALFACT_R {
        CALFACT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - ADC calibration factor in single-ended mode"]
    #[inline(always)]
    pub fn calfact(&mut self) -> CALFACT_W {
        CALFACT_W { w: self }
    }
}
