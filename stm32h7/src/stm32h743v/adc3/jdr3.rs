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
#[doc = "Field `JDATA3` reader - ADC group injected sequencer rank 3 conversion data"]
pub struct JDATA3_R(crate::FieldReader<u32, u32>);
impl JDATA3_R {
    pub(crate) fn new(bits: u32) -> Self {
        JDATA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JDATA3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - ADC group injected sequencer rank 3 conversion data"]
    #[inline(always)]
    pub fn jdata3(&self) -> JDATA3_R {
        JDATA3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "ADC group injected sequencer rank 3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr3](index.html) module"]
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
