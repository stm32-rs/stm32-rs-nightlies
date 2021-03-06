#[doc = "Reader of register LTR3"]
pub type R = crate::R<u32, super::LTR3>;
#[doc = "Writer for register LTR3"]
pub type W = crate::W<u32, super::LTR3>;
#[doc = "Register LTR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::LTR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LTR3`"]
pub type LTR3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LTR3`"]
pub struct LTR3_W<'a> {
    w: &'a mut W,
}
impl<'a> LTR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25 - Analog watchdog 3 lower threshold"]
    #[inline(always)]
    pub fn ltr3(&self) -> LTR3_R {
        LTR3_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25 - Analog watchdog 3 lower threshold"]
    #[inline(always)]
    pub fn ltr3(&mut self) -> LTR3_W {
        LTR3_W { w: self }
    }
}
