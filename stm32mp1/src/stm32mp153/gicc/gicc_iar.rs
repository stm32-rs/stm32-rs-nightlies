#[doc = "Register `GICC_IAR` reader"]
pub struct R(crate::R<GICC_IAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICC_IAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICC_IAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICC_IAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTERRUPT_ID` reader - INTERRUPT_ID"]
pub struct INTERRUPT_ID_R(crate::FieldReader<u16, u16>);
impl INTERRUPT_ID_R {
    pub(crate) fn new(bits: u16) -> Self {
        INTERRUPT_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRUPT_ID_R {
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
    #[doc = "Bits 0:9 - INTERRUPT_ID"]
    #[inline(always)]
    pub fn interrupt_id(&self) -> INTERRUPT_ID_R {
        INTERRUPT_ID_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - CPUID"]
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "GICC interrupt acknowledge register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_iar](index.html) module"]
pub struct GICC_IAR_SPEC;
impl crate::RegisterSpec for GICC_IAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicc_iar::R](R) reader structure"]
impl crate::Readable for GICC_IAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICC_IAR to value 0x03ff"]
impl crate::Resettable for GICC_IAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff
    }
}
