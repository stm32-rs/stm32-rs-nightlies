#[doc = "Register `DDRPERFM_HWCFG` reader"]
pub struct R(crate::R<DDRPERFM_HWCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_HWCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_HWCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_HWCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NCNT` reader - NCNT"]
pub struct NCNT_R(crate::FieldReader<u8, u8>);
impl NCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        NCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - NCNT"]
    #[inline(always)]
    pub fn ncnt(&self) -> NCNT_R {
        NCNT_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "DDRPERFM hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_hwcfg](index.html) module"]
pub struct DDRPERFM_HWCFG_SPEC;
impl crate::RegisterSpec for DDRPERFM_HWCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrperfm_hwcfg::R](R) reader structure"]
impl crate::Readable for DDRPERFM_HWCFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPERFM_HWCFG to value 0x04"]
impl crate::Resettable for DDRPERFM_HWCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
