#[doc = "Register `C2MISR` reader"]
pub struct R(crate::R<C2MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2MISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MISFm` reader - masked CPU(2) semaphore m status bit after enable (mask)."]
pub struct MISFM_R(crate::FieldReader<u32, u32>);
impl MISFM_R {
    pub(crate) fn new(bits: u32) -> Self {
        MISFM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISFM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - masked CPU(2) semaphore m status bit after enable (mask)."]
    #[inline(always)]
    pub fn misfm(&self) -> MISFM_R {
        MISFM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "HSEM Masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2misr](index.html) module"]
pub struct C2MISR_SPEC;
impl crate::RegisterSpec for C2MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2misr::R](R) reader structure"]
impl crate::Readable for C2MISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets C2MISR to value 0"]
impl crate::Resettable for C2MISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
