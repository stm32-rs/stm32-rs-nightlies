#[doc = "Register `M3FDRL` reader"]
pub struct R(crate::R<M3FDRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M3FDRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M3FDRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M3FDRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FDATAL` reader - Failing data low"]
pub struct FDATAL_R(crate::FieldReader<u32, u32>);
impl FDATAL_R {
    pub(crate) fn new(bits: u32) -> Self {
        FDATAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDATAL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Failing data low"]
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "RAMECC monitor x failing data low register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m3fdrl](index.html) module"]
pub struct M3FDRL_SPEC;
impl crate::RegisterSpec for M3FDRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m3fdrl::R](R) reader structure"]
impl crate::Readable for M3FDRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M3FDRL to value 0"]
impl crate::Resettable for M3FDRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
