#[doc = "Reader of register ETH_MACHT1R"]
pub type R = crate::R<u32, super::ETH_MACHT1R>;
#[doc = "Writer for register ETH_MACHT1R"]
pub type W = crate::W<u32, super::ETH_MACHT1R>;
#[doc = "Register ETH_MACHT1R `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACHT1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HT63T32`"]
pub type HT63T32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HT63T32`"]
pub struct HT63T32_W<'a> {
    w: &'a mut W,
}
impl<'a> HT63T32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - HT63T32"]
    #[inline(always)]
    pub fn ht63t32(&self) -> HT63T32_R {
        HT63T32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - HT63T32"]
    #[inline(always)]
    pub fn ht63t32(&mut self) -> HT63T32_W {
        HT63T32_W { w: self }
    }
}
