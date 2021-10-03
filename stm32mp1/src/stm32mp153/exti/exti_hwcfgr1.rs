#[doc = "Register `EXTI_HWCFGR1` reader"]
pub struct R(crate::R<EXTI_HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_HWCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NBEVENTS` reader - NBEVENTS"]
pub struct NBEVENTS_R(crate::FieldReader<u8, u8>);
impl NBEVENTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBEVENTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBEVENTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBCPUS` reader - NBCPUS"]
pub struct NBCPUS_R(crate::FieldReader<u8, u8>);
impl NBCPUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBCPUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBCPUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPUEVTEN` reader - CPUEVTEN"]
pub struct CPUEVTEN_R(crate::FieldReader<u8, u8>);
impl CPUEVTEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPUEVTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPUEVTEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBIOPORT` reader - NBIOPORT"]
pub struct NBIOPORT_R(crate::FieldReader<u8, u8>);
impl NBIOPORT_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBIOPORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBIOPORT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - NBEVENTS"]
    #[inline(always)]
    pub fn nbevents(&self) -> NBEVENTS_R {
        NBEVENTS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - NBCPUS"]
    #[inline(always)]
    pub fn nbcpus(&self) -> NBCPUS_R {
        NBCPUS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - CPUEVTEN"]
    #[inline(always)]
    pub fn cpuevten(&self) -> CPUEVTEN_R {
        CPUEVTEN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - NBIOPORT"]
    #[inline(always)]
    pub fn nbioport(&self) -> NBIOPORT_R {
        NBIOPORT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "EXTI hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr1](index.html) module"]
pub struct EXTI_HWCFGR1_SPEC;
impl crate::RegisterSpec for EXTI_HWCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_hwcfgr1::R](R) reader structure"]
impl crate::Readable for EXTI_HWCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXTI_HWCFGR1 to value 0x000b_214b"]
impl crate::Resettable for EXTI_HWCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000b_214b
    }
}
