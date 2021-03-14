#[doc = "Reader of register RCC_SDMMC12CKSELR"]
pub type R = crate::R<u32, super::RCC_SDMMC12CKSELR>;
#[doc = "Writer for register RCC_SDMMC12CKSELR"]
pub type W = crate::W<u32, super::RCC_SDMMC12CKSELR>;
#[doc = "Register RCC_SDMMC12CKSELR `reset()`'s with value 0x03"]
impl crate::ResetValue for super::RCC_SDMMC12CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `SDMMC12SRC`"]
pub type SDMMC12SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDMMC12SRC`"]
pub struct SDMMC12SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC12SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SDMMC12SRC"]
    #[inline(always)]
    pub fn sdmmc12src(&self) -> SDMMC12SRC_R {
        SDMMC12SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SDMMC12SRC"]
    #[inline(always)]
    pub fn sdmmc12src(&mut self) -> SDMMC12SRC_W {
        SDMMC12SRC_W { w: self }
    }
}
