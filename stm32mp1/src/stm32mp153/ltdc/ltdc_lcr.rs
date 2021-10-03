#[doc = "Register `LTDC_LCR` reader"]
pub struct R(crate::R<LTDC_LCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_LCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_LCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_LCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LNBR` reader - LNBR"]
pub struct LNBR_R(crate::FieldReader<u8, u8>);
impl LNBR_R {
    pub(crate) fn new(bits: u8) -> Self {
        LNBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNBR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - LNBR"]
    #[inline(always)]
    pub fn lnbr(&self) -> LNBR_R {
        LNBR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "LDTC layer count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_lcr](index.html) module"]
pub struct LTDC_LCR_SPEC;
impl crate::RegisterSpec for LTDC_LCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_lcr::R](R) reader structure"]
impl crate::Readable for LTDC_LCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTDC_LCR to value 0x02"]
impl crate::Resettable for LTDC_LCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
