#[doc = "Register `ITLINE16` reader"]
pub struct R(crate::R<ITLINE16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIM3` reader - TIM3"]
pub struct TIM3_R(crate::FieldReader<bool, bool>);
impl TIM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TIM3"]
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "interrupt line 16 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline16](index.html) module"]
pub struct ITLINE16_SPEC;
impl crate::RegisterSpec for ITLINE16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline16::R](R) reader structure"]
impl crate::Readable for ITLINE16_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE16 to value 0"]
impl crate::Resettable for ITLINE16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
