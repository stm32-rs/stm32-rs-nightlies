#[doc = "Register `PTPPPSCR` reader"]
pub struct R(crate::R<PTPPPSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPPPSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPPPSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPPPSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PPSFREQ` reader - PPS frequency selection"]
pub struct PPSFREQ_R(crate::FieldReader<u8, u8>);
impl PPSFREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        PPSFREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPSFREQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - PPS frequency selection"]
    #[inline(always)]
    pub fn ppsfreq(&self) -> PPSFREQ_R {
        PPSFREQ_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Ethernet PTP PPS control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptpppscr](index.html) module"]
pub struct PTPPPSCR_SPEC;
impl crate::RegisterSpec for PTPPPSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptpppscr::R](R) reader structure"]
impl crate::Readable for PTPPPSCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTPPPSCR to value 0"]
impl crate::Resettable for PTPPPSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
