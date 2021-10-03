#[doc = "Register `DIVL` reader"]
pub struct R(crate::R<DIVL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIVL` reader - RTC prescaler divider register Low"]
pub struct DIVL_R(crate::FieldReader<u16, u16>);
impl DIVL_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - RTC prescaler divider register Low"]
    #[inline(always)]
    pub fn divl(&self) -> DIVL_R {
        DIVL_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RTC Prescaler Divider Register Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divl](index.html) module"]
pub struct DIVL_SPEC;
impl crate::RegisterSpec for DIVL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [divl::R](R) reader structure"]
impl crate::Readable for DIVL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIVL to value 0x8000"]
impl crate::Resettable for DIVL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
