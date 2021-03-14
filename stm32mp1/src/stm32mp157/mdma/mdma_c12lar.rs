#[doc = "Reader of register MDMA_C12LAR"]
pub type R = crate::R<u32, super::MDMA_C12LAR>;
#[doc = "Writer for register MDMA_C12LAR"]
pub type W = crate::W<u32, super::MDMA_C12LAR>;
#[doc = "Register MDMA_C12LAR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C12LAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LAR`"]
pub type LAR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LAR`"]
pub struct LAR_W<'a> {
    w: &'a mut W,
}
impl<'a> LAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - LAR"]
    #[inline(always)]
    pub fn lar(&self) -> LAR_R {
        LAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - LAR"]
    #[inline(always)]
    pub fn lar(&mut self) -> LAR_W {
        LAR_W { w: self }
    }
}
