#[doc = "Reader of register DINR"]
pub type R = crate::R<u32, super::DINR>;
#[doc = "Writer for register DINR"]
pub type W = crate::W<u32, super::DINR>;
#[doc = "Register DINR `reset()`'s with value 0"]
impl crate::ResetValue for super::DINR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DINR`"]
pub type DINR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DINR`"]
pub struct DINR_W<'a> {
    w: &'a mut W,
}
impl<'a> DINR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data input"]
    #[inline(always)]
    pub fn dinr(&self) -> DINR_R {
        DINR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data input"]
    #[inline(always)]
    pub fn dinr(&mut self) -> DINR_W {
        DINR_W { w: self }
    }
}
