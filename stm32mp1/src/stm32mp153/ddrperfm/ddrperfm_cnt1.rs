#[doc = "Register `DDRPERFM_CNT1` reader"]
pub struct R(crate::R<DDRPERFM_CNT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_CNT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_CNT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_CNT1_SPEC>) -> Self {
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
#[doc = "DDRPERFM event counter 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_cnt1](index.html) module"]
pub struct DDRPERFM_CNT1_SPEC;
impl crate::RegisterSpec for DDRPERFM_CNT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrperfm_cnt1::R](R) reader structure"]
impl crate::Readable for DDRPERFM_CNT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPERFM_CNT1 to value 0"]
impl crate::Resettable for DDRPERFM_CNT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
