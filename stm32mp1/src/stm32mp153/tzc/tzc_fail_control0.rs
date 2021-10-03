#[doc = "Register `TZC_FAIL_CONTROL0` reader"]
pub struct R(crate::R<TZC_FAIL_CONTROL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_FAIL_CONTROL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_FAIL_CONTROL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_FAIL_CONTROL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRIVILEGE` reader - PRIVILEGE"]
pub struct PRIVILEGE_R(crate::FieldReader<bool, bool>);
impl PRIVILEGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIVILEGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIVILEGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NON_SECURE` reader - NON_SECURE"]
pub struct NON_SECURE_R(crate::FieldReader<bool, bool>);
impl NON_SECURE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NON_SECURE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NON_SECURE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRECTION` reader - DIRECTION"]
pub struct DIRECTION_R(crate::FieldReader<bool, bool>);
impl DIRECTION_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRECTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRECTION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 20 - PRIVILEGE"]
    #[inline(always)]
    pub fn privilege(&self) -> PRIVILEGE_R {
        PRIVILEGE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - NON_SECURE"]
    #[inline(always)]
    pub fn non_secure(&self) -> NON_SECURE_R {
        NON_SECURE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DIRECTION"]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
#[doc = "Status information about the first access that failed a region permission check in the associated filter (0 to 1).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_fail_control0](index.html) module"]
pub struct TZC_FAIL_CONTROL0_SPEC;
impl crate::RegisterSpec for TZC_FAIL_CONTROL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_fail_control0::R](R) reader structure"]
impl crate::Readable for TZC_FAIL_CONTROL0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TZC_FAIL_CONTROL0 to value 0"]
impl crate::Resettable for TZC_FAIL_CONTROL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
