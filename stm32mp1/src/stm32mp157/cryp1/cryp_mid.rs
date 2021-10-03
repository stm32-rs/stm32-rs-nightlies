#[doc = "Register `CRYP_MID` reader"]
pub struct R(crate::R<CRYP_MID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_MID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_MID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_MID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MID` reader - MID"]
pub struct MID_R(crate::FieldReader<u32, u32>);
impl MID_R {
    pub(crate) fn new(bits: u32) -> Self {
        MID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - MID"]
    #[inline(always)]
    pub fn mid(&self) -> MID_R {
        MID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "CRYP HW Magic ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_mid](index.html) module"]
pub struct CRYP_MID_SPEC;
impl crate::RegisterSpec for CRYP_MID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_mid::R](R) reader structure"]
impl crate::Readable for CRYP_MID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRYP_MID to value 0xa3c5_dd01"]
impl crate::Resettable for CRYP_MID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa3c5_dd01
    }
}
