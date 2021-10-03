#[doc = "Register `RLR25` reader"]
pub struct R(crate::R<RLR25_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLR25_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLR25_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLR25_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PROCID` reader - Semaphore ProcessID"]
pub struct PROCID_R(crate::FieldReader<u8, u8>);
impl PROCID_R {
    pub(crate) fn new(bits: u8) -> Self {
        PROCID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROCID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTERID` reader - Semaphore MasterID"]
pub struct MASTERID_R(crate::FieldReader<u8, u8>);
impl MASTERID_R {
    pub(crate) fn new(bits: u8) -> Self {
        MASTERID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTERID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` reader - Lock indication"]
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Semaphore MasterID"]
    #[inline(always)]
    pub fn masterid(&self) -> MASTERID_R {
        MASTERID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Lock indication"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr25](index.html) module"]
pub struct RLR25_SPEC;
impl crate::RegisterSpec for RLR25_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rlr25::R](R) reader structure"]
impl crate::Readable for RLR25_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RLR25 to value 0"]
impl crate::Resettable for RLR25_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
