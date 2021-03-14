#[doc = "Reader of register MCNTR"]
pub type R = crate::R<u32, super::MCNTR>;
#[doc = "Writer for register MCNTR"]
pub type W = crate::W<u32, super::MCNTR>;
#[doc = "Register MCNTR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCNTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCNT`"]
pub type MCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MCNT`"]
pub struct MCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    pub fn mcnt(&mut self) -> MCNT_W {
        MCNT_W { w: self }
    }
}
