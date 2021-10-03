#[doc = "Register `LTDC_ISR` reader"]
pub struct R(crate::R<LTDC_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LIF` reader - LIF"]
pub struct LIF_R(crate::FieldReader<bool, bool>);
impl LIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUIF` reader - FUIF"]
pub struct FUIF_R(crate::FieldReader<bool, bool>);
impl FUIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FUIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERRIF` reader - TERRIF"]
pub struct TERRIF_R(crate::FieldReader<bool, bool>);
impl TERRIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TERRIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERRIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRIF` reader - RRIF"]
pub struct RRIF_R(crate::FieldReader<bool, bool>);
impl RRIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - LIF"]
    #[inline(always)]
    pub fn lif(&self) -> LIF_R {
        LIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FUIF"]
    #[inline(always)]
    pub fn fuif(&self) -> FUIF_R {
        FUIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TERRIF"]
    #[inline(always)]
    pub fn terrif(&self) -> TERRIF_R {
        TERRIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RRIF"]
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "This register returns the interrupt status flag.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_isr](index.html) module"]
pub struct LTDC_ISR_SPEC;
impl crate::RegisterSpec for LTDC_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_isr::R](R) reader structure"]
impl crate::Readable for LTDC_ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTDC_ISR to value 0"]
impl crate::Resettable for LTDC_ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
