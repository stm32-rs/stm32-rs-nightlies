#[doc = "Register `JDR1` reader"]
pub struct R(crate::R<JDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDATA1` reader - JDATA1"]
pub struct JDATA1_R(crate::FieldReader<u16, u16>);
impl JDATA1_R {
    pub(crate) fn new(bits: u16) -> Self {
        JDATA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JDATA1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - JDATA1"]
    #[inline(always)]
    pub fn jdata1(&self) -> JDATA1_R {
        JDATA1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "injected data register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr1](index.html) module"]
pub struct JDR1_SPEC;
impl crate::RegisterSpec for JDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jdr1::R](R) reader structure"]
impl crate::Readable for JDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JDR1 to value 0"]
impl crate::Resettable for JDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
