#[doc = "Register `SDMMC_RESP4R` reader"]
pub struct R(crate::R<SDMMC_RESP4R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_RESP4R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_RESP4R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_RESP4R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CARDSTATUS4` reader - CARDSTATUS4"]
pub struct CARDSTATUS4_R(crate::FieldReader<u32, u32>);
impl CARDSTATUS4_R {
    pub(crate) fn new(bits: u32) -> Self {
        CARDSTATUS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARDSTATUS4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS4"]
    #[inline(always)]
    pub fn cardstatus4(&self) -> CARDSTATUS4_R {
        CARDSTATUS4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_resp4r](index.html) module"]
pub struct SDMMC_RESP4R_SPEC;
impl crate::RegisterSpec for SDMMC_RESP4R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_resp4r::R](R) reader structure"]
impl crate::Readable for SDMMC_RESP4R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDMMC_RESP4R to value 0"]
impl crate::Resettable for SDMMC_RESP4R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
