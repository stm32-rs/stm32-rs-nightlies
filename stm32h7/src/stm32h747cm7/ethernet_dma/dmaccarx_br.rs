#[doc = "Reader of register DMACCARxBR"]
pub type R = crate::R<u32, super::DMACCARXBR>;
#[doc = "Writer for register DMACCARxBR"]
pub type W = crate::W<u32, super::DMACCARXBR>;
#[doc = "Register DMACCARxBR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACCARXBR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CURRBUFAPTR`"]
pub type CURRBUFAPTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CURRBUFAPTR`"]
pub struct CURRBUFAPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRBUFAPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Application Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn currbufaptr(&self) -> CURRBUFAPTR_R {
        CURRBUFAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Application Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn currbufaptr(&mut self) -> CURRBUFAPTR_W {
        CURRBUFAPTR_W { w: self }
    }
}
