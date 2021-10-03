#[doc = "Register `TZC_CID1` reader"]
pub struct R(crate::R<TZC_CID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_CID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_CID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_CID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COMP_ID_1` reader - COMP_ID_1"]
pub struct COMP_ID_1_R(crate::FieldReader<u8, u8>);
impl COMP_ID_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP_ID_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP_ID_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - COMP_ID_1"]
    #[inline(always)]
    pub fn comp_id_1(&self) -> COMP_ID_1_R {
        COMP_ID_1_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_cid1](index.html) module"]
pub struct TZC_CID1_SPEC;
impl crate::RegisterSpec for TZC_CID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_cid1::R](R) reader structure"]
impl crate::Readable for TZC_CID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TZC_CID1 to value 0xf0"]
impl crate::Resettable for TZC_CID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf0
    }
}
