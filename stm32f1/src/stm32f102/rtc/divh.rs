#[doc = "Register `DIVH` reader"]
pub struct R(crate::R<DIVH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIVH` reader - RTC prescaler divider register high"]
pub struct DIVH_R(crate::FieldReader<u8, u8>);
impl DIVH_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - RTC prescaler divider register high"]
    #[inline(always)]
    pub fn divh(&self) -> DIVH_R {
        DIVH_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "RTC Prescaler Divider Register High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divh](index.html) module"]
pub struct DIVH_SPEC;
impl crate::RegisterSpec for DIVH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [divh::R](R) reader structure"]
impl crate::Readable for DIVH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIVH to value 0"]
impl crate::Resettable for DIVH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
