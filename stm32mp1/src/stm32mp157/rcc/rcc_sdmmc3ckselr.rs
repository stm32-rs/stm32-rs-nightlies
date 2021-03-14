#[doc = "Reader of register RCC_SDMMC3CKSELR"]
pub type R = crate::R<u32, super::RCC_SDMMC3CKSELR>;
#[doc = "Writer for register RCC_SDMMC3CKSELR"]
pub type W = crate::W<u32, super::RCC_SDMMC3CKSELR>;
#[doc = "Register RCC_SDMMC3CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_SDMMC3CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDMMC3SRC`"]
pub type SDMMC3SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDMMC3SRC`"]
pub struct SDMMC3SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC3SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SDMMC3SRC"]
    #[inline(always)]
    pub fn sdmmc3src(&self) -> SDMMC3SRC_R {
        SDMMC3SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SDMMC3SRC"]
    #[inline(always)]
    pub fn sdmmc3src(&mut self) -> SDMMC3SRC_W {
        SDMMC3SRC_W { w: self }
    }
}
