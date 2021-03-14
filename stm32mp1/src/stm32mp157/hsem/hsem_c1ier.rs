#[doc = "Reader of register HSEM_C1IER"]
pub type R = crate::R<u32, super::HSEM_C1IER>;
#[doc = "Writer for register HSEM_C1IER"]
pub type W = crate::W<u32, super::HSEM_C1IER>;
#[doc = "Register HSEM_C1IER `reset()`'s with value 0"]
impl crate::ResetValue for super::HSEM_C1IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISE`"]
pub type ISE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISE`"]
pub struct ISE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISE"]
    #[inline(always)]
    pub fn ise(&self) -> ISE_R {
        ISE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISE"]
    #[inline(always)]
    pub fn ise(&mut self) -> ISE_W {
        ISE_W { w: self }
    }
}
