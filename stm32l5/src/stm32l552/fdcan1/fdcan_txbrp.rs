#[doc = "Register `FDCAN_TXBRP` reader"]
pub struct R(crate::R<FDCAN_TXBRP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBRP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBRP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBRP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TRP` reader - Transmission Request Pending"]
pub struct TRP_R(crate::FieldReader<u8, u8>);
impl TRP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmission Request Pending"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "FDCAN Tx Buffer Request Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbrp](index.html) module"]
pub struct FDCAN_TXBRP_SPEC;
impl crate::RegisterSpec for FDCAN_TXBRP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txbrp::R](R) reader structure"]
impl crate::Readable for FDCAN_TXBRP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_TXBRP to value 0"]
impl crate::Resettable for FDCAN_TXBRP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
