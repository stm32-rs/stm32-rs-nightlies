#[doc = "Register `GICC_AHPPIR` reader"]
pub struct R(crate::R<GICC_AHPPIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICC_AHPPIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICC_AHPPIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICC_AHPPIR_SPEC>) -> Self {
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
#[doc = "ICC_AHPPIR is an alias of the non-secure GICC_HPPIR. A secure access to this register is equivalent to a non-secure access to GICC_HPPIR.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_ahppir](index.html) module"]
pub struct GICC_AHPPIR_SPEC;
impl crate::RegisterSpec for GICC_AHPPIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicc_ahppir::R](R) reader structure"]
impl crate::Readable for GICC_AHPPIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICC_AHPPIR to value 0x03ff"]
impl crate::Resettable for GICC_AHPPIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff
    }
}