#[doc = "Reader of register HTR3"]
pub type R = crate::R<u32, super::HTR3>;
#[doc = "Writer for register HTR3"]
pub type W = crate::W<u32, super::HTR3>;
#[doc = "Register HTR3 `reset()`'s with value 0x03ff_ffff"]
impl crate::ResetValue for super::HTR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03ff_ffff
    }
}
#[doc = "Reader of field `HTR3`"]
pub type HTR3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HTR3`"]
pub struct HTR3_W<'a> {
    w: &'a mut W,
}
impl<'a> HTR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25 - Analog watchdog 3 higher threshold"]
    #[inline(always)]
    pub fn htr3(&self) -> HTR3_R {
        HTR3_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25 - Analog watchdog 3 higher threshold"]
    #[inline(always)]
    pub fn htr3(&mut self) -> HTR3_W {
        HTR3_W { w: self }
    }
}
