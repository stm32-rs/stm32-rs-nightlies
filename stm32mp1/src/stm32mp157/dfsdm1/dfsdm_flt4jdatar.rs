#[doc = "Register `DFSDM_FLT4JDATAR` reader"]
pub struct R(crate::R<DFSDM_FLT4JDATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT4JDATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT4JDATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT4JDATAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDATACH` reader - JDATACH"]
pub struct JDATACH_R(crate::FieldReader<u8, u8>);
impl JDATACH_R {
    pub(crate) fn new(bits: u8) -> Self {
        JDATACH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JDATACH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JDATA` reader - JDATA"]
pub struct JDATA_R(crate::FieldReader<u32, u32>);
impl JDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        JDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - JDATACH"]
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:31 - JDATA"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
#[doc = "DFSDM filter 4 data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt4jdatar](index.html) module"]
pub struct DFSDM_FLT4JDATAR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT4JDATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_flt4jdatar::R](R) reader structure"]
impl crate::Readable for DFSDM_FLT4JDATAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM_FLT4JDATAR to value 0"]
impl crate::Resettable for DFSDM_FLT4JDATAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
