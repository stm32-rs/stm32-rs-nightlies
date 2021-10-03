#[doc = "Register `DFSDM1_AWSR` reader"]
pub struct R(crate::R<DFSDM1_AWSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM1_AWSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM1_AWSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM1_AWSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AWLTF` reader - Analog watchdog low threshold flag"]
pub struct AWLTF_R(crate::FieldReader<u8, u8>);
impl AWLTF_R {
    pub(crate) fn new(bits: u8) -> Self {
        AWLTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWLTF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWHTF` reader - Analog watchdog high threshold flag"]
pub struct AWHTF_R(crate::FieldReader<u8, u8>);
impl AWHTF_R {
    pub(crate) fn new(bits: u8) -> Self {
        AWHTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWHTF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Analog watchdog low threshold flag"]
    #[inline(always)]
    pub fn awltf(&self) -> AWLTF_R {
        AWLTF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Analog watchdog high threshold flag"]
    #[inline(always)]
    pub fn awhtf(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "DFSDM analog watchdog status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_awsr](index.html) module"]
pub struct DFSDM1_AWSR_SPEC;
impl crate::RegisterSpec for DFSDM1_AWSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm1_awsr::R](R) reader structure"]
impl crate::Readable for DFSDM1_AWSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM1_AWSR to value 0"]
impl crate::Resettable for DFSDM1_AWSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
