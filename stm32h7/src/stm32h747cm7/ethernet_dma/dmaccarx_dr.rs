#[doc = "Reader of register DMACCARxDR"]
pub type R = crate::R<u32, super::DMACCARXDR>;
#[doc = "Writer for register DMACCARxDR"]
pub type W = crate::W<u32, super::DMACCARXDR>;
#[doc = "Register DMACCARxDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACCARXDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CURRDESAPTR`"]
pub type CURRDESAPTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CURRDESAPTR`"]
pub struct CURRDESAPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRDESAPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Application Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Application Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&mut self) -> CURRDESAPTR_W {
        CURRDESAPTR_W { w: self }
    }
}
