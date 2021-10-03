#[doc = "Register `MACSTSR` reader"]
pub struct R(crate::R<MACSTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACSTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACSTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACSTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSS` reader - Timestamp Second"]
pub struct TSS_R(crate::FieldReader<u32, u32>);
impl TSS_R {
    pub(crate) fn new(bits: u32) -> Self {
        TSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "System time seconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macstsr](index.html) module"]
pub struct MACSTSR_SPEC;
impl crate::RegisterSpec for MACSTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macstsr::R](R) reader structure"]
impl crate::Readable for MACSTSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MACSTSR to value 0"]
impl crate::Resettable for MACSTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
