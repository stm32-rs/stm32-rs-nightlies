#[doc = "Register `ITLINE14` reader"]
pub struct R(crate::R<ITLINE14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIM1_CC` reader - TIM1_CC"]
pub struct TIM1_CC_R(crate::FieldReader<bool, bool>);
impl TIM1_CC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1_CC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1_CC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TIM1_CC"]
    #[inline(always)]
    pub fn tim1_cc(&self) -> TIM1_CC_R {
        TIM1_CC_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "interrupt line 14 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline14](index.html) module"]
pub struct ITLINE14_SPEC;
impl crate::RegisterSpec for ITLINE14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline14::R](R) reader structure"]
impl crate::Readable for ITLINE14_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE14 to value 0"]
impl crate::Resettable for ITLINE14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
