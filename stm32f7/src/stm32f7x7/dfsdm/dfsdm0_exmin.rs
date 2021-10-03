#[doc = "Register `DFSDM0_EXMIN` reader"]
pub struct R(crate::R<DFSDM0_EXMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM0_EXMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM0_EXMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM0_EXMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXMINCH` reader - Extremes detector minimum data channel"]
pub struct EXMINCH_R(crate::FieldReader<u8, u8>);
impl EXMINCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXMINCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXMINCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXMIN` reader - Extremes detector minimum value"]
pub struct EXMIN_R(crate::FieldReader<u32, u32>);
impl EXMIN_R {
    pub(crate) fn new(bits: u32) -> Self {
        EXMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXMIN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Extremes detector minimum data channel"]
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:31 - Extremes detector minimum value"]
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
#[doc = "DFSDM Extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_exmin](index.html) module"]
pub struct DFSDM0_EXMIN_SPEC;
impl crate::RegisterSpec for DFSDM0_EXMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm0_exmin::R](R) reader structure"]
impl crate::Readable for DFSDM0_EXMIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM0_EXMIN to value 0x7fff_ff00"]
impl crate::Resettable for DFSDM0_EXMIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7fff_ff00
    }
}
