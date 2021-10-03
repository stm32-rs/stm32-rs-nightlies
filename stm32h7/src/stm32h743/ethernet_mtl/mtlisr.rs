#[doc = "Register `MTLISR` reader"]
pub struct R(crate::R<MTLISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Q0IS` reader - Queue interrupt status"]
pub struct Q0IS_R(crate::FieldReader<bool, bool>);
impl Q0IS_R {
    pub(crate) fn new(bits: bool) -> Self {
        Q0IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Q0IS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Queue interrupt status"]
    #[inline(always)]
    pub fn q0is(&self) -> Q0IS_R {
        Q0IS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Interrupt status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlisr](index.html) module"]
pub struct MTLISR_SPEC;
impl crate::RegisterSpec for MTLISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtlisr::R](R) reader structure"]
impl crate::Readable for MTLISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MTLISR to value 0"]
impl crate::Resettable for MTLISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
