#[doc = "Register `TZC_INT_STATUS` reader"]
pub struct R(crate::R<TZC_INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATUS` reader - STATUS"]
pub struct STATUS_R(crate::FieldReader<u8, u8>);
impl STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN` reader - OVERRUN"]
pub struct OVERRUN_R(crate::FieldReader<u8, u8>);
impl OVERRUN_R {
    pub(crate) fn new(bits: u8) -> Self {
        OVERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERLAP` reader - OVERLAP"]
pub struct OVERLAP_R(crate::FieldReader<u8, u8>);
impl OVERLAP_R {
    pub(crate) fn new(bits: u8) -> Self {
        OVERLAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERLAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - STATUS"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - OVERRUN"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - OVERLAP"]
    #[inline(always)]
    pub fn overlap(&self) -> OVERLAP_R {
        OVERLAP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
#[doc = "Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_int_status](index.html) module"]
pub struct TZC_INT_STATUS_SPEC;
impl crate::RegisterSpec for TZC_INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_int_status::R](R) reader structure"]
impl crate::Readable for TZC_INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TZC_INT_STATUS to value 0"]
impl crate::Resettable for TZC_INT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
