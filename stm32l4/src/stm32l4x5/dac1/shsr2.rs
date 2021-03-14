#[doc = "Reader of register SHSR2"]
pub type R = crate::R<u32, super::SHSR2>;
#[doc = "Writer for register SHSR2"]
pub type W = crate::W<u32, super::SHSR2>;
#[doc = "Register SHSR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SHSR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSAMPLE2`"]
pub type TSAMPLE2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSAMPLE2`"]
pub struct TSAMPLE2_W<'a> {
    w: &'a mut W,
}
impl<'a> TSAMPLE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - DAC Channel 2 sample Time"]
    #[inline(always)]
    pub fn tsample2(&self) -> TSAMPLE2_R {
        TSAMPLE2_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC Channel 2 sample Time"]
    #[inline(always)]
    pub fn tsample2(&mut self) -> TSAMPLE2_W {
        TSAMPLE2_W { w: self }
    }
}
