#[doc = "Register `DMAISR` reader"]
pub struct R(crate::R<DMAISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DC0IS` reader - DMA Channel Interrupt Status"]
pub struct DC0IS_R(crate::FieldReader<bool, bool>);
impl DC0IS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DC0IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC0IS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTLIS` reader - MTL Interrupt Status"]
pub struct MTLIS_R(crate::FieldReader<bool, bool>);
impl MTLIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTLIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTLIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACIS` reader - MAC Interrupt Status"]
pub struct MACIS_R(crate::FieldReader<bool, bool>);
impl MACIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MACIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - DMA Channel Interrupt Status"]
    #[inline(always)]
    pub fn dc0is(&self) -> DC0IS_R {
        DC0IS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - MTL Interrupt Status"]
    #[inline(always)]
    pub fn mtlis(&self) -> MTLIS_R {
        MTLIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MAC Interrupt Status"]
    #[inline(always)]
    pub fn macis(&self) -> MACIS_R {
        MACIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaisr](index.html) module"]
pub struct DMAISR_SPEC;
impl crate::RegisterSpec for DMAISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaisr::R](R) reader structure"]
impl crate::Readable for DMAISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMAISR to value 0"]
impl crate::Resettable for DMAISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
