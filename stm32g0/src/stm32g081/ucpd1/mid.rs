#[doc = "Register `MID` reader"]
pub struct R(crate::R<MID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IPID` reader - IPID"]
pub struct IPID_R(crate::FieldReader<u32, u32>);
impl IPID_R {
    pub(crate) fn new(bits: u32) -> Self {
        IPID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - IPID"]
    #[inline(always)]
    pub fn ipid(&self) -> IPID_R {
        IPID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "UCPD IP ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mid](index.html) module"]
pub struct MID_SPEC;
impl crate::RegisterSpec for MID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mid::R](R) reader structure"]
impl crate::Readable for MID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MID to value 0xa3c5_dd01"]
impl crate::Resettable for MID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa3c5_dd01
    }
}
