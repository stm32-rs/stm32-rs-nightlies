#[doc = "Reader of register DMACTxDTPR"]
pub type R = crate::R<u32, super::DMACTXDTPR>;
#[doc = "Writer for register DMACTxDTPR"]
pub type W = crate::W<u32, super::DMACTXDTPR>;
#[doc = "Register DMACTxDTPR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACTXDTPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TDT`"]
pub type TDT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TDT`"]
pub struct TDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Transmit Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn tdt(&self) -> TDT_R {
        TDT_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn tdt(&mut self) -> TDT_W {
        TDT_W { w: self }
    }
}
