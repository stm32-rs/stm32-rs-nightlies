#[doc = "Reader of register JOFR4"]
pub type R = crate::R<u32, super::JOFR4>;
#[doc = "Writer for register JOFR4"]
pub type W = crate::W<u32, super::JOFR4>;
#[doc = "Register JOFR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::JOFR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `JOFFSET4`"]
pub type JOFFSET4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `JOFFSET4`"]
pub struct JOFFSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> JOFFSET4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset4(&self) -> JOFFSET4_R {
        JOFFSET4_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset4(&mut self) -> JOFFSET4_W {
        JOFFSET4_W { w: self }
    }
}
