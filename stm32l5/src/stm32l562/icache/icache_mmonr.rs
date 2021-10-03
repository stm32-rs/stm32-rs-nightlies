#[doc = "Register `ICACHE_MMONR` reader"]
pub struct R(crate::R<ICACHE_MMONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_MMONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_MMONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_MMONR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MISSMON` reader - MISSMON"]
pub struct MISSMON_R(crate::FieldReader<u16, u16>);
impl MISSMON_R {
    pub(crate) fn new(bits: u16) -> Self {
        MISSMON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISSMON_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - MISSMON"]
    #[inline(always)]
    pub fn missmon(&self) -> MISSMON_R {
        MISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ICACHE miss monitor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_mmonr](index.html) module"]
pub struct ICACHE_MMONR_SPEC;
impl crate::RegisterSpec for ICACHE_MMONR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_mmonr::R](R) reader structure"]
impl crate::Readable for ICACHE_MMONR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ICACHE_MMONR to value 0"]
impl crate::Resettable for ICACHE_MMONR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
