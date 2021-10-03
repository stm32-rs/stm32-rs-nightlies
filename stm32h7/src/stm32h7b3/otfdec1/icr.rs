#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEIF` reader - SEIF"]
pub struct SEIF_R(crate::FieldReader<bool, bool>);
impl SEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XONEIF` reader - Execute-only execute-Never Error Interrupt Flag clear"]
pub struct XONEIF_R(crate::FieldReader<bool, bool>);
impl XONEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        XONEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XONEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEIF` reader - KEIF"]
pub struct KEIF_R(crate::FieldReader<bool, bool>);
impl KEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        KEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - SEIF"]
    #[inline(always)]
    pub fn seif(&self) -> SEIF_R {
        SEIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Execute-only execute-Never Error Interrupt Flag clear"]
    #[inline(always)]
    pub fn xoneif(&self) -> XONEIF_R {
        XONEIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - KEIF"]
    #[inline(always)]
    pub fn keif(&self) -> KEIF_R {
        KEIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "OTFDEC interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
