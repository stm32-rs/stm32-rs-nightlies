#[doc = "Register `ETZPC_VERR` reader"]
pub struct R(crate::R<ETZPC_VERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETZPC_VERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETZPC_VERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETZPC_VERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MINREV` reader - MINREV"]
pub struct MINREV_R(crate::FieldReader<u8, u8>);
impl MINREV_R {
    pub(crate) fn new(bits: u8) -> Self {
        MINREV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINREV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAJREV` reader - MAJREV"]
pub struct MAJREV_R(crate::FieldReader<u8, u8>);
impl MAJREV_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAJREV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAJREV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - MINREV"]
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - MAJREV"]
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "ETZPC IP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_verr](index.html) module"]
pub struct ETZPC_VERR_SPEC;
impl crate::RegisterSpec for ETZPC_VERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etzpc_verr::R](R) reader structure"]
impl crate::Readable for ETZPC_VERR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETZPC_VERR to value 0x20"]
impl crate::Resettable for ETZPC_VERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
