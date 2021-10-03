#[doc = "Register `SDMMC_IDMABAR` reader"]
pub struct R(crate::R<SDMMC_IDMABAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_IDMABAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_IDMABAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_IDMABAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_IDMABAR` writer"]
pub struct W(crate::W<SDMMC_IDMABAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_IDMABAR_SPEC>;
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
impl From<crate::W<SDMMC_IDMABAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_IDMABAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDMABA` reader - IDMABA"]
pub struct IDMABA_R(crate::FieldReader<u32, u32>);
impl IDMABA_R {
    pub(crate) fn new(bits: u32) -> Self {
        IDMABA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDMABA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDMABA` writer - IDMABA"]
pub struct IDMABA_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMABA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDMMC IDMA linked list memory base register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_idmabar](index.html) module"]
pub struct SDMMC_IDMABAR_SPEC;
impl crate::RegisterSpec for SDMMC_IDMABAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_idmabar::R](R) reader structure"]
impl crate::Readable for SDMMC_IDMABAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_idmabar::W](W) writer structure"]
impl crate::Writable for SDMMC_IDMABAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMMC_IDMABAR to value 0"]
impl crate::Resettable for SDMMC_IDMABAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
