#[doc = "Register `DDRPERFM_ISR` reader"]
pub struct R(crate::R<DDRPERFM_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVFF` reader - OVFF"]
pub struct OVFF_R(crate::FieldReader<bool, bool>);
impl OVFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - OVFF"]
    #[inline(always)]
    pub fn ovff(&self) -> OVFF_R {
        OVFF_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "DDRPERFM interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_isr](index.html) module"]
pub struct DDRPERFM_ISR_SPEC;
impl crate::RegisterSpec for DDRPERFM_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrperfm_isr::R](R) reader structure"]
impl crate::Readable for DDRPERFM_ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPERFM_ISR to value 0"]
impl crate::Resettable for DDRPERFM_ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
