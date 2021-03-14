#[doc = "Reader of register JOFR2"]
pub type R = crate::R<u32, super::JOFR2>;
#[doc = "Writer for register JOFR2"]
pub type W = crate::W<u32, super::JOFR2>;
#[doc = "Register JOFR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::JOFR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `JOFFSET2`"]
pub type JOFFSET2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `JOFFSET2`"]
pub struct JOFFSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> JOFFSET2_W<'a> {
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
    pub fn joffset2(&self) -> JOFFSET2_R {
        JOFFSET2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset2(&mut self) -> JOFFSET2_W {
        JOFFSET2_W { w: self }
    }
}
