#[doc = "Register `DFSDM_FLT3EXMAX` reader"]
pub struct R(crate::R<DFSDM_FLT3EXMAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT3EXMAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT3EXMAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT3EXMAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXMAXCH` reader - EXMAXCH"]
pub struct EXMAXCH_R(crate::FieldReader<u8, u8>);
impl EXMAXCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXMAXCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXMAXCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXMAX` reader - EXMAX"]
pub struct EXMAX_R(crate::FieldReader<u32, u32>);
impl EXMAX_R {
    pub(crate) fn new(bits: u32) -> Self {
        EXMAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXMAX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - EXMAXCH"]
    #[inline(always)]
    pub fn exmaxch(&self) -> EXMAXCH_R {
        EXMAXCH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:31 - EXMAX"]
    #[inline(always)]
    pub fn exmax(&self) -> EXMAX_R {
        EXMAX_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
#[doc = "DFSDM filter 3 extremes detector maximum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3exmax](index.html) module"]
pub struct DFSDM_FLT3EXMAX_SPEC;
impl crate::RegisterSpec for DFSDM_FLT3EXMAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_flt3exmax::R](R) reader structure"]
impl crate::Readable for DFSDM_FLT3EXMAX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM_FLT3EXMAX to value 0x8000_0000"]
impl crate::Resettable for DFSDM_FLT3EXMAX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
