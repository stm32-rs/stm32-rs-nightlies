#[doc = "Register `FDCAN_TTCTC` reader"]
pub struct R(crate::R<FDCAN_TTCTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTCTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTCTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTCTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CT` reader - Cycle Time"]
pub struct CT_R(crate::FieldReader<u16, u16>);
impl CT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC` reader - Cycle Count"]
pub struct CC_R(crate::FieldReader<u8, u8>);
impl CC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Cycle Time"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - Cycle Count"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[doc = "FDCAN TT Cycle Time and Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttctc](index.html) module"]
pub struct FDCAN_TTCTC_SPEC;
impl crate::RegisterSpec for FDCAN_TTCTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttctc::R](R) reader structure"]
impl crate::Readable for FDCAN_TTCTC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_TTCTC to value 0"]
impl crate::Resettable for FDCAN_TTCTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
