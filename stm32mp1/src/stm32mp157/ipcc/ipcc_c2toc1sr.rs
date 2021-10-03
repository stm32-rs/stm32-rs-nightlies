#[doc = "Register `IPCC_C2TOC1SR` reader"]
pub struct R(crate::R<IPCC_C2TOC1SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCC_C2TOC1SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCC_C2TOC1SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCC_C2TOC1SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHxF` reader - CHxF"]
pub struct CHXF_R(crate::FieldReader<u8, u8>);
impl CHXF_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHXF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHXF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - CHxF"]
    #[inline(always)]
    pub fn chx_f(&self) -> CHXF_R {
        CHXF_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "IPCC processor 2 to processor 1 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_c2toc1sr](index.html) module"]
pub struct IPCC_C2TOC1SR_SPEC;
impl crate::RegisterSpec for IPCC_C2TOC1SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipcc_c2toc1sr::R](R) reader structure"]
impl crate::Readable for IPCC_C2TOC1SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IPCC_C2TOC1SR to value 0"]
impl crate::Resettable for IPCC_C2TOC1SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
