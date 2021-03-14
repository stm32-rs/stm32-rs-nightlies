#[doc = "Reader of register ETH_DMAC0TxRLR"]
pub type R = crate::R<u32, super::ETH_DMAC0TXRLR>;
#[doc = "Writer for register ETH_DMAC0TxRLR"]
pub type W = crate::W<u32, super::ETH_DMAC0TXRLR>;
#[doc = "Register ETH_DMAC0TxRLR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_DMAC0TXRLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TDRL`"]
pub type TDRL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TDRL`"]
pub struct TDRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TDRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Transmit Descriptor Ring Length"]
    #[inline(always)]
    pub fn tdrl(&self) -> TDRL_R {
        TDRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transmit Descriptor Ring Length"]
    #[inline(always)]
    pub fn tdrl(&mut self) -> TDRL_W {
        TDRL_W { w: self }
    }
}
