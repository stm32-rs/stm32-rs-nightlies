#[doc = "Register `FIFOCNT` reader"]
pub struct R(crate::R<FIFOCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIF0COUNT` reader - FIF0COUNT"]
pub struct FIF0COUNT_R(crate::FieldReader<u32, u32>);
impl FIF0COUNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        FIF0COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIF0COUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - FIF0COUNT"]
    #[inline(always)]
    pub fn fif0count(&self) -> FIF0COUNT_R {
        FIF0COUNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifocnt](index.html) module"]
pub struct FIFOCNT_SPEC;
impl crate::RegisterSpec for FIFOCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifocnt::R](R) reader structure"]
impl crate::Readable for FIFOCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFOCNT to value 0"]
impl crate::Resettable for FIFOCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
