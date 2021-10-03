#[doc = "Register `IPCC_ID` reader"]
pub struct R(crate::R<IPCC_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCC_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCC_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCC_ID_SPEC>) -> Self {
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
#[doc = "IPCC IP Identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_id](index.html) module"]
pub struct IPCC_ID_SPEC;
impl crate::RegisterSpec for IPCC_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipcc_id::R](R) reader structure"]
impl crate::Readable for IPCC_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IPCC_ID to value 0x0010_0071"]
impl crate::Resettable for IPCC_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_0071
    }
}
