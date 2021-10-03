#[doc = "Register `FDCAN_TXBCF` reader"]
pub struct R(crate::R<FDCAN_TXBCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CF` reader - Cancellation Finished"]
pub struct CF_R(crate::FieldReader<u8, u8>);
impl CF_R {
    pub(crate) fn new(bits: u8) -> Self {
        CF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "FDCAN Tx Buffer Cancellation Finished Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbcf](index.html) module"]
pub struct FDCAN_TXBCF_SPEC;
impl crate::RegisterSpec for FDCAN_TXBCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txbcf::R](R) reader structure"]
impl crate::Readable for FDCAN_TXBCF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_TXBCF to value 0"]
impl crate::Resettable for FDCAN_TXBCF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
