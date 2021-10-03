#[doc = "Register `ITLINE2` reader"]
pub struct R(crate::R<ITLINE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TAMP` reader - TAMP"]
pub struct TAMP_R(crate::FieldReader<bool, bool>);
impl TAMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC` reader - RTC"]
pub struct RTC_R(crate::FieldReader<bool, bool>);
impl RTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TAMP"]
    #[inline(always)]
    pub fn tamp(&self) -> TAMP_R {
        TAMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "interrupt line 2 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline2](index.html) module"]
pub struct ITLINE2_SPEC;
impl crate::RegisterSpec for ITLINE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline2::R](R) reader structure"]
impl crate::Readable for ITLINE2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE2 to value 0"]
impl crate::Resettable for ITLINE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
