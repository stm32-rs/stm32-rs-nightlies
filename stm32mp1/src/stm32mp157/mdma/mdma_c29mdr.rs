#[doc = "Reader of register MDMA_C29MDR"]
pub type R = crate::R<u32, super::MDMA_C29MDR>;
#[doc = "Writer for register MDMA_C29MDR"]
pub type W = crate::W<u32, super::MDMA_C29MDR>;
#[doc = "Register MDMA_C29MDR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C29MDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MDR`"]
pub type MDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MDR`"]
pub struct MDR_W<'a> {
    w: &'a mut W,
}
impl<'a> MDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MDR"]
    #[inline(always)]
    pub fn mdr(&self) -> MDR_R {
        MDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MDR"]
    #[inline(always)]
    pub fn mdr(&mut self) -> MDR_W {
        MDR_W { w: self }
    }
}
