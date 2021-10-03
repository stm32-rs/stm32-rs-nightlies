#[doc = "Register `GICD_ITARGETSR5` reader"]
pub struct R(crate::R<GICD_ITARGETSR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPU_TARGETS0` reader - CPU_TARGETS0"]
pub struct CPU_TARGETS0_R(crate::FieldReader<u8, u8>);
impl CPU_TARGETS0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPU_TARGETS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TARGETS0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TARGETS1` reader - CPU_TARGETS1"]
pub struct CPU_TARGETS1_R(crate::FieldReader<u8, u8>);
impl CPU_TARGETS1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPU_TARGETS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TARGETS1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TARGETS2` reader - CPU_TARGETS2"]
pub struct CPU_TARGETS2_R(crate::FieldReader<u8, u8>);
impl CPU_TARGETS2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPU_TARGETS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TARGETS2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TARGETS3` reader - CPU_TARGETS3"]
pub struct CPU_TARGETS3_R(crate::FieldReader<u8, u8>);
impl CPU_TARGETS3_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPU_TARGETS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TARGETS3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - CPU_TARGETS0"]
    #[inline(always)]
    pub fn cpu_targets0(&self) -> CPU_TARGETS0_R {
        CPU_TARGETS0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - CPU_TARGETS1"]
    #[inline(always)]
    pub fn cpu_targets1(&self) -> CPU_TARGETS1_R {
        CPU_TARGETS1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - CPU_TARGETS2"]
    #[inline(always)]
    pub fn cpu_targets2(&self) -> CPU_TARGETS2_R {
        CPU_TARGETS2_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - CPU_TARGETS3"]
    #[inline(always)]
    pub fn cpu_targets3(&self) -> CPU_TARGETS3_R {
        CPU_TARGETS3_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr5](index.html) module"]
pub struct GICD_ITARGETSR5_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr5::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_ITARGETSR5 to value 0"]
impl crate::Resettable for GICD_ITARGETSR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
