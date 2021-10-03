#[doc = "Register `ITLINE15` reader"]
pub struct R(crate::R<ITLINE15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIM2` reader - TIM2"]
pub struct TIM2_R(crate::FieldReader<bool, bool>);
impl TIM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TIM2"]
    #[inline(always)]
    pub fn tim2(&self) -> TIM2_R {
        TIM2_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "interrupt line 15 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline15](index.html) module"]
pub struct ITLINE15_SPEC;
impl crate::RegisterSpec for ITLINE15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline15::R](R) reader structure"]
impl crate::Readable for ITLINE15_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE15 to value 0"]
impl crate::Resettable for ITLINE15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
