#[doc = "Register `ITLINE4` reader"]
pub struct R(crate::R<ITLINE4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RCC` reader - RCC"]
pub struct RCC_R(crate::FieldReader<bool, bool>);
impl RCC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RCC"]
    #[inline(always)]
    pub fn rcc(&self) -> RCC_R {
        RCC_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "interrupt line 4 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline4](index.html) module"]
pub struct ITLINE4_SPEC;
impl crate::RegisterSpec for ITLINE4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline4::R](R) reader structure"]
impl crate::Readable for ITLINE4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE4 to value 0"]
impl crate::Resettable for ITLINE4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
