#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INITRDY` reader - Initialization mode is ready"]
pub struct INITRDY_R(crate::FieldReader<bool, bool>);
impl INITRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INITRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STABIP` reader - Stabilization in progress status"]
pub struct STABIP_R(crate::FieldReader<bool, bool>);
impl STABIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        STABIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STABIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCIP` reader - Regular conversion in progress status"]
pub struct RCIP_R(crate::FieldReader<bool, bool>);
impl RCIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JCIP` reader - Injected conversion in progress status"]
pub struct JCIP_R(crate::FieldReader<bool, bool>);
impl JCIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        JCIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JCIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALIBIP` reader - Calibration in progress status"]
pub struct CALIBIP_R(crate::FieldReader<bool, bool>);
impl CALIBIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALIBIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALIBIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROVRF` reader - Regular conversion overrun flag"]
pub struct ROVRF_R(crate::FieldReader<bool, bool>);
impl ROVRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROVRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROVRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REOCF` reader - End of regular conversion flag"]
pub struct REOCF_R(crate::FieldReader<bool, bool>);
impl REOCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        REOCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REOCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JOVRF` reader - Injected conversion overrun flag"]
pub struct JOVRF_R(crate::FieldReader<bool, bool>);
impl JOVRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        JOVRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JOVRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEOCF` reader - End of injected conversion flag"]
pub struct JEOCF_R(crate::FieldReader<bool, bool>);
impl JEOCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        JEOCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEOCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOCALF` reader - End of calibration flag"]
pub struct EOCALF_R(crate::FieldReader<bool, bool>);
impl EOCALF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOCALF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOCALF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31 - Initialization mode is ready"]
    #[inline(always)]
    pub fn initrdy(&self) -> INITRDY_R {
        INITRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Stabilization in progress status"]
    #[inline(always)]
    pub fn stabip(&self) -> STABIP_R {
        STABIP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Regular conversion in progress status"]
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Injected conversion in progress status"]
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Calibration in progress status"]
    #[inline(always)]
    pub fn calibip(&self) -> CALIBIP_R {
        CALIBIP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Regular conversion overrun flag"]
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of regular conversion flag"]
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Injected conversion overrun flag"]
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of injected conversion flag"]
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - End of calibration flag"]
    #[inline(always)]
    pub fn eocalf(&self) -> EOCALF_R {
        EOCALF_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
