#[doc = "Reader of register JOFR3"]
pub type R = crate::R<u32, super::JOFR3>;
#[doc = "Writer for register JOFR3"]
pub type W = crate::W<u32, super::JOFR3>;
#[doc = "Register JOFR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::JOFR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `JOFFSET3`"]
pub type JOFFSET3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `JOFFSET3`"]
pub struct JOFFSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> JOFFSET3_W<'a> {
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
    pub fn joffset3(&self) -> JOFFSET3_R {
        JOFFSET3_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset3(&mut self) -> JOFFSET3_W {
        JOFFSET3_W { w: self }
    }
}
