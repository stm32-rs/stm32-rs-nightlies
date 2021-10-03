#[doc = "Register `CDSR` reader"]
pub struct R(crate::R<CDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VDES` reader - Vertical Data Enable display Status"]
pub struct VDES_R(crate::FieldReader<bool, bool>);
impl VDES_R {
    pub(crate) fn new(bits: bool) -> Self {
        VDES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDES` reader - Horizontal Data Enable display Status"]
pub struct HDES_R(crate::FieldReader<bool, bool>);
impl HDES_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSYNCS` reader - Vertical Synchronization display Status"]
pub struct VSYNCS_R(crate::FieldReader<bool, bool>);
impl VSYNCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VSYNCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSYNCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSYNCS` reader - Horizontal Synchronization display Status"]
pub struct HSYNCS_R(crate::FieldReader<bool, bool>);
impl HSYNCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSYNCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSYNCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Vertical Data Enable display Status"]
    #[inline(always)]
    pub fn vdes(&self) -> VDES_R {
        VDES_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Horizontal Data Enable display Status"]
    #[inline(always)]
    pub fn hdes(&self) -> HDES_R {
        HDES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Vertical Synchronization display Status"]
    #[inline(always)]
    pub fn vsyncs(&self) -> VSYNCS_R {
        VSYNCS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Horizontal Synchronization display Status"]
    #[inline(always)]
    pub fn hsyncs(&self) -> HSYNCS_R {
        HSYNCS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "LTDC Current Display Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdsr](index.html) module"]
pub struct CDSR_SPEC;
impl crate::RegisterSpec for CDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdsr::R](R) reader structure"]
impl crate::Readable for CDSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CDSR to value 0x0f"]
impl crate::Resettable for CDSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
