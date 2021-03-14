#[doc = "Reader of register BDMADR"]
pub type R = crate::R<u32, super::BDMADR>;
#[doc = "Writer for register BDMADR"]
pub type W = crate::W<u32, super::BDMADR>;
#[doc = "Register BDMADR `reset()`'s with value 0"]
impl crate::ResetValue for super::BDMADR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BDMADR`"]
pub type BDMADR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BDMADR`"]
pub struct BDMADR_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Burst DMA Data register"]
    #[inline(always)]
    pub fn bdmadr(&self) -> BDMADR_R {
        BDMADR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Burst DMA Data register"]
    #[inline(always)]
    pub fn bdmadr(&mut self) -> BDMADR_W {
        BDMADR_W { w: self }
    }
}
