#[doc = "Reader of register MPER"]
pub type R = crate::R<u32, super::MPER>;
#[doc = "Writer for register MPER"]
pub type W = crate::W<u32, super::MPER>;
#[doc = "Register MPER `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::MPER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `MPER`"]
pub type MPER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MPER`"]
pub struct MPER_W<'a> {
    w: &'a mut W,
}
impl<'a> MPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Master Timer Period value"]
    #[inline(always)]
    pub fn mper(&self) -> MPER_R {
        MPER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Period value"]
    #[inline(always)]
    pub fn mper(&mut self) -> MPER_W {
        MPER_W { w: self }
    }
}
