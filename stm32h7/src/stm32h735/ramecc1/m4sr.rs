#[doc = "Register `M4SR` reader"]
pub struct R(crate::R<M4SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4SR_SPEC>) -> Self {
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
#[doc = "RAMECC monitor x status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4sr](index.html) module"]
pub struct M4SR_SPEC;
impl crate::RegisterSpec for M4SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4sr::R](R) reader structure"]
impl crate::Readable for M4SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M4SR to value 0"]
impl crate::Resettable for M4SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
