#[doc = "Register `RX_PAYSZ` reader"]
pub struct R(crate::R<RX_PAYSZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_PAYSZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_PAYSZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_PAYSZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXPAYSZ` reader - RXPAYSZ"]
pub struct RXPAYSZ_R(crate::FieldReader<u16, u16>);
impl RXPAYSZ_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXPAYSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPAYSZ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - RXPAYSZ"]
    #[inline(always)]
    pub fn rxpaysz(&self) -> RXPAYSZ_R {
        RXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "UCPD Rx Paysize Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_paysz](index.html) module"]
pub struct RX_PAYSZ_SPEC;
impl crate::RegisterSpec for RX_PAYSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_paysz::R](R) reader structure"]
impl crate::Readable for RX_PAYSZ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_PAYSZ to value 0"]
impl crate::Resettable for RX_PAYSZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
