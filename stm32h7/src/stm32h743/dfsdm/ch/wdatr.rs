#[doc = "Register `WDATR` reader"]
pub struct R(crate::R<WDATR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDATR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDATR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDATR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDATA` reader - Input channel y watchdog data"]
pub struct WDATA_R(crate::FieldReader<u16, u16>);
impl WDATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        WDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Input channel y watchdog data"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DFSDM channel watchdog filter data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdatr](index.html) module"]
pub struct WDATR_SPEC;
impl crate::RegisterSpec for WDATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdatr::R](R) reader structure"]
impl crate::Readable for WDATR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDATR to value 0"]
impl crate::Resettable for WDATR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
