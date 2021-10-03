#[doc = "Register `FAR` reader"]
pub struct R(crate::R<FAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FAIL_ECC_ADDR` reader - Bank 1 ECC error address"]
pub struct FAIL_ECC_ADDR_R(crate::FieldReader<u16, u16>);
impl FAIL_ECC_ADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        FAIL_ECC_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAIL_ECC_ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:14 - Bank 1 ECC error address"]
    #[inline(always)]
    pub fn fail_ecc_addr(&self) -> FAIL_ECC_ADDR_R {
        FAIL_ECC_ADDR_R::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "FLASH ECC fail address for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [far](index.html) module"]
pub struct FAR_SPEC;
impl crate::RegisterSpec for FAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [far::R](R) reader structure"]
impl crate::Readable for FAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FAR to value 0"]
impl crate::Resettable for FAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
