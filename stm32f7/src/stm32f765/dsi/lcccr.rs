#[doc = "Register `LCCCR` reader"]
pub struct R(crate::R<LCCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COLC` reader - Color Coding"]
pub struct COLC_R(crate::FieldReader<u8, u8>);
impl COLC_R {
    pub(crate) fn new(bits: u8) -> Self {
        COLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPE` reader - Loosely Packed Enable"]
pub struct LPE_R(crate::FieldReader<bool, bool>);
impl LPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Color Coding"]
    #[inline(always)]
    pub fn colc(&self) -> COLC_R {
        COLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Loosely Packed Enable"]
    #[inline(always)]
    pub fn lpe(&self) -> LPE_R {
        LPE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "DSI Host LTDC Current Color Coding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcccr](index.html) module"]
pub struct LCCCR_SPEC;
impl crate::RegisterSpec for LCCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcccr::R](R) reader structure"]
impl crate::Readable for LCCCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LCCCR to value 0"]
impl crate::Resettable for LCCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
