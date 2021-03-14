#[doc = "Reader of register SDMMC_IDMABAR"]
pub type R = crate::R<u32, super::SDMMC_IDMABAR>;
#[doc = "Writer for register SDMMC_IDMABAR"]
pub type W = crate::W<u32, super::SDMMC_IDMABAR>;
#[doc = "Register SDMMC_IDMABAR `reset()`'s with value 0"]
impl crate::ResetValue for super::SDMMC_IDMABAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDMABA`"]
pub type IDMABA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IDMABA`"]
pub struct IDMABA_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMABA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - IDMABA"]
    #[inline(always)]
    pub fn idmaba(&self) -> IDMABA_R {
        IDMABA_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - IDMABA"]
    #[inline(always)]
    pub fn idmaba(&mut self) -> IDMABA_W {
        IDMABA_W { w: self }
    }
}
