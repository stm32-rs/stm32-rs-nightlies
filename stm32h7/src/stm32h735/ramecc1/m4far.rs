#[doc = "Register `M4FAR` reader"]
pub struct R(crate::R<M4FAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4FAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4FAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4FAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FDATAH` reader - Failing data high (64-bit memory)"]
pub struct FDATAH_R(crate::FieldReader<u32, u32>);
impl FDATAH_R {
    pub(crate) fn new(bits: u32) -> Self {
        FDATAH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDATAH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "RAMECC monitor x failing address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4far](index.html) module"]
pub struct M4FAR_SPEC;
impl crate::RegisterSpec for M4FAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4far::R](R) reader structure"]
impl crate::Readable for M4FAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M4FAR to value 0"]
impl crate::Resettable for M4FAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
