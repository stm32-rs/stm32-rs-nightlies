#[doc = "Reader of register ETH_DMAC0TxDLAR"]
pub type R = crate::R<u32, super::ETH_DMAC0TXDLAR>;
#[doc = "Writer for register ETH_DMAC0TxDLAR"]
pub type W = crate::W<u32, super::ETH_DMAC0TXDLAR>;
#[doc = "Register ETH_DMAC0TxDLAR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_DMAC0TXDLAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TDESLA`"]
pub type TDESLA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TDESLA`"]
pub struct TDESLA_W<'a> {
    w: &'a mut W,
}
impl<'a> TDESLA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - Start of Transmit List"]
    #[inline(always)]
    pub fn tdesla(&self) -> TDESLA_R {
        TDESLA_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:31 - Start of Transmit List"]
    #[inline(always)]
    pub fn tdesla(&mut self) -> TDESLA_W {
        TDESLA_W { w: self }
    }
}
