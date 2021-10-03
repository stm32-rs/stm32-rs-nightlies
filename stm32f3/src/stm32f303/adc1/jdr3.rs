#[doc = "Register `JDR3` reader"]
pub struct R(crate::R<JDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDATA3` reader - JDATA3"]
pub struct JDATA3_R(crate::FieldReader<u16, u16>);
impl JDATA3_R {
    pub(crate) fn new(bits: u16) -> Self {
        JDATA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JDATA3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - JDATA3"]
    #[inline(always)]
    pub fn jdata3(&self) -> JDATA3_R {
        JDATA3_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "injected data register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr3](index.html) module"]
pub struct JDR3_SPEC;
impl crate::RegisterSpec for JDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jdr3::R](R) reader structure"]
impl crate::Readable for JDR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JDR3 to value 0"]
impl crate::Resettable for JDR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
