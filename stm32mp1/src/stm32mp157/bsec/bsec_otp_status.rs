#[doc = "Register `BSEC_OTP_STATUS` reader"]
pub struct R(crate::R<BSEC_OTP_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SECURE` reader - SECURE"]
pub struct SECURE_R(crate::FieldReader<bool, bool>);
impl SECURE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECURE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FULLDBG` reader - FULLDBG"]
pub struct FULLDBG_R(crate::FieldReader<bool, bool>);
impl FULLDBG_R {
    pub(crate) fn new(bits: bool) -> Self {
        FULLDBG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULLDBG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVALID` reader - INVALID"]
pub struct INVALID_R(crate::FieldReader<bool, bool>);
impl INVALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        INVALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVALID_R {
    type Target = crate::FieldReader<bool, bool>;
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
#[doc = "Field `PROGFAIL` reader - PROGFAIL"]
pub struct PROGFAIL_R(crate::FieldReader<bool, bool>);
impl PROGFAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROGFAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROGFAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRON` reader - PWRON"]
pub struct PWRON_R(crate::FieldReader<bool, bool>);
impl PWRON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIST1LOCK` reader - BIST1LOCK"]
pub struct BIST1LOCK_R(crate::FieldReader<bool, bool>);
impl BIST1LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIST1LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIST1LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIST2LOCK` reader - BIST2LOCK"]
pub struct BIST2LOCK_R(crate::FieldReader<bool, bool>);
impl BIST2LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIST2LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIST2LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - SECURE"]
    #[inline(always)]
    pub fn secure(&self) -> SECURE_R {
        SECURE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FULLDBG"]
    #[inline(always)]
    pub fn fulldbg(&self) -> FULLDBG_R {
        FULLDBG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - INVALID"]
    #[inline(always)]
    pub fn invalid(&self) -> INVALID_R {
        INVALID_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PROGFAIL"]
    #[inline(always)]
    pub fn progfail(&self) -> PROGFAIL_R {
        PROGFAIL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWRON"]
    #[inline(always)]
    pub fn pwron(&self) -> PWRON_R {
        PWRON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BIST1LOCK"]
    #[inline(always)]
    pub fn bist1lock(&self) -> BIST1LOCK_R {
        BIST1LOCK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BIST2LOCK"]
    #[inline(always)]
    pub fn bist2lock(&self) -> BIST2LOCK_R {
        BIST2LOCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "BSEC OTP status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_status](index.html) module"]
pub struct BSEC_OTP_STATUS_SPEC;
impl crate::RegisterSpec for BSEC_OTP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_otp_status::R](R) reader structure"]
impl crate::Readable for BSEC_OTP_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BSEC_OTP_STATUS to value 0"]
impl crate::Resettable for BSEC_OTP_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
