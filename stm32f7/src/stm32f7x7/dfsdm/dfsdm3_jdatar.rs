#[doc = "Register `DFSDM3_JDATAR` reader"]
pub struct R(crate::R<DFSDM3_JDATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM3_JDATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM3_JDATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM3_JDATAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDATACH` reader - Injected channel most recently converted"]
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
#[doc = "Field `JDATA` reader - Injected group conversion data"]
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
    #[doc = "Bits 0:2 - Injected channel most recently converted"]
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:31 - Injected group conversion data"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
#[doc = "DFSDM data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_jdatar](index.html) module"]
pub struct DFSDM3_JDATAR_SPEC;
impl crate::RegisterSpec for DFSDM3_JDATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm3_jdatar::R](R) reader structure"]
impl crate::Readable for DFSDM3_JDATAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM3_JDATAR to value 0"]
impl crate::Resettable for DFSDM3_JDATAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
