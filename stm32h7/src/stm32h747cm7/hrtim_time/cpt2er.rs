#[doc = "Register `CPT2ER` reader"]
pub struct R(crate::R<CPT2ER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPT2ER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPT2ER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPT2ER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPT2x` reader - Timerx Capture 2 value"]
pub struct CPT2X_R(crate::FieldReader<u16, u16>);
impl CPT2X_R {
    pub(crate) fn new(bits: u16) -> Self {
        CPT2X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPT2X_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 2 value"]
    #[inline(always)]
    pub fn cpt2x(&self) -> CPT2X_R {
        CPT2X_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timerx Capture 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt2er](index.html) module"]
pub struct CPT2ER_SPEC;
impl crate::RegisterSpec for CPT2ER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpt2er::R](R) reader structure"]
impl crate::Readable for CPT2ER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPT2ER to value 0"]
impl crate::Resettable for CPT2ER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
