#[doc = "Register `JDR4` reader"]
pub struct R(crate::R<JDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDATA4` reader - ADC group injected sequencer rank 4 conversion data"]
pub struct JDATA4_R(crate::FieldReader<u32, u32>);
impl JDATA4_R {
    pub(crate) fn new(bits: u32) -> Self {
        JDATA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JDATA4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - ADC group injected sequencer rank 4 conversion data"]
    #[inline(always)]
    pub fn jdata4(&self) -> JDATA4_R {
        JDATA4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "ADC group injected sequencer rank 4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr4](index.html) module"]
pub struct JDR4_SPEC;
impl crate::RegisterSpec for JDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jdr4::R](R) reader structure"]
impl crate::Readable for JDR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JDR4 to value 0"]
impl crate::Resettable for JDR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
