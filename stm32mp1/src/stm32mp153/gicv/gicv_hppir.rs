#[doc = "Register `GICV_HPPIR` reader"]
pub struct R(crate::R<GICV_HPPIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICV_HPPIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICV_HPPIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICV_HPPIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PENDINTID` reader - PENDINTID"]
pub struct PENDINTID_R(crate::FieldReader<u16, u16>);
impl PENDINTID_R {
    pub(crate) fn new(bits: u16) -> Self {
        PENDINTID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDINTID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPUID` reader - CPUID"]
pub struct CPUID_R(crate::FieldReader<bool, bool>);
impl CPUID_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPUID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPUID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - PENDINTID"]
    #[inline(always)]
    pub fn pendintid(&self) -> PENDINTID_R {
        PENDINTID_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - CPUID"]
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "GICV VM highest priority pending interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_hppir](index.html) module"]
pub struct GICV_HPPIR_SPEC;
impl crate::RegisterSpec for GICV_HPPIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicv_hppir::R](R) reader structure"]
impl crate::Readable for GICV_HPPIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICV_HPPIR to value 0x03ff"]
impl crate::Resettable for GICV_HPPIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff
    }
}
