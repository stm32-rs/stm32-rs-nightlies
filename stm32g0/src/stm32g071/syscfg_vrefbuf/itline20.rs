#[doc = "Register `ITLINE20` reader"]
pub struct R(crate::R<ITLINE20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIM15` reader - TIM15"]
pub struct TIM15_R(crate::FieldReader<bool, bool>);
impl TIM15_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TIM15"]
    #[inline(always)]
    pub fn tim15(&self) -> TIM15_R {
        TIM15_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "interrupt line 20 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline20](index.html) module"]
pub struct ITLINE20_SPEC;
impl crate::RegisterSpec for ITLINE20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline20::R](R) reader structure"]
impl crate::Readable for ITLINE20_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE20 to value 0"]
impl crate::Resettable for ITLINE20_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
