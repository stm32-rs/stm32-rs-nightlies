#[doc = "Reader of register LCCR"]
pub type R = crate::R<u32, super::LCCR>;
#[doc = "Writer for register LCCR"]
pub type W = crate::W<u32, super::LCCR>;
#[doc = "Register LCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::LCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMDSIZE`"]
pub type CMDSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMDSIZE`"]
pub struct CMDSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Command size"]
    #[inline(always)]
    pub fn cmdsize(&self) -> CMDSIZE_R {
        CMDSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Command size"]
    #[inline(always)]
    pub fn cmdsize(&mut self) -> CMDSIZE_W {
        CMDSIZE_W { w: self }
    }
}
