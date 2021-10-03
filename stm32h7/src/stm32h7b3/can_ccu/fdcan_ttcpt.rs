#[doc = "Register `FDCAN_TTCPT` reader"]
pub struct R(crate::R<FDCAN_TTCPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTCPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTCPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTCPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CT` reader - Cycle Count Value"]
pub struct CT_R(crate::FieldReader<u8, u8>);
impl CT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWV` reader - Stop Watch Value"]
pub struct SWV_R(crate::FieldReader<u16, u16>);
impl SWV_R {
    pub(crate) fn new(bits: u16) -> Self {
        SWV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Cycle Count Value"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - Stop Watch Value"]
    #[inline(always)]
    pub fn swv(&self) -> SWV_R {
        SWV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "FDCAN TT Capture Time Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttcpt](index.html) module"]
pub struct FDCAN_TTCPT_SPEC;
impl crate::RegisterSpec for FDCAN_TTCPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttcpt::R](R) reader structure"]
impl crate::Readable for FDCAN_TTCPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_TTCPT to value 0"]
impl crate::Resettable for FDCAN_TTCPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
