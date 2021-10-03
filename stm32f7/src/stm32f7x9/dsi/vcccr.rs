#[doc = "Register `VCCCR` reader"]
pub struct R(crate::R<VCCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUMC` reader - Number of Chunks"]
pub struct NUMC_R(crate::FieldReader<u16, u16>);
impl NUMC_R {
    pub(crate) fn new(bits: u16) -> Self {
        NUMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUMC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:12 - Number of Chunks"]
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x1fff) as u16)
    }
}
#[doc = "DSI Host Video Chunks Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vcccr](index.html) module"]
pub struct VCCCR_SPEC;
impl crate::RegisterSpec for VCCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vcccr::R](R) reader structure"]
impl crate::Readable for VCCCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VCCCR to value 0"]
impl crate::Resettable for VCCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
