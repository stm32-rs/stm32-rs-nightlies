#[doc = "Register `SECR` reader"]
pub struct R(crate::R<SECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEC_SIZE` reader - Securable memory area size"]
pub struct SEC_SIZE_R(crate::FieldReader<u8, u8>);
impl SEC_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT_LOCK` reader - used to force boot from user area"]
pub struct BOOT_LOCK_R(crate::FieldReader<bool, bool>);
impl BOOT_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:6 - Securable memory area size"]
    #[inline(always)]
    pub fn sec_size(&self) -> SEC_SIZE_R {
        SEC_SIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - used to force boot from user area"]
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "Flash Security register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secr](index.html) module"]
pub struct SECR_SPEC;
impl crate::RegisterSpec for SECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secr::R](R) reader structure"]
impl crate::Readable for SECR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SECR to value 0xf000_0000"]
impl crate::Resettable for SECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf000_0000
    }
}
