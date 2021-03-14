#[doc = "Reader of register DMACRxDTPR"]
pub type R = crate::R<u32, super::DMACRXDTPR>;
#[doc = "Writer for register DMACRxDTPR"]
pub type W = crate::W<u32, super::DMACRXDTPR>;
#[doc = "Register DMACRxDTPR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACRXDTPR {
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
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Receive Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn rdt(&self) -> RDT_R {
        RDT_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn rdt(&mut self) -> RDT_W {
        RDT_W { w: self }
    }
}
