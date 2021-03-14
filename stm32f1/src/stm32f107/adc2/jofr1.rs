#[doc = "Reader of register JOFR1"]
pub type R = crate::R<u32, super::JOFR1>;
#[doc = "Writer for register JOFR1"]
pub type W = crate::W<u32, super::JOFR1>;
#[doc = "Register JOFR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::JOFR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `JOFFSET1`"]
pub type JOFFSET1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `JOFFSET1`"]
pub struct JOFFSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> JOFFSET1_W<'a> {
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
    pub fn joffset1(&self) -> JOFFSET1_R {
        JOFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset1(&mut self) -> JOFFSET1_W {
        JOFFSET1_W { w: self }
    }
}
