#[doc = "Register `ITLINE17` reader"]
pub struct R(crate::R<ITLINE17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIM6` reader - TIM6"]
pub struct TIM6_R(crate::FieldReader<bool, bool>);
impl TIM6_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC` reader - DAC"]
pub struct DAC_R(crate::FieldReader<bool, bool>);
impl DAC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM1` reader - LPTIM1"]
pub struct LPTIM1_R(crate::FieldReader<bool, bool>);
impl LPTIM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TIM6"]
    #[inline(always)]
    pub fn tim6(&self) -> TIM6_R {
        TIM6_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DAC"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LPTIM1"]
    #[inline(always)]
    pub fn lptim1(&self) -> LPTIM1_R {
        LPTIM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "interrupt line 17 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline17](index.html) module"]
pub struct ITLINE17_SPEC;
impl crate::RegisterSpec for ITLINE17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline17::R](R) reader structure"]
impl crate::Readable for ITLINE17_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE17 to value 0"]
impl crate::Resettable for ITLINE17_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
