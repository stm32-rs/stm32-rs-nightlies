#[doc = "Register `HSEM_C1ISR` reader"]
pub struct R(crate::R<HSEM_C1ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSEM_C1ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSEM_C1ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSEM_C1ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ISF` reader - ISF"]
pub struct ISF_R(crate::FieldReader<u32, u32>);
impl ISF_R {
    pub(crate) fn new(bits: u32) -> Self {
        ISF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - ISF"]
    #[inline(always)]
    pub fn isf(&self) -> ISF_R {
        ISF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "HSEM i1terrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_c1isr](index.html) module"]
pub struct HSEM_C1ISR_SPEC;
impl crate::RegisterSpec for HSEM_C1ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsem_c1isr::R](R) reader structure"]
impl crate::Readable for HSEM_C1ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HSEM_C1ISR to value 0"]
impl crate::Resettable for HSEM_C1ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
