#[doc = "Reader of register ETH_DMAC0RxDTPR"]
pub type R = crate::R<u32, super::ETH_DMAC0RXDTPR>;
#[doc = "Writer for register ETH_DMAC0RxDTPR"]
pub type W = crate::W<u32, super::ETH_DMAC0RXDTPR>;
#[doc = "Register ETH_DMAC0RxDTPR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_DMAC0RXDTPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDT`"]
pub type RDT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RDT`"]
pub struct RDT_W<'a> {
    w: &'a mut W,
}
impl<'a> RDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - Receive Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn rdt(&self) -> RDT_R {
        RDT_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:31 - Receive Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn rdt(&mut self) -> RDT_W {
        RDT_W { w: self }
    }
}
