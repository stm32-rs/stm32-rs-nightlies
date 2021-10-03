#[doc = "Register `UR12` reader"]
pub struct R(crate::R<UR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IWDG2M` reader - Independent Watchdog 2 mode"]
pub struct IWDG2M_R(crate::FieldReader<bool, bool>);
impl IWDG2M_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDG2M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDG2M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURE` reader - Secure mode"]
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
impl R {
    #[doc = "Bit 0 - Independent Watchdog 2 mode"]
    #[inline(always)]
    pub fn iwdg2m(&self) -> IWDG2M_R {
        IWDG2M_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Secure mode"]
    #[inline(always)]
    pub fn secure(&self) -> SECURE_R {
        SECURE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "SYSCFG user register 12\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur12](index.html) module"]
pub struct UR12_SPEC;
impl crate::RegisterSpec for UR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur12::R](R) reader structure"]
impl crate::Readable for UR12_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UR12 to value 0"]
impl crate::Resettable for UR12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
