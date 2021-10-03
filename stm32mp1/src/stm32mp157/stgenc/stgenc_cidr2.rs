#[doc = "Register `STGENC_CIDR2` reader"]
pub struct R(crate::R<STGENC_CIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_CIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_CIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_CIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRMBL_2` reader - PRMBL_2"]
pub struct PRMBL_2_R(crate::FieldReader<u8, u8>);
impl PRMBL_2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRMBL_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRMBL_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - PRMBL_2"]
    #[inline(always)]
    pub fn prmbl_2(&self) -> PRMBL_2_R {
        PRMBL_2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "STGENC component ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cidr2](index.html) module"]
pub struct STGENC_CIDR2_SPEC;
impl crate::RegisterSpec for STGENC_CIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenc_cidr2::R](R) reader structure"]
impl crate::Readable for STGENC_CIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STGENC_CIDR2 to value 0x50"]
impl crate::Resettable for STGENC_CIDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x50
    }
}
