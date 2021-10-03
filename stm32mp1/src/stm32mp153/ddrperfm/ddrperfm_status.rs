#[doc = "Register `DDRPERFM_STATUS` reader"]
pub struct R(crate::R<DDRPERFM_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COVF` reader - COVF"]
pub struct COVF_R(crate::FieldReader<u8, u8>);
impl COVF_R {
    pub(crate) fn new(bits: u8) -> Self {
        COVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COVF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` reader - BUSY"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOVF` reader - TOVF"]
pub struct TOVF_R(crate::FieldReader<bool, bool>);
impl TOVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - COVF"]
    #[inline(always)]
    pub fn covf(&self) -> COVF_R {
        COVF_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 31 - TOVF"]
    #[inline(always)]
    pub fn tovf(&self) -> TOVF_R {
        TOVF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "DDRPERFM status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_status](index.html) module"]
pub struct DDRPERFM_STATUS_SPEC;
impl crate::RegisterSpec for DDRPERFM_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrperfm_status::R](R) reader structure"]
impl crate::Readable for DDRPERFM_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPERFM_STATUS to value 0"]
impl crate::Resettable for DDRPERFM_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
