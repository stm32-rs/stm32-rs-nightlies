#[doc = "Register `HWCFGR1` reader"]
pub struct R(crate::R<HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NBEVENTS` reader - HW configuration number of event"]
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
#[doc = "Field `NBCPUS` reader - HW configuration number of CPUs"]
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
#[doc = "Field `CPUEVTEN` reader - HW configuration of CPU(m) event output enable"]
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
impl R {
    #[doc = "Bits 0:7 - HW configuration number of event"]
    #[inline(always)]
    pub fn nbevents(&self) -> NBEVENTS_R {
        NBEVENTS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - HW configuration number of CPUs"]
    #[inline(always)]
    pub fn nbcpus(&self) -> NBCPUS_R {
        NBCPUS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - HW configuration of CPU(m) event output enable"]
    #[inline(always)]
    pub fn cpuevten(&self) -> CPUEVTEN_R {
        CPUEVTEN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "Hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr1](index.html) module"]
pub struct HWCFGR1_SPEC;
impl crate::RegisterSpec for HWCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr1::R](R) reader structure"]
impl crate::Readable for HWCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWCFGR1 to value 0x3130"]
impl crate::Resettable for HWCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3130
    }
}
