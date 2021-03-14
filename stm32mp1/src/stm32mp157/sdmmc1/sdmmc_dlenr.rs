#[doc = "Reader of register SDMMC_DLENR"]
pub type R = crate::R<u32, super::SDMMC_DLENR>;
#[doc = "Writer for register SDMMC_DLENR"]
pub type W = crate::W<u32, super::SDMMC_DLENR>;
#[doc = "Register SDMMC_DLENR `reset()`'s with value 0"]
impl crate::ResetValue for super::SDMMC_DLENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATALENGTH`"]
pub type DATALENGTH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DATALENGTH`"]
pub struct DATALENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATALENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24 - DATALENGTH"]
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24 - DATALENGTH"]
    #[inline(always)]
    pub fn datalength(&mut self) -> DATALENGTH_W {
        DATALENGTH_W { w: self }
    }
}
