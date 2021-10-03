#[doc = "Register `DDRPERFM_TCNT` reader"]
pub struct R(crate::R<DDRPERFM_TCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_TCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_TCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_TCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - CNT"]
pub struct CNT_R(crate::FieldReader<u32, u32>);
impl CNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "DDRPERFM time counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_tcnt](index.html) module"]
pub struct DDRPERFM_TCNT_SPEC;
impl crate::RegisterSpec for DDRPERFM_TCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrperfm_tcnt::R](R) reader structure"]
impl crate::Readable for DDRPERFM_TCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPERFM_TCNT to value 0"]
impl crate::Resettable for DDRPERFM_TCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
