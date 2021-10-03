#[doc = "Register `UR15` reader"]
pub struct R(crate::R<UR15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FZIWDGSTB` reader - Freeze independent watchdog in Standby mode"]
pub struct FZIWDGSTB_R(crate::FieldReader<bool, bool>);
impl FZIWDGSTB_R {
    pub(crate) fn new(bits: bool) -> Self {
        FZIWDGSTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FZIWDGSTB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 16 - Freeze independent watchdog in Standby mode"]
    #[inline(always)]
    pub fn fziwdgstb(&self) -> FZIWDGSTB_R {
        FZIWDGSTB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "SYSCFG user register 15\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur15](index.html) module"]
pub struct UR15_SPEC;
impl crate::RegisterSpec for UR15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur15::R](R) reader structure"]
impl crate::Readable for UR15_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UR15 to value 0"]
impl crate::Resettable for UR15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
