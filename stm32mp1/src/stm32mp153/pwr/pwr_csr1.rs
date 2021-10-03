#[doc = "Register `PWR_CSR1` reader"]
pub struct R(crate::R<PWR_CSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PVDO` reader - PVDO"]
pub struct PVDO_R(crate::FieldReader<bool, bool>);
impl PVDO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVDO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVDO` reader - AVDO"]
pub struct AVDO_R(crate::FieldReader<bool, bool>);
impl AVDO_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVDO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - PVDO"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AVDO"]
    #[inline(always)]
    pub fn avdo(&self) -> AVDO_R {
        AVDO_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "Reset on any system reset.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_csr1](index.html) module"]
pub struct PWR_CSR1_SPEC;
impl crate::RegisterSpec for PWR_CSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_csr1::R](R) reader structure"]
impl crate::Readable for PWR_CSR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWR_CSR1 to value 0"]
impl crate::Resettable for PWR_CSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
