#[doc = "Register `UR11` reader"]
pub struct R(crate::R<UR11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IWDG1M` reader - Independent Watchdog 1 mode"]
pub struct IWDG1M_R(crate::FieldReader<bool, bool>);
impl IWDG1M_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDG1M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDG1M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 16 - Independent Watchdog 1 mode"]
    #[inline(always)]
    pub fn iwdg1m(&self) -> IWDG1M_R {
        IWDG1M_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "SYSCFG user register 11\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur11](index.html) module"]
pub struct UR11_SPEC;
impl crate::RegisterSpec for UR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur11::R](R) reader structure"]
impl crate::Readable for UR11_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UR11 to value 0"]
impl crate::Resettable for UR11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
