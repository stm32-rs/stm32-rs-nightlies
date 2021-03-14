#[doc = "Reader of register ETH_DMAC0RxDLAR"]
pub type R = crate::R<u32, super::ETH_DMAC0RXDLAR>;
#[doc = "Writer for register ETH_DMAC0RxDLAR"]
pub type W = crate::W<u32, super::ETH_DMAC0RXDLAR>;
#[doc = "Register ETH_DMAC0RxDLAR `reset()`'s with value 0x8000"]
impl crate::ResetValue for super::ETH_DMAC0RXDLAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
#[doc = "Reader of field `RDESLA`"]
pub type RDESLA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RDESLA`"]
pub struct RDESLA_W<'a> {
    w: &'a mut W,
}
impl<'a> RDESLA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla(&self) -> RDESLA_R {
        RDESLA_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla(&mut self) -> RDESLA_W {
        RDESLA_W { w: self }
    }
}
