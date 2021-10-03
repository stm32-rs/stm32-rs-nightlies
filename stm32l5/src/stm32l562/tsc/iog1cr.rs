#[doc = "Register `IOG1CR` reader"]
pub struct R(crate::R<IOG1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOG1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOG1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOG1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - Counter value"]
pub struct CNT_R(crate::FieldReader<u16, u16>);
impl CNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:13 - Counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "I/O group x counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iog1cr](index.html) module"]
pub struct IOG1CR_SPEC;
impl crate::RegisterSpec for IOG1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iog1cr::R](R) reader structure"]
impl crate::Readable for IOG1CR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IOG1CR to value 0"]
impl crate::Resettable for IOG1CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
