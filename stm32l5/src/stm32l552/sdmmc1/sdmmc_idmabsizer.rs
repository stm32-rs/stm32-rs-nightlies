#[doc = "Register `SDMMC_IDMABSIZER` reader"]
pub struct R(crate::R<SDMMC_IDMABSIZER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_IDMABSIZER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_IDMABSIZER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_IDMABSIZER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_IDMABSIZER` writer"]
pub struct W(crate::W<SDMMC_IDMABSIZER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_IDMABSIZER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SDMMC_IDMABSIZER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_IDMABSIZER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDMABNDT` reader - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub struct IDMABNDT_R(crate::FieldReader<u8, u8>);
impl IDMABNDT_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDMABNDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDMABNDT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDMABNDT` writer - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub struct IDMABNDT_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMABNDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 5)) | ((value as u32 & 0xff) << 5);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_idmabsizer](index.html) module"]
pub struct SDMMC_IDMABSIZER_SPEC;
impl crate::RegisterSpec for SDMMC_IDMABSIZER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_idmabsizer::R](R) reader structure"]
impl crate::Readable for SDMMC_IDMABSIZER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_idmabsizer::W](W) writer structure"]
impl crate::Writable for SDMMC_IDMABSIZER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMMC_IDMABSIZER to value 0"]
impl crate::Resettable for SDMMC_IDMABSIZER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
