#[doc = "Reader of register SHHR"]
pub type R = crate::R<u32, super::SHHR>;
#[doc = "Writer for register SHHR"]
pub type W = crate::W<u32, super::SHHR>;
#[doc = "Register SHHR `reset()`'s with value 0x0001_0001"]
impl crate::ResetValue for super::SHHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0001
    }
}
#[doc = "Reader of field `THOLD1`"]
pub type THOLD1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `THOLD1`"]
pub struct THOLD1_W<'a> {
    w: &'a mut W,
}
impl<'a> THOLD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - DAC Channel 1 hold Time (only valid in Sample and Hold mode)"]
    #[inline(always)]
    pub fn thold1(&self) -> THOLD1_R {
        THOLD1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC Channel 1 hold Time (only valid in Sample and Hold mode)"]
    #[inline(always)]
    pub fn thold1(&mut self) -> THOLD1_W {
        THOLD1_W { w: self }
    }
}
