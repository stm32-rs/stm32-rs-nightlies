#[doc = "Register `ITLINE18` reader"]
pub struct R(crate::R<ITLINE18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LPTIM2` reader - LPTIM2"]
pub struct LPTIM2_R(crate::FieldReader<bool, bool>);
impl LPTIM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - LPTIM2"]
    #[inline(always)]
    pub fn lptim2(&self) -> LPTIM2_R {
        LPTIM2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "interrupt line 18 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline18](index.html) module"]
pub struct ITLINE18_SPEC;
impl crate::RegisterSpec for ITLINE18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline18::R](R) reader structure"]
impl crate::Readable for ITLINE18_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE18 to value 0"]
impl crate::Resettable for ITLINE18_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
