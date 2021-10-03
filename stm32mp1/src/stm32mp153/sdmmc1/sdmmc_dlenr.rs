#[doc = "Register `SDMMC_DLENR` reader"]
pub struct R(crate::R<SDMMC_DLENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_DLENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_DLENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_DLENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_DLENR` writer"]
pub struct W(crate::W<SDMMC_DLENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_DLENR_SPEC>;
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
impl From<crate::W<SDMMC_DLENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_DLENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATALENGTH` reader - DATALENGTH"]
pub struct DATALENGTH_R(crate::FieldReader<u32, u32>);
impl DATALENGTH_R {
    pub(crate) fn new(bits: u32) -> Self {
        DATALENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATALENGTH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATALENGTH` writer - DATALENGTH"]
pub struct DATALENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATALENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | (value as u32 & 0x01ff_ffff);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_dlenr](index.html) module"]
pub struct SDMMC_DLENR_SPEC;
impl crate::RegisterSpec for SDMMC_DLENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_dlenr::R](R) reader structure"]
impl crate::Readable for SDMMC_DLENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_dlenr::W](W) writer structure"]
impl crate::Writable for SDMMC_DLENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMMC_DLENR to value 0"]
impl crate::Resettable for SDMMC_DLENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
