#[doc = "Register `RX_UNICAST_PACKETS_GOOD` reader"]
pub struct R(crate::R<RX_UNICAST_PACKETS_GOOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_UNICAST_PACKETS_GOOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_UNICAST_PACKETS_GOOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_UNICAST_PACKETS_GOOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXUCASTG` reader - Rx Unicast Packets Good"]
pub struct RXUCASTG_R(crate::FieldReader<u32, u32>);
impl RXUCASTG_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXUCASTG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUCASTG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Rx Unicast Packets Good"]
    #[inline(always)]
    pub fn rxucastg(&self) -> RXUCASTG_R {
        RXUCASTG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Rx unicast packets good register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_unicast_packets_good](index.html) module"]
pub struct RX_UNICAST_PACKETS_GOOD_SPEC;
impl crate::RegisterSpec for RX_UNICAST_PACKETS_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_unicast_packets_good::R](R) reader structure"]
impl crate::Readable for RX_UNICAST_PACKETS_GOOD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_UNICAST_PACKETS_GOOD to value 0"]
impl crate::Resettable for RX_UNICAST_PACKETS_GOOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
