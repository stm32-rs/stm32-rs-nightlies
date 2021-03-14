#[doc = "Reader of register FDCAN_TXBCIE"]
pub type R = crate::R<u32, super::FDCAN_TXBCIE>;
#[doc = "Writer for register FDCAN_TXBCIE"]
pub type W = crate::W<u32, super::FDCAN_TXBCIE>;
#[doc = "Register FDCAN_TXBCIE `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TXBCIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFIE`"]
pub type CFIE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CFIE`"]
pub struct CFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CFIE"]
    #[inline(always)]
    pub fn cfie(&self) -> CFIE_R {
        CFIE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CFIE"]
    #[inline(always)]
    pub fn cfie(&mut self) -> CFIE_W {
        CFIE_W { w: self }
    }
}
