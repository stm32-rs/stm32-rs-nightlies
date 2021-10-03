#[doc = "Register `FDCAN_TURNA` reader"]
pub struct R(crate::R<FDCAN_TURNA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TURNA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TURNA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TURNA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NAV` reader - Numerator Actual Value"]
pub struct NAV_R(crate::FieldReader<u32, u32>);
impl NAV_R {
    pub(crate) fn new(bits: u32) -> Self {
        NAV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17 - Numerator Actual Value"]
    #[inline(always)]
    pub fn nav(&self) -> NAV_R {
        NAV_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
#[doc = "FDCAN TUR Numerator Actual Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_turna](index.html) module"]
pub struct FDCAN_TURNA_SPEC;
impl crate::RegisterSpec for FDCAN_TURNA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_turna::R](R) reader structure"]
impl crate::Readable for FDCAN_TURNA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_TURNA to value 0"]
impl crate::Resettable for FDCAN_TURNA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
