#[doc = "Reader of register DMACRxRLR"]
pub type R = crate::R<u32, super::DMACRXRLR>;
#[doc = "Writer for register DMACRxRLR"]
pub type W = crate::W<u32, super::DMACRXRLR>;
#[doc = "Register DMACRxRLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACRXRLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDRL`"]
pub type RDRL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RDRL`"]
pub struct RDRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RDRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Receive Descriptor Ring Length"]
    #[inline(always)]
    pub fn rdrl(&self) -> RDRL_R {
        RDRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Receive Descriptor Ring Length"]
    #[inline(always)]
    pub fn rdrl(&mut self) -> RDRL_W {
        RDRL_W { w: self }
    }
}
