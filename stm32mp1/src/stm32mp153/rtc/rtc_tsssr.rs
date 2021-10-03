#[doc = "Register `RTC_TSSSR` reader"]
pub struct R(crate::R<RTC_TSSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TSSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TSSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TSSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SS` reader - SS"]
pub struct SS_R(crate::FieldReader<u16, u16>);
impl SS_R {
    pub(crate) fn new(bits: u16) -> Self {
        SS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - SS"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when the TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_tsssr](index.html) module"]
pub struct RTC_TSSSR_SPEC;
impl crate::RegisterSpec for RTC_TSSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_tsssr::R](R) reader structure"]
impl crate::Readable for RTC_TSSSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTC_TSSSR to value 0"]
impl crate::Resettable for RTC_TSSSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
