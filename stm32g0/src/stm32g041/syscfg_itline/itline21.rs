#[doc = "Register `ITLINE21` reader"]
pub struct R(crate::R<ITLINE21_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE21_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE21_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE21_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIM16` reader - TIM16"]
pub struct TIM16_R(crate::FieldReader<bool, bool>);
impl TIM16_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TIM16"]
    #[inline(always)]
    pub fn tim16(&self) -> TIM16_R {
        TIM16_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "interrupt line 21 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline21](index.html) module"]
pub struct ITLINE21_SPEC;
impl crate::RegisterSpec for ITLINE21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline21::R](R) reader structure"]
impl crate::Readable for ITLINE21_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE21 to value 0"]
impl crate::Resettable for ITLINE21_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
