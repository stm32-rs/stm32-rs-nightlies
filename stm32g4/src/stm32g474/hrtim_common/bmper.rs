#[doc = "Reader of register BMPER"]
pub type R = crate::R<u32, super::BMPER>;
#[doc = "Writer for register BMPER"]
pub type W = crate::W<u32, super::BMPER>;
#[doc = "Register BMPER `reset()`'s with value 0"]
impl crate::ResetValue for super::BMPER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BMPER`"]
pub type BMPER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BMPER`"]
pub struct BMPER_W<'a> {
    w: &'a mut W,
}
impl<'a> BMPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Burst mode Period"]
    #[inline(always)]
    pub fn bmper(&self) -> BMPER_R {
        BMPER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Burst mode Period"]
    #[inline(always)]
    pub fn bmper(&mut self) -> BMPER_W {
        BMPER_W { w: self }
    }
}
