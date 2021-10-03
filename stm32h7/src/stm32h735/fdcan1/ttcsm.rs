#[doc = "Register `TTCSM` reader"]
pub struct R(crate::R<TTCSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTCSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTCSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTCSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSM` reader - Cycle Sync Mark"]
pub struct CSM_R(crate::FieldReader<u16, u16>);
impl CSM_R {
    pub(crate) fn new(bits: u16) -> Self {
        CSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Cycle Sync Mark"]
    #[inline(always)]
    pub fn csm(&self) -> CSM_R {
        CSM_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "FDCAN TT Cycle Sync Mark Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttcsm](index.html) module"]
pub struct TTCSM_SPEC;
impl crate::RegisterSpec for TTCSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttcsm::R](R) reader structure"]
impl crate::Readable for TTCSM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TTCSM to value 0"]
impl crate::Resettable for TTCSM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
