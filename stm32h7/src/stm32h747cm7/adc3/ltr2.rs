#[doc = "Reader of register LTR2"]
pub type R = crate::R<u32, super::LTR2>;
#[doc = "Writer for register LTR2"]
pub type W = crate::W<u32, super::LTR2>;
#[doc = "Register LTR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::LTR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LTR2`"]
pub type LTR2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LTR2`"]
pub struct LTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> LTR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25 - Analog watchdog 2 lower threshold"]
    #[inline(always)]
    pub fn ltr2(&self) -> LTR2_R {
        LTR2_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25 - Analog watchdog 2 lower threshold"]
    #[inline(always)]
    pub fn ltr2(&mut self) -> LTR2_W {
        LTR2_W { w: self }
    }
}
