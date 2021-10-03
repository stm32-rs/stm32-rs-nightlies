#[doc = "Register `ICACHE_HMONR` reader"]
pub struct R(crate::R<ICACHE_HMONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_HMONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_HMONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_HMONR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HITMON` reader - HITMON"]
pub struct HITMON_R(crate::FieldReader<u32, u32>);
impl HITMON_R {
    pub(crate) fn new(bits: u32) -> Self {
        HITMON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HITMON_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - HITMON"]
    #[inline(always)]
    pub fn hitmon(&self) -> HITMON_R {
        HITMON_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "ICACHE hit monitor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_hmonr](index.html) module"]
pub struct ICACHE_HMONR_SPEC;
impl crate::RegisterSpec for ICACHE_HMONR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_hmonr::R](R) reader structure"]
impl crate::Readable for ICACHE_HMONR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ICACHE_HMONR to value 0"]
impl crate::Resettable for ICACHE_HMONR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
