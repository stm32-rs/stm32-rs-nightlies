#[doc = "Register `PCROP1ASR` reader"]
pub struct R(crate::R<PCROP1ASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP1ASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP1ASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP1ASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PCROP1A_STRT` reader - PCROP1A area start offset"]
pub struct PCROP1A_STRT_R(crate::FieldReader<u8, u8>);
impl PCROP1A_STRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCROP1A_STRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP1A_STRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - PCROP1A area start offset"]
    #[inline(always)]
    pub fn pcrop1a_strt(&self) -> PCROP1A_STRT_R {
        PCROP1A_STRT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Flash PCROP zone A Start address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop1asr](index.html) module"]
pub struct PCROP1ASR_SPEC;
impl crate::RegisterSpec for PCROP1ASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcrop1asr::R](R) reader structure"]
impl crate::Readable for PCROP1ASR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCROP1ASR to value 0xf000_0000"]
impl crate::Resettable for PCROP1ASR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf000_0000
    }
}
