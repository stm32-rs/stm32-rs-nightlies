#[doc = "Register `BSEC_HWCFGR` reader"]
pub struct R(crate::R<BSEC_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIZE` reader - SIZE"]
pub struct SIZE_R(crate::FieldReader<u8, u8>);
impl SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECC_USE` reader - ECC_USE"]
pub struct ECC_USE_R(crate::FieldReader<u8, u8>);
impl ECC_USE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECC_USE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECC_USE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - SIZE"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ECC_USE"]
    #[inline(always)]
    pub fn ecc_use(&self) -> ECC_USE_R {
        ECC_USE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "BSEC hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_hwcfgr](index.html) module"]
pub struct BSEC_HWCFGR_SPEC;
impl crate::RegisterSpec for BSEC_HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_hwcfgr::R](R) reader structure"]
impl crate::Readable for BSEC_HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BSEC_HWCFGR to value 0x14"]
impl crate::Resettable for BSEC_HWCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x14
    }
}
