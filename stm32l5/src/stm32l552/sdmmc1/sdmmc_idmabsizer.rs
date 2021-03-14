#[doc = "Reader of register SDMMC_IDMABSIZER"]
pub type R = crate::R<u32, super::SDMMC_IDMABSIZER>;
#[doc = "Writer for register SDMMC_IDMABSIZER"]
pub type W = crate::W<u32, super::SDMMC_IDMABSIZER>;
#[doc = "Register SDMMC_IDMABSIZER `reset()`'s with value 0"]
impl crate::ResetValue for super::SDMMC_IDMABSIZER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDMABNDT`"]
pub type IDMABNDT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDMABNDT`"]
pub struct IDMABNDT_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMABNDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 5)) | (((value as u32) & 0xff) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:12 - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn idmabndt(&self) -> IDMABNDT_R {
        IDMABNDT_R::new(((self.bits >> 5) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 5:12 - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn idmabndt(&mut self) -> IDMABNDT_W {
        IDMABNDT_W { w: self }
    }
}
