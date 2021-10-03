#[doc = "Register `CCVR` reader"]
pub struct R(crate::R<CCVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NCV` reader - NMOS compensation value"]
pub struct NCV_R(crate::FieldReader<u8, u8>);
impl NCV_R {
    pub(crate) fn new(bits: u8) -> Self {
        NCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCV` reader - PMOS compensation value"]
pub struct PCV_R(crate::FieldReader<u8, u8>);
impl PCV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - NMOS compensation value"]
    #[inline(always)]
    pub fn ncv(&self) -> NCV_R {
        NCV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PMOS compensation value"]
    #[inline(always)]
    pub fn pcv(&self) -> PCV_R {
        PCV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "SYSCFG compensation cell value register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccvr](index.html) module"]
pub struct CCVR_SPEC;
impl crate::RegisterSpec for CCVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccvr::R](R) reader structure"]
impl crate::Readable for CCVR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CCVR to value 0"]
impl crate::Resettable for CCVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
